-- Your SQL goes here
-- dateカラムはstringで大丈夫なはず
ALTER TABLE num_infected
ALTER COLUMN date TYPE VARCHAR(20);
