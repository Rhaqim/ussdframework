PHONY: run-example run-migration migrate build-frontend copy-frontend clean-frontend start-frontend

run-example:
	cargo run --example basic_usage

run-migration:
	diesel migration generate ussd_services

migrate:
	diesel migration run --database-url=${DATABASE_URL}

build-frontend:
	cd frontend && npm install && npm run build

copy-frontend:
	cp -r frontend/.next/* src/builder/static/

clean-frontend:
	rm -rf src/builder/static/*

start-frontend:
	cd frontend && npm run dev