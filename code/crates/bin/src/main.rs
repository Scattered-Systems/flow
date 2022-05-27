mod api;

#[tokio::main]
async fn main() {
    let app = crate::api::interface::Interface::new().await;
    println!("{:#?}", app.context.settings)
}