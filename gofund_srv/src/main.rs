#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
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
