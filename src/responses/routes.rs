use crate::*;

#[get("/<page>")]
pub fn pages(page: &RawStr, storage: State<Storage<Html<String>>>) -> Result<Html<String>, Status>
{
    let mut identifier: &str = &*page;

    if page.is_empty()
    {
        identifier = "home";
    }

    match storage.get(identifier)
    {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/assets/css/<file>")]
pub fn asset_css(file: &RawStr, storage: State<Storage<Css<String>>>) -> Result<Css<String>, Status>
{
    match storage.get(file)
    {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/assets/images/<file>")]
pub fn assets(file: &RawStr, storage: State<Storage<Vec<u8>>>) -> Result<Vec<u8>, Status>
{
    match storage.get(file)
    {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}

#[get("/favicon.ico")]
pub fn asset_favicon(storage: State<Storage<Vec<u8>>>) -> Result<Vec<u8>, Status>
{
    match storage.get("favicon.ico")
    {
        None => Err(Status::NotFound),
        Some(val) => Ok(val.to_owned())
    }
}
