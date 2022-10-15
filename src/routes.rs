use redis::AsyncCommands;

use actix_web::{get, http::header, web, HttpResponse, Result};

pub use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo, RedisError};

use crate::config::AppConfig;
use crate::error::WebError;
use crate::utils::{decide_decompress, decide_validate};

#[get("/GET/{key}")]
pub async fn get_from_key(
    client: web::Data<redis::Client>,
    config: web::Data<AppConfig>,
    key: web::Path<String>,
) -> Result<HttpResponse> {
    let key_name = key.into_inner();

    let mut con = client
        .get_tokio_connection_manager()
        .await
        .map_err(|e| WebError::RedisError(e).log())?;

    let result: Option<Vec<u8>> = con
        .get(&key_name.as_bytes())
        .await
        .map_err(|e| WebError::RedisError(e).log())?;

    match result {
        Some(value) if value.is_empty() => Ok(HttpResponse::NoContent().finish()),
        Some(value) => {
            let decompressed = decide_decompress(config.decompress_gzip, value)
                .map_err(|_| WebError::GzipError(key_name.clone()).log())?;

            let validated = decide_validate(config.validate_json, decompressed)
                .map_err(|e| WebError::InvalidJsonError(e.to_string()).log())?;

            let mut partial_response = HttpResponse::Ok();

            if config.output_json {
                partial_response.insert_header(header::ContentType(mime::APPLICATION_JSON));
            }

            Ok(partial_response.body(validated))
        }
        None => Ok(HttpResponse::NoContent().finish()),
    }
}
