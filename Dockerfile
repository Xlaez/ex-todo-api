FROM 1.80-alpine3.20

RUN mkdir -p /usr/src/ex

WORKDIR /usr/src/ex

COPY Cargo.toml Cargo.lock ./

COPY src ./

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/ex

COPY . /usr/src/ex/target/release/todo-app

# ENVIRONMENTAL VARIABLES

EXPOSE 8083


RUN sqlx database create
RUN sqlx migrate run


CMD [ "cargo", "watch" ,"-q" ,"-c", "-w", "src/" ,"-x" ,"run" ]