-- Your SQL goes here
ALTER TABLE `club`
ADD `moderation_state` INTEGER NOT NULL DEFAULT 0
AFTER `is_published`;