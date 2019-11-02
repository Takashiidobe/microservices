-- Your SQL goes here
CREATE TABLE users (
    `id` INT(11) PRIMARY KEY AUTO_INCREMENT,
    `username` VARCHAR(60) NOT NULL,
    `password` VARCHAR(255) NOT NULL
);