PHONY: run-example run-migration migrate build-frontend copy-frontend clean-frontend start-frontend run-all

run-example:
	cargo run --example basic_usage

run-migration:
	diesel migration generate ussd_services

migrate:
	diesel migration run --database-url menu.sqlite3

build-frontend:
	cd frontend && npm install && npm run build

serve-frontend:
	cd frontend && npm run start

frontend: build-frontend serve-frontend

copy-frontend:
	cp -r frontend/.next/* src/builder/static/

clean-frontend:
	rm -rf src/builder/static/*

start-frontend:
	cd frontend && npm run dev

# Run the MenuBuilder server (port 8080) and Next.js dev server (port 3000) together.
# The MenuBuilder server proxies all non-API requests to Next.js.
run-all: stop-all
	cd frontend && npm run dev &
	cargo run --features menubuilder --example basic_usage

# Kill any stale Next.js dev server or example processes from a previous run.
stop-all:
	@-pkill -f "next dev" 2>/dev/null; true
	@-pkill -f "target/debug/examples/basic_usage" 2>/dev/null; true
	@sleep 1