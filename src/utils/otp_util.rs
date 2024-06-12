use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// Function to generate a numeric OTP
pub fn generate_otp(length: usize) -> String {
    let mut rng = thread_rng();
    let otp: String = (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();
    otp
}
