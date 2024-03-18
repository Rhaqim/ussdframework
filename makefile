run-example:
	cargo run --example basic_usage

build-frontend:
	cd src/frontend && npm install && npm run build

copy-frontend:
	cp -r src/frontend/.next/* src/static/

build-copy-frontend: build-frontend copy-frontend