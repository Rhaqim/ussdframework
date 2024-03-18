use actix_web::Result;

pub async fn get_data() -> Result<String> {
    Ok("Some data".to_string())
}
