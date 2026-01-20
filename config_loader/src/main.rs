
pub mod config;
pub mod error;
pub mod loader;
pub mod validator;
pub mod test;

fn main() {
   dotenv::dotenv().ok();
   let config = loader::ConfigLoader::load().unwrap();
   println!("{:#?}", config);
}