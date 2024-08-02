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

- health checker (GET) -------- */api/health_checker*
- register endpoint (POST) -------- */api/user/register*
- login endpoint (POST) -------- */api/user/login*
- verify email endpoint (POST) --------- */api/user/verify_email*
- upload/update profile image (PATCH) --------- */api/user/update/img*
- change password (PATCH) --------- */api/user/update/password*
- get user (GET) --------- */api/user/:username*
- add list item (POST) ----------- */api/lists/list*
- get user's todo lists (GET) ----------- */api/lists/:id*
- update todo list (PATCH) --------------- */api/lists/list*
- delete todo list (DELETE) -------------- */api/lists/list:id*

Note: **I'm done, it's a simple API for frontend devs to use for practice. If you are following, I'll soon deploy and provide postman documentation.**

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
