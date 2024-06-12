use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OtpSchema {
    pub email: String,
    pub otp: String,
}