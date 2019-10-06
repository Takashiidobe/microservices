-- Your SQL goes here
CREATE TABLE api_keys (
    `id` INT(11) PRIMARY KEY AUTO_INCREMENT,
    `public_id` VARCHAR(20) NOT NULL UNIQUE,
    `permission` VARCHAR(20) NOT NULL
)