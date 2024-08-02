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

ENV POSTGRES_HOST=127.0.0.1
ENV POSTGRES_PORT=5432
ENV POSTGRES_USER=user
ENV POSTGRES_PASSWORD=password123
ENV POSTGRES_DB=todo_app

ENV DATABASE_URL=postgresql://user:password123@localhost:5432/todo_app?schema=public

ENV PGADMIN_DEFAULT_EMAIL=pgadmin4@pgadmin.org
ENV PGADMIN_DEFAULT_PASSWORD=password123

ENV JWT_SECRET=thisisaverystrongsecretindeed
ENV SMTP_USER=kamounation@gmail.com
ENV SMTP_PASSWORD=zjxorggpslpmwkzi
ENV SMTP_SERVICE=smtp.gmail.com

ENV CLOUDINARY_CLOUD_NAME=dwgh9pbcr
ENV CLOUDINARY_API_SECRET=Iji-x2MzbuOoPZa1FI4HNL42nX4
ENV CLOUDINARY_API_KEY=934283981171746


EXPOSE 8083


RUN sqlx database create
RUN sqlx migrate run


CMD [ "cargo", "watch" ,"-q" ,"-c", "-w", "src/" ,"-x" ,"run" ]