/*
    Appellation: api <binary> (rs-sandbox)
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("Welcome to Flow");

    let instance = u2f::protocol::U2f::new("flow".to_string());
    let challange = instance.generate_challenge().ok().unwrap();
    Ok(())
}