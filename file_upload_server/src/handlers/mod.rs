use warp::Filter;
use std::fs;
use std::path::PathBuf;

pub async fn upload_file(file: warp::multipart::FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let parts: Vec<_> = file.collect().await.map_err(|_| warp::reject())?;
    
    for part in parts {
        let filename = part.filename().unwrap_or("file");
        let filepath = PathBuf::from(format!("./uploads/{}", filename));

        let data = part.stream().try_concat().await.map_err(|_| warp::reject())?;
        fs::write(filepath, &data).map_err(|_| warp::reject())?;
    }

    Ok(warp::reply::with_status("File uploaded", warp::http::StatusCode::OK))
}

pub async fn download_file(filename: String) -> Result<impl warp::Reply, warp::Rejection> {
    let filepath = PathBuf::from(format!("./uploads/{}", filename));

    if !filepath.exists() {
        return Err(warp::reject());
    }

    let data = fs::read(filepath).map_err(|_| warp::reject())?;
    Ok(warp::reply::with_header(data, "Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
}