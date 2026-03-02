-- Create weekly_recaps table to cache AI-generated summaries
CREATE TABLE weekly_recaps (
    id INT AUTO_INCREMENT PRIMARY KEY,
    week_number INT NOT NULL,
    year INT NOT NULL,
    recap TEXT NOT NULL,
    generated_at DATETIME NOT NULL,
    UNIQUE KEY unique_week_year (week_number, year)
);
