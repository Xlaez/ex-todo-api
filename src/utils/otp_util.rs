use chrono::{DateTime, Duration, Utc};
use std::str::FromStr;

use rand::{thread_rng, Rng};

// Function to generate a numeric OTP
pub fn generate_otp(length: usize) -> String {
    let mut rng = thread_rng();
    let otp: String = (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();
    otp
}

pub fn check_otp_expiry(created_at: &str) -> Result<(), &'static str>{
    let created_at = DateTime::from_str(created_at).map_err(|_| "Invalid datetime format")?;

    let current_time = Utc::now();
    
    let duration = current_time - created_at;

     if duration > Duration::minutes(4) {
        Err("OTP expired")
    } else {
        Ok(())
    }
}
