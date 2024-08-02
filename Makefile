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
	cargo add anyhow@1.0.86
	cargo add argon2@0.5.3
	cargo add axum@0.7.3 -F multipart
	cargo add axum-macros@0.4.1
	cargo add chrono@0.4.24 -F serde
	cargo add cloudinary@0.4.0
	cargo add dotenv@0.15.0
	cargo add jsonwebtoken@9.3.0
	cargo add lettre@0.10.0-rc.3
	cargo add lettre_email@0.9.4
	cargo add rand@0.8.5
	cargo add serde@1.0.159 -F derive
	cargo add serde_json@1.0.95
	cargo add sqlx@0.7.3 -F "runtime-async-std-native-tls postgres chrono uuid"
	cargo add tempfile@3.10.1
	cargo add tokio@1.27.0 -F full
	cargo add tower-http@0.5.0 -F cors
	cargo add uuid@1.3.0 -F "serde v4"
	# HotReload
	cargo install cargo-watch
	# SQLX-CLI
	cargo install sqlx-cli
