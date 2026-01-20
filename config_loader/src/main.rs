pub mod config;
pub mod error;
pub mod loader;
pub mod test;
pub mod validator;

fn main() {
    dotenv::dotenv().ok();
    let config = loader::ConfigLoader::load().unwrap();
    println!("{:#?}", config);
}
