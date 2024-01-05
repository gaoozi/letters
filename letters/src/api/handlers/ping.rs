use axum::http::StatusCode;

pub async fn ping() -> Result<String, StatusCode> {
    Ok("Hello World!".to_string())
}
