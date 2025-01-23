.PHONY: run db clean

run: db
	cargo run

db:
	export DATABASE_URL="postgres://postgres:password123@localhost/craw"
	docker run --name craw-db -e POSTGRES_PASSWORD=password123 -e POSTGRES_DB=craw -p 5432:5432 -d postgres:14
clean:
	docker stop craw-db
	docker rm craw-db