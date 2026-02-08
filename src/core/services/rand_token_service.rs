// This should be used to create random token for actions as:
// - account verification
// - reset password
// - etc.
use rand::Rng;
// pub trait TokenService: Send + Sync {
//     fn generate_token(&self) -> String;
// }

pub struct TokenService;

impl TokenService {
    pub fn generate_token() -> String {
        let mut rng = rand::thread_rng();
        let token: u32 = rng.gen_range(0..1000000); // Generate a number between 0 and 999999
        format!("{:06}", token) // Format as a 6-digit string, padding with leading zeros
    }
}
