pub mod api {

    use crate::schema::weekly_recaps;
    use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
    use chrono::Datelike;
    use diesel::mysql::{Mysql, MysqlConnection};
    use diesel::prelude::*;
    use dotenv::dotenv;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Queryable, Selectable, QueryableByName, Serialize)]
    #[diesel(table_name = weekly_recaps)]
    #[diesel(check_for_backend(Mysql))]
    #[allow(dead_code)]
    pub struct WeeklyRecap {
        pub id: i32,
        pub week_number: i32,
        pub year: i32,
        pub recap: String,
        pub generated_at: chrono::NaiveDateTime,
    }
    #[derive(Deserialize)]
    pub struct CreateWeeklyRecap {
        pub week_number: i32,
        pub year: i32,
        pub recap: String,
    }

    pub fn connect_db() -> Result<MysqlConnection, Box<dyn std::error::Error>> {
        println!("Checking database...");
        dotenv().ok();

        let user = std::env::var("DB_USER")?;
        let password = std::env::var("DB_PASSWORD")?;
        let port = std::env::var("DB_PORT")?;
        let db_name = std::env::var("DB_NAME")?;
        let db_host =
            std::env::var("DB_HOST").unwrap_or_else(|_| "host.docker.internal".to_string());

        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            &user, &password, &db_host, &port, &db_name,
        );

        let conn = MysqlConnection::establish(&url)
            .map_err(|e| format!("Failed to connect to the database: {}", e))?;

        println!(
            "Successfully connected to database {} as {}",
            &db_host, &user
        );
        Ok(conn)
    }

    // Weekly Recap endpoints
    pub async fn get_weekly_recap(Path((year, week)): Path<(i32, i32)>) -> impl IntoResponse {
        let mut conn = connect_db().expect("Failed to connect to DB");

        let result = weekly_recaps::table
            .filter(weekly_recaps::week_number.eq(week))
            .filter(weekly_recaps::year.eq(year))
            .first::<WeeklyRecap>(&mut conn)
            .optional()
            .expect("Error loading recap");

        match result {
            Some(recap) => (StatusCode::OK, Json(Some(recap))),
            None => (StatusCode::NOT_FOUND, Json(None)),
        }
    }

    pub async fn get_latest_recap() -> impl IntoResponse {
        let mut conn = connect_db().expect("Failed to connect to DB");

        let result = weekly_recaps::table
            .order(weekly_recaps::id.desc())
            .first::<WeeklyRecap>(&mut conn)
            .optional()
            .expect("Error loading latest recap");

        match result {
            Some(recap) => (StatusCode::OK, Json(Some(recap))),
            None => (StatusCode::NOT_FOUND, Json(None)),
        }
    }

    pub async fn save_weekly_recap(Json(payload): Json<CreateWeeklyRecap>) -> impl IntoResponse {
        let mut conn = connect_db().expect("Failed to connect to DB");

        // Check if recap already exists for this week/year
        let existing = weekly_recaps::table
            .filter(weekly_recaps::week_number.eq(payload.week_number))
            .filter(weekly_recaps::year.eq(payload.year))
            .first::<WeeklyRecap>(&mut conn)
            .optional()
            .expect("Error checking for existing recap");

        if existing.is_some() {
            // Update existing recap
            diesel::update(
                weekly_recaps::table
                    .filter(weekly_recaps::week_number.eq(payload.week_number))
                    .filter(weekly_recaps::year.eq(payload.year)),
            )
            .set((
                weekly_recaps::recap.eq(&payload.recap),
                weekly_recaps::generated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
            .expect("Error updating recap");
            return (StatusCode::OK, "Recap updated successfully".to_string());
        }

        // Insert new recap
        diesel::insert_into(weekly_recaps::table)
            .values((
                weekly_recaps::week_number.eq(payload.week_number),
                weekly_recaps::year.eq(payload.year),
                weekly_recaps::recap.eq(payload.recap),
                weekly_recaps::generated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
            .expect("Error inserting recap");
        (StatusCode::CREATED, "Recap saved successfully".to_string())
    }

    #[derive(Deserialize)]
    pub struct GenerateRecapRequest {
        pub azure_token: Option<String>,
    }

    #[derive(Deserialize)]
    struct AzureResponse {
        output: Vec<AzureOutput>,
    }

    #[derive(Deserialize)]
    struct AzureOutput {
        #[serde(rename = "type")]
        output_type: String,
        content: Option<Vec<AzureContent>>,
    }

    #[derive(Deserialize)]
    struct AzureContent {
        #[serde(rename = "type")]
        content_type: String,
        text: Option<String>,
    }

    pub async fn generate_weekly_recap(
        Json(payload): Json<GenerateRecapRequest>,
    ) -> impl IntoResponse {
        dotenv().ok();

        let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT").unwrap_or_else(|_| "".to_string());
        let api_key = std::env::var("AZURE_OPENAI_API_KEY").unwrap_or_else(|_| "".to_string());
        let deployment_name = std::env::var("AZURE_OPENAI_DEPLOYMENT_NAME")
            .unwrap_or_else(|_| "scheduler".to_string());

        if endpoint.is_empty() {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing AZURE_OPENAI_ENDPOINT".to_string(),
            );
        }

        // Calculate current week number
        let now = chrono::Utc::now();
        let week_number = now.iso_week().week() as i32;
        let year = now.iso_week().year();

        let date_str = now.format("%A %d. %B %Y").to_string();
        let input = format!(
            "Dagens dato er {}. Uke {}, {}.",
            date_str, week_number, year
        );

        let url = format!(
            "{}/applications/{}/protocols/openai/responses?api-version=2025-11-15-preview",
            endpoint, deployment_name
        );

        let client = reqwest::Client::new();
        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({ "input": input }));

        if !api_key.is_empty() {
            request = request.header("api-key", api_key);
        } else if let Some(azure_token) = payload.azure_token {
            request = request.header("Authorization", format!("Bearer {}", azure_token));
        } else {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing Azure authentication. Set AZURE_OPENAI_API_KEY or provide an Azure bearer token.".to_string(),
            );
        }

        let response = request.send().await;

        let response = match response {
            Ok(r) => r,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Request failed: {}", e),
                );
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return (
                StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                format!("Azure API error: {}", body),
            );
        }

        let data: AzureResponse = match response.json().await {
            Ok(d) => d,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to parse response: {}", e),
                );
            }
        };

        // Extract recap text from response
        let recap = data
            .output
            .iter()
            .find(|o| o.output_type == "message")
            .and_then(|o| o.content.as_ref())
            .and_then(|c| c.iter().find(|c| c.content_type == "output_text"))
            .and_then(|c| c.text.clone())
            .unwrap_or_else(|| "No recap generated".to_string());

        // Save to database
        let mut conn = match connect_db() {
            Ok(c) => c,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("DB error: {}", e),
                );
            }
        };

        // Check if recap exists for this week
        let existing = weekly_recaps::table
            .filter(weekly_recaps::week_number.eq(week_number))
            .filter(weekly_recaps::year.eq(year))
            .first::<WeeklyRecap>(&mut conn)
            .optional()
            .ok()
            .flatten();

        if existing.is_some() {
            diesel::update(
                weekly_recaps::table
                    .filter(weekly_recaps::week_number.eq(week_number))
                    .filter(weekly_recaps::year.eq(year)),
            )
            .set((
                weekly_recaps::recap.eq(&recap),
                weekly_recaps::generated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
            .ok();
        } else {
            diesel::insert_into(weekly_recaps::table)
                .values((
                    weekly_recaps::week_number.eq(week_number),
                    weekly_recaps::year.eq(year),
                    weekly_recaps::recap.eq(&recap),
                    weekly_recaps::generated_at.eq(chrono::Utc::now().naive_utc()),
                ))
                .execute(&mut conn)
                .ok();
        }

        (
            StatusCode::OK,
            format!("Recap generated for week {} of {}", week_number, year),
        )
    }
}
