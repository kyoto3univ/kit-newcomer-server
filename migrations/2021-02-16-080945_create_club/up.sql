-- Your SQL goes here
CREATE TABLE `club` (
    `id` VARCHAR(36) NOT NULL,
    `name` VARCHAR(256) NOT NULL,
    `is_published` BOOLEAN NOT NULL DEFAULT FALSE,
    `short_description` TEXT,
    `long_description` MEDIUMTEXT,
    `join_description` TEXT,
    `place` TEXT,
    `schedule` TEXT,
    `video_url` TEXT,
    `contact_url` TEXT,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB CHARSET = utf8mb4;