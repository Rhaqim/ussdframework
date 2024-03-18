run-example:
	cargo run --example basic_usage

build-frontend:
	cd frontend && npm install && npm run build

copy-frontend:
	cp -r frontend/.next/* static/

build-copy-frontend: build-frontend copy-frontend