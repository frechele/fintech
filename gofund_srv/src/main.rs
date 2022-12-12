#[macro_use] extern crate rocket;

mod analyzer;

use std::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Hell World!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let analyzer = analyzer::setup_analyzer();
    let thread_handle = analyzer.watch_thread(Duration::from_secs(1));

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await?;

    thread_handle.stop();

    Ok(())
}
