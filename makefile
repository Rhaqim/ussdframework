.PHONY: \
	run run-standalone \
	run-all stop-all \
	frontend frontend-dev frontend-build frontend-start \
	migrate \
	docker-build docker-build-postgres docker-run docker-up docker-down \
	clean clean-db

# ── default ────────────────────────────────────────────────────────────────────

# Print available targets
help:
	@echo ""
	@echo "  Examples"
	@echo "    make run            run the menubuilder example (server + frontend)"
	@echo "    make run-standalone run the standalone example (no server, no DB)"
	@echo ""
	@echo "  Frontend"
	@echo "    make frontend-dev   start the Next.js dev server on port 3000"
	@echo "    make frontend-build build the Next.js production bundle"
	@echo "    make frontend-start serve the production bundle"
	@echo ""
	@echo "  Database"
	@echo "    make migrate        run pending Diesel migrations against menu.sqlite3"
	@echo ""
	@echo "  Housekeeping"
	@echo "    make stop-all             kill any stale example / Next.js processes"
	@echo "    make clean                remove frontend build artefacts"
	@echo "    make clean-db             remove local SQLite database files"
	@echo ""
	@echo "  Docker"
	@echo "    make docker-build         build Docker image (SQLite)"
	@echo "    make docker-build-postgres build Docker image (PostgreSQL)"
	@echo "    make docker-run           run the container locally on port 8080"
	@echo "    make docker-up            start the full Compose stack"
	@echo "    make docker-down          stop the Compose stack"
	@echo ""

# ── examples ───────────────────────────────────────────────────────────────────

# menubuilder mode: Actix server (port 8080) + Next.js dev server (port 3000).
# The Actix server proxies non-API requests to Next.js.
run: stop-all
	cd frontend && npm run dev &
	cargo run --manifest-path examples/Cargo.toml --bin basic_usage

# Standalone mode: no server, no database.
# The developer supplies the USSDMenu and calls UssdApp::run() directly.
run-standalone:
	cargo run --manifest-path examples/Cargo.toml --bin standalone

# ── frontend ───────────────────────────────────────────────────────────────────

frontend-dev:
	cd frontend && npm run dev

frontend-build:
	cd frontend && npm install && npm run build

frontend-start: frontend-build
	cd frontend && npm run start

# ── database ───────────────────────────────────────────────────────────────────

migrate:
	diesel migration run --database-url menu.sqlite3

# ── housekeeping ───────────────────────────────────────────────────────────────

stop-all:
	@-pkill -f "next dev"                    2>/dev/null; true
	@-pkill -f "debug/basic_usage"           2>/dev/null; true
	@sleep 1

clean:
	rm -rf frontend/.next

clean-db:
	rm -f menu.sqlite3 examples/menu.sqlite3

# ── Docker ─────────────────────────────────────────────────────────────────────

docker-build:
	docker build -t ussdframework:latest .

docker-build-postgres:
	docker build --build-arg DB_FEATURE=db-postgres -t ussdframework:postgres .

docker-run:
	docker run --rm -p 8080:8080 \
	  -v "$(PWD)/examples/data/menu.json:/app/menu.json:ro" \
	  -e USSD_JSON_SEED=/app/menu.json \
	  ussdframework:latest

docker-up:
	docker compose up --build

docker-down:
	docker compose down
