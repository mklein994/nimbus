extern crate dotenv;

pub fn setup() {
    dotenv::from_filename(".secret").ok();
}
