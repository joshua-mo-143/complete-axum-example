use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Dog {
    id: i32,
    name: String,
    colour: String,
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn get_all_records(db_conn: PgPool) -> Result<Json<Vec<Dog>>, sqlx::Error> {
    let records: Vec<Dog> = Vec::new();

    let query = sqlx::query("SELECT * FROM dogs").fetch_all(&db_conn).await?;

    Ok(Json(records))
}

pub async fn create_record(name: String, colour: String, db_conn: PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO dogs (name, colour) values ($1, $2")
            .bind(name)
            .bind(colour)
            .execute(&db_conn)
            .await?;
    
    Ok(())

}

pub async fn delete_record(id: i32, db_conn: PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM dogs WHERE id = $1")
        .bind(id)
        .execute(&db_conn)
        .await?;
    
    Ok(())
}