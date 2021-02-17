-- Your SQL goes here
CREATE TABLE `asset` (
    `id` BIGINT NOT NULL AUTO_INCREMENT,
    `owner_id` BIGINT NOT NULL,
    `club_id` BIGINT NOT NULL,
    `name` TEXT NOT NULL,
    `alternative_description` TEXT,
    `file_path` TEXT NOT NULL,
    `file_size` INTEGER NOT NULL,
    `image_width` INTEGER,
    `image_height` INTEGER,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB CHARSET = utf8mb4;
-- Add foreign keys
ALTER TABLE `asset`
ADD CONSTRAINT `fk_asset_club_id` FOREIGN KEY (`club_id`) REFERENCES `club`(`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;