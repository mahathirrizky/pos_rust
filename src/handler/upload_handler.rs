use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Error};
use actix_web::error::ErrorInternalServerError;
use futures_util::stream::{StreamExt, TryStreamExt};
use std::io::Write;
use uuid::Uuid;
use image::io::Reader as ImageReader;
use webp::{Encoder, WebPMemory};

#[derive(serde::Serialize)]
pub struct UploadResponse {
    pub url: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteFilePayload {
    pub url: String,
}

pub async fn upload_product_image(payload: Multipart) -> Result<HttpResponse, Error> {
    upload_to_subfolder(payload, "products").await
}

pub async fn upload_employee_photo(payload: Multipart) -> Result<HttpResponse, Error> {
    upload_to_subfolder(payload, "employees").await
}

async fn upload_to_subfolder(mut payload: Multipart, subfolder: &str) -> Result<HttpResponse, Error> {
    let mut file_path = String::new();
    let upload_dir = format!("./uploads/{}", subfolder);

    std::fs::create_dir_all(&upload_dir)
        .map_err(|e| ErrorInternalServerError(format!("Failed to create upload directory: {}", e)))?;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let _content_disposition = field.content_disposition();

        let mut buffer = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            buffer.write_all(&data)?;
        }

        let img = ImageReader::new(std::io::Cursor::new(buffer))
            .with_guessed_format()? 
            .decode()
            .map_err(|e| ErrorInternalServerError(format!("Failed to decode image: {}", e)))?;

        let webp_filename = format!("{}.webp", Uuid::new_v4());
        let webp_filepath = format!("{}/{}", upload_dir, webp_filename);
        println!("Attempting to save WebP image to: {}", webp_filepath);

        // Encode the image with a quality of 80%
        let encoder = Encoder::from_image(&img)
            .map_err(|e| ErrorInternalServerError(format!("Failed to create WebP encoder: {}", e)))?;
        let webp_memory: WebPMemory = encoder.encode(80.0);

        // Write the WebP data to a file
        let mut webp_file = std::fs::File::create(&webp_filepath)
            .map_err(|e| ErrorInternalServerError(format!("Failed to create WebP file: {}", e)))?;
        webp_file.write_all(&webp_memory)
            .map_err(|e| ErrorInternalServerError(format!("Failed to write WebP data: {}", e)))?;
        println!("Image saved successfully to: {}", webp_filepath);

        file_path = format!("/uploads/{}/{}", subfolder, webp_filename);
        println!("Returning file_path: {}", file_path);
    }

    Ok(HttpResponse::Ok().json(UploadResponse { url: file_path }))
}

pub async fn delete_file(payload: web::Json<DeleteFilePayload>) -> Result<HttpResponse, Error> {
    let file_url = &payload.url;

    if !file_url.starts_with("/uploads/") || file_url.contains("..") {
        return Ok(HttpResponse::BadRequest().body("Invalid file path"));
    }

    let file_path = format!(".{}", file_url);

    match std::fs::remove_file(&file_path) {
        Ok(_) => {
            println!("Deleted file: {}", file_path);
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            eprintln!("Error deleting file {}: {}", file_path, e);
            Ok(HttpResponse::InternalServerError().body("Failed to delete file"))
        }
    }
}
