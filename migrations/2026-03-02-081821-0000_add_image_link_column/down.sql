-- This file should undo anything in `up.sql`
UPDATE games
SET image_link = NULL
WHERE LENGTH(image_link) > 1024;   
