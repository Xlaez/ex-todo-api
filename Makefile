dev:
	docker-compose up -d
	
dev-down:
	docker-compose down

migrate-add:
	sqlx migrate add -r init

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

db-up:
	sqlx database create

db-down:
	sqlx database drop

start-server:
	cargo watch -q -c -w src/ -x run

install:
	cargo add axum
	cargo add tokio -F full
	cargo add tower-http -F "cors"
	cargo add serde_json
	cargo add serde -F derive
	cargo add chrono -F serde
	cargo add dotenv
	cargo add uuid -F "serde v4"
	cargo add sqlx -F "runtime-async-std-native-tls postgres chrono uuid"
	# HotReload
	cargo install cargo-watch
	# SQLX-CLI
	cargo install sqlx-cli