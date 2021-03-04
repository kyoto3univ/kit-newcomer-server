-- Your SQL goes here
ALTER TABLE `user_club_relation`
ADD `is_visible` BOOLEAN NOT NULL DEFAULT 0
AFTER `level`;