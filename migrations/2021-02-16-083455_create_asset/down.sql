-- This file should undo anything in `up.sql`
ALTER TABLE `asset` DROP FOREIGN KEY `fk_asset_club_id`;
DROP TABLE `asset`;