use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::models::documents::views::json_data_response::JsonDataResponse;

#[get("/documents/<domain>/<folder>")]
pub async fn get_documents(
    domain: &str,
    folder: &str
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {

    let response_value: String =
        format!("les documents que vous voulez recup sont dans le domain {} et dans le dossier : {}", domain, folder);

    Ok(Json(JsonDataResponse::new(response_value.as_str())))
}
