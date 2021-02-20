-- Your SQL goes here
ALTER TABLE `club`
ADD `top_image_id` BIGINT
AFTER `join_description`;
ALTER TABLE `club`
ADD `top_content_type` INTEGER NOT NULL DEFAULT 0
AFTER `top_image_id`;
ALTER TABLE `club`
ADD `thumb_image_id` BIGINT
AFTER `top_content_type`;
-- Add foreign keys
ALTER TABLE `club`
ADD CONSTRAINT `fk_club_top_image_id` FOREIGN KEY (`top_image_id`) REFERENCES `asset`(`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;
ALTER TABLE `club`
ADD CONSTRAINT `fk_club_thumb_image_id` FOREIGN KEY (`thumb_image_id`) REFERENCES `asset`(`id`) ON DELETE RESTRICT ON UPDATE RESTRICT;