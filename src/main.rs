// #[macro_use] extern crate rocket;

use rocket::{Build, launch, Rocket};
use crate::api::app::launcher::launcher;

mod api;
mod models;
mod core;

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    launcher()
        .await
        .unwrap()
}

