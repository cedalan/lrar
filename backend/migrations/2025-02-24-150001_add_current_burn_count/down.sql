-- This file should undo anything in `up.sql`
ALTER TABLE tenants
DROP COLUMN current_burn_count;