use dotenv::dotenv;

pub fn get_secret() -> String {
    dotenv().ok();

    let secret = std::env::var("SECRET").expect("SECRET not set");
    secret
}
