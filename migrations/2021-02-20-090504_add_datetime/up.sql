-- Your SQL goes here
ALTER TABLE `club`
ADD `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
AFTER `is_published`;
ALTER TABLE `club`
ADD `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
AFTER `created_at`;