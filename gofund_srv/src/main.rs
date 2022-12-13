#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

use std::collections::HashMap;

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

#[get("/<value_type>/<code>/<enddate>/<term>", format = "application/json")]
async fn get_value(value_type: String, code: String, enddate: String, term: i32) -> Json<HashMap<String, f64>> {
    let mut db = database::connect_db().await;
    let values= db.get_value(value_type.as_str(), code.as_str(), enddate.as_str(), term).await;

    Json(values)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, corr])
        .mount("/value", routes![get_value])
        .launch()
        .await?;

    Ok(())
}
