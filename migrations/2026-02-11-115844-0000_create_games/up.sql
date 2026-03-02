-- Your SQL goes here
CREATE TABLE games (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    genre VARCHAR(255) NOT NULL,
    image_link VARCHAR(1024) NOT NULL,
    utgivelsesdato DATE
);
