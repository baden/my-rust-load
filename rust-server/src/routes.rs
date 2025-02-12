mod actix_web;
use actix_web::{web, HttpResponse, Responder};

use crate::handlers::{upload::upload_file, download::download_file};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/upload/{filename}/{path:.*}")
            .route(web::post().to(upload_file))
    )
    .service(
        web::resource("/download/{filename}/{path:.*}")
            .route(web::get().to(download_file))
    );
}