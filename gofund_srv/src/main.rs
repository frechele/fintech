#[macro_use] extern crate rocket;

mod database;
mod config;

#[get("/")]
async fn index() -> &'static str {
    let mut db = database::connect_db().await;

    db.execute().await;

    "Hell World!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
