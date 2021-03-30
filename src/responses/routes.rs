use crate::*;

#[get("/")]
pub fn page_home(storage: State<Storage<Html<String>>>) -> Result<Html<String>, Status> {
    match storage.get("home.html") {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/<page>")]
pub fn pages(page: &RawStr, storage: State<Storage<Html<String>>>) -> Result<Html<String>, Status> {
    match storage.get(page) {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/assets/style.css")]
pub fn asset_css(storage: State<Storage<Css<String>>>) -> Result<Css<String>, Status> {
    match storage.get("style.css") {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/assets/<file>")]
pub fn assets(file: &RawStr, storage: State<Storage<Vec<u8>>>) -> Result<Vec<u8>, Status> {
    match storage.get(file) {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/favicon.ico")]
pub fn asset_favicon(storage: State<Storage<Vec<u8>>>) -> Result<Vec<u8>, Status> {
    match storage.get("favicon.ico") {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}
