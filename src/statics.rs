use actix_files::NamedFile;
use actix_web::{get, Error, HttpRequest, Result};
use std::path::PathBuf;

#[get("/static/{filename:.*}")]
async fn read_static(req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut path_string = path.into_os_string().into_string().unwrap();
    path_string = format!("./static/{}", path_string);
    let file = NamedFile::open(path_string)?;

    Ok(file.use_last_modified(true))
}
