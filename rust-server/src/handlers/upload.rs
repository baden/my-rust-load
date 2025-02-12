use actix_web::{web, HttpResponse, Responder};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub async fn upload_file(
    path: web::Path<(String, String)>,
    payload: web::Payload,
) -> impl Responder {
    let (file_name, file_path) = path.into_inner();
    let full_path = Path::new(&file_path).join(&file_name);

    let mut file = match File::create(&full_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut bytes = Vec::new();
    let mut stream = payload.map_err(|_| HttpResponse::BadRequest())?;
    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|_| HttpResponse::BadRequest())?;
        bytes.extend_from_slice(&data);
    }

    if let Err(_) = file.write_all(&bytes) {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Created().finish()
}