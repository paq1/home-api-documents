use rocket::{Build, Rocket, routes};
use crate::api::app::cors::CORS;

use crate::api::documents::routes::hello_world::hello;
use crate::api::documents::routes::read_documents_router::get_documents;

pub async fn launcher () -> Result<Rocket<Build>, String> {
    let build = rocket::build()
        // .manage(taks_mongo_repository)
        .attach(CORS)
        .mount(
            "/",
            routes![hello, get_documents]
        );

    Ok(build)
}