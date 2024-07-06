install_sqlx:
	cargo install sqlx-cli --no-default-features --features rustls,postgres

set_env:
	DATABASE_URL=postgres://postgres:password@localhost:5432/passwords
	export DATABASE_URL

init_db:
	docker run \
		-e POSTGRES_USER=postgres \
		-e POSTGRES_PASSWORD=password \
		-e POSTGRES_DB=passwords \
		-p 5432:5432 \
		-d postgres \
		postgres -N 1000

create_db:
	make set_env
	sqlx database create
	
run_migration:
	sqlx migrate run

run:
	cargo watch -x "run --release"