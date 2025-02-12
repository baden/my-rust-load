use actix_web::{web, HttpResponse, Responder};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn download_file(web::Path((file_name, file_path)): web::Path<(String, String)>) -> impl Responder {
    let full_path = Path::new(&file_path).join(&file_name);
    
    match File::open(&full_path) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if let Err(_) = file.read_to_end(&mut contents) {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(contents)
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}