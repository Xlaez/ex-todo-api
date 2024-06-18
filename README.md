## TODO API

A Simple Todo API built with Rust and Axum

### Features

- Register
- Login
- Upload Avatar
- Update Email
- Update Password
- Forget and Recover Password
- Delete Account
- Add Todo List
- Update Todo List
- Delete Todo List
- Fetch All & Single Todo List(s)

### Current Endpoints

- */api/health_checker* health checker (GET)
- */api/user/register* register endpoint (POST)
- */api/user/login* login endpoint (POST)
- */api/user/verify_email* verify email endpoint (POST)
- */api/user/update/img* upload/update profile image (PATCH)
- */api/user/update/password* change password (PATCH)

### Usage

1. run `make install` to install dependencies

2. run `make dev` to start the docker container based on the config in the **docker-compose.yml** file

3. run `make migrate-up` to run migrations

4. run `make start-server` to start the server in watch mode


### Requirements

- Rust
- Cargo
- Docker

*Tip*: might build a desktop app for it using rust too !! 😀😀
*Tip*: I said **might** cause I am yet to complete the dolph-sso elixir & phoenix project I was working on... 👀
