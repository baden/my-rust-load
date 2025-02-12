pub mod routes {
    use crate::handlers::{upload_file, download_file};
    use warp::Filter;

    pub fn set_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let upload = warp::post()
            .and(warp::path("upload"))
            .and(warp::body::bytes())
            .and_then(upload_file);

        let download = warp::get()
            .and(warp::path("download"))
            .and(warp::path::param())
            .and_then(download_file);

        upload.or(download)
    }
}