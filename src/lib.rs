use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;
use sqlx::PgPool;
use shuttle_secrets::SecretStore;

mod router;
use router::hello_world;

#[shuttle_service::main]
async fn axum(
    #[shuttle_shared_db::Postgres] postgres: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore
) -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}