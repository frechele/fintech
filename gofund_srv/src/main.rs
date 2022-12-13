#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

mod config;
mod database;

#[get("/")]
async fn index() -> &'static str {
    "Hell World!"
}

#[get("/corr", format = "application/json")]
async fn corr() -> Json<Vec<database::Correlation>> {
    let mut db = database::connect_db().await;
    let corr = db.get_correlations().await;

    Json(corr)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, corr])
        .launch()
        .await?;

    Ok(())
}
