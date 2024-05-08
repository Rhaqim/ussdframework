run-example:
	cargo run --example basic_usage

build-frontend:
	cd frontend && npm install && npm run build

copy-frontend:
	cp -r frontend/.next/* src/builder/static/

clean-frontend:
	rm -rf src/builder/static/*

start-frontend:
	cd frontend && npm run dev