mod health_checker;
mod user_and_auth;

pub use health_checker::health_checker_handler;
pub use  user_and_auth::{create_user_handler,login_handler, get_user_by_email, verify_email, upload_img, update_password};