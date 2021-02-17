-- This file should undo anything in `up.sql`
ALTER TABLE `asset` DROP FOREIGN KEY `fk_asset_owner_id`;
ALTER TABLE `user_club_relation` DROP FOREIGN KEY `fk_user_club_relation_user_id`;
ALTER TABLE `user_club_relation` DROP FOREIGN KEY `fk_user_club_relation_club_id`;
DROP TABLE `user_club_relation`;
DROP TABLE `user`;