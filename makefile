run-example:
	cargo run --example basic_usage

build-frontend:
	cd frontend && npm install && npm run build

copy-frontend:
	cp -r frontend/.next/* _next/

start-frontend:
	cd frontend && npm run dev