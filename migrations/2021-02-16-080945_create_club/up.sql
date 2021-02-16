-- Your SQL goes here
CREATE TABLE `club` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(128) NOT NULL,
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