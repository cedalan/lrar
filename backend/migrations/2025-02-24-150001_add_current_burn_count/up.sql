-- Your SQL goes here
ALTER TABLE tenants
ADD COLUMN current_burn_count INTEGER DEFAULT 0;