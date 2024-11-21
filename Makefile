.PHONY: help populate empty

# Variables
CONTAINER_NAME := lrar_database
SQL_SCRIPT := backend/insert_tenants.sql
REMOTE_SQL_SCRIPT := /insert_tenants.sql
DB_USER := my_user
DB_NAME := my_db

# Display available commands
help:
	@echo "Available commands:"
	@echo "  populate    Populate the PostgreSQL database with tenants data."
	@echo "  empty       Remove all records from the tenants table."

# Populate the database with tenants
populate:
	@echo "Checking if container $(CONTAINER_NAME) is running..."
	@if ! docker ps --format '{{.Names}}' | grep -w $(CONTAINER_NAME) > /dev/null; then \
		echo "Error: Container $(CONTAINER_NAME) is not running."; \
		exit 1; \
	fi
	@echo "Populating the PostgreSQL database with tenants data..."
	docker cp $(SQL_SCRIPT) $(CONTAINER_NAME):$(REMOTE_SQL_SCRIPT)
	docker exec -i $(CONTAINER_NAME) psql -U $(DB_USER) -d $(DB_NAME) -f $(REMOTE_SQL_SCRIPT)
	@echo "Database population complete."

# Remove all records from the tenants table
empty:
	@echo "Checking if container $(CONTAINER_NAME) is running..."
	@if ! docker ps --format '{{.Names}}' | grep -w $(CONTAINER_NAME) > /dev/null; then \
		echo "Error: Container $(CONTAINER_NAME) is not running."; \
		exit 1; \
	fi
	@echo "Removing all records from the tenants table..."
	docker exec -i $(CONTAINER_NAME) psql -U $(DB_USER) -d $(DB_NAME) -c "DELETE FROM tenants;"
	@echo "All tenant records removed."
