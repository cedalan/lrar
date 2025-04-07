.PHONY: help populate empty

# Variables
CONTAINER_NAME := lrar_database
SQL_SCRIPT := backend/populate_db.sql
REMOTE_SQL_SCRIPT := /populate_db.sql
DB_USER := user 
DB_NAME := db 

# Display available commands
help:
	@echo "Available commands:"
	@echo "  populate    Populate the PostgreSQL database with tenants and burn data."
	@echo "  empty       Remove all records from the tenants and burn tables."

# Populate the database with tenants and burns
populate:
	@echo "Checking if container $(CONTAINER_NAME) is running..."
	@if ! docker ps --format '{{.Names}}' | grep -w $(CONTAINER_NAME) > /dev/null; then \
		echo "Error: Container $(CONTAINER_NAME) is not running."; \
		exit 1; \
	fi
	@echo "Populating the PostgreSQL database with tenants and burn data..."
	docker cp $(SQL_SCRIPT) $(CONTAINER_NAME):$(REMOTE_SQL_SCRIPT)
	docker exec -i $(CONTAINER_NAME) psql -U $(DB_USER) -d $(DB_NAME) -f $(REMOTE_SQL_SCRIPT) || \
	(echo "Error: Failed to execute SQL script." && exit 1)
	docker exec -i $(CONTAINER_NAME) rm $(REMOTE_SQL_SCRIPT)
	@echo "Database population complete."

# Remove all records from the tenants and burn tables
empty:
	@echo "Checking if container $(CONTAINER_NAME) is running..."
	@if ! docker ps --format '{{.Names}}' | grep -w $(CONTAINER_NAME) > /dev/null; then \
		echo "Error: Container $(CONTAINER_NAME) is not running."; \
		exit 1; \
	fi
	@echo "Removing all records from the tenants and burn tables..."
	docker exec -i $(CONTAINER_NAME) psql -U $(DB_USER) -d $(DB_NAME) -c "DELETE FROM burn; DELETE FROM tenants;" || \
	(echo "Error: Failed to clear tables." && exit 1)
	@echo "All tenant and burn records removed."
