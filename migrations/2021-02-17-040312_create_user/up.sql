-- Your SQL goes here
CREATE TABLE `user` (
    `id` BIGINT NOT NULL,
    `name` TEXT NOT NULL,
    `screen_name` TEXT NOT NULL,
    `icon` VARCHAR(128),
    `permission` INTEGER NOT NULL DEFAULT 0,
    `access_token` VARCHAR(64),
    `access_token_secret` VARCHAR(64),
    PRIMARY KEY (`id`)
) ENGINE = InnoDB CHARSET = utf8mb4;
CREATE TABLE `user_club_relation`(
    `user_id` BIGINT NOT NULL,
    `club_id` BIGINT NOT NULL,
    `level` INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (`user_id`, `club_id`)
);
-- Add foreign keys
ALTER TABLE `asset`
ADD CONSTRAINT `fk_asset_owner_id` FOREIGN KEY (`owner_id`) REFERENCES `user`(`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;
ALTER TABLE `user_club_relation`
ADD CONSTRAINT `fk_user_club_relation_user_id` FOREIGN KEY (`user_id`) REFERENCES `user`(`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;
ALTER TABLE `user_club_relation`
ADD CONSTRAINT `fk_user_club_relation_club_id` FOREIGN KEY (`club_id`) REFERENCES `club`(`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;