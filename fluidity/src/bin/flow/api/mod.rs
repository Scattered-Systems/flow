/*
   Appellation: api <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::interface::Api;

pub(crate) mod interface;
pub mod routes;

pub async fn spawn_api_with_ctx(ctx: fluidity::Context) -> scsys::prelude::BoxResult {
    let api = Api::new(ctx);
    api.run().await?;
    Ok(())
}
