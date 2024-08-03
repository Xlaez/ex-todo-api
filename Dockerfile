# Stage 1: Build the application
FROM rust:1.80-alpine3.20 as builder

# Install necessary build tools and libraries
RUN apk add --no-cache musl-dev build-base

RUN mkdir -p /usr/src/ex

WORKDIR /usr/src/ex

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Stage 2: Create the runtime environment
FROM debian:buster-slim

WORKDIR /usr/src/ex

COPY --from=builder /usr/src/ex/target/release/todo-app /usr/src/ex/target/release/todo-app

# Install required tools and dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and cargo tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install sqlx-cli
RUN cargo install sqlx-cli

# Set environment variables
ENV DATABASE_URL=postgresql://user:password123@db:5432/todo_app?schema=public

# Database setup
RUN sqlx database create
RUN sqlx migrate run

# Expose the application port
EXPOSE 8083

CMD ["/usr/src/ex/target/release/todo-app"]
