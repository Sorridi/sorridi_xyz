#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use] extern crate rocket;

#[path = "utils/imports.rs"]
mod imports;
use imports::*;

const EXT_ASSETS: [&str; 3] = [".jpg", ".png", ".ico"];
const EXT_PAGES: [&str; 1] = [".html"];
const EXT_CSS: [&str; 1] = [".css"];

fn main()
{
    let mut assets: Storage<Vec<u8>> = Storage::init();
    let mut css: Storage<Css<String>> = Storage::init();
    let mut pages: Storage<Html<String>> = Storage::init();

    let paths = match fs::read_dir("./front/pages/")
    {
        Err(e) => panic!("Could not load contents: {}", e),
        Ok(val) => val
    };

    for path in paths
    {
        let path = match path
        {
            Err(e) => panic!("Could not get path: {}", e),
            Ok(val) => val
        };

        let name = String::from(path.file_name().to_str().unwrap());
        if !name.contains(".")
        {
            continue;
        }
        println!("| Found: {}", name);

        let path = format!("./front/pages/{}", &*name);

        for elem in EXT_PAGES.iter()
        {
            if !name.ends_with(elem)
            {
                continue;
            }
            pages.push(&*name.trim_end_matches(elem), Html(read_string(&*path)));
            println!("| ^ as page.");
        }
    }

    let paths = match fs::read_dir("./front/css/")
    {
        Err(e) => panic!("Could not load contents: {}", e),
        Ok(val) => val
    };

    for path in paths
    {
        let path = match path
        {
            Err(e) => panic!("Could not get path: {}", e),
            Ok(val) => val
        };

        let name = String::from(path.file_name().to_str().unwrap());
        if !name.contains(".")
        {
            continue;
        }
        println!("| Found: {}", name);

        let path = format!("./front/css/{}", &*name);

        for elem in EXT_CSS.iter()
        {
            if !name.ends_with(elem)
            {
                continue;
            }
            css.push(&*name.trim_end_matches(elem), Css(read_string(&*path)));
            println!("| ^ as css asset.");
        }
    }

    let paths = match fs::read_dir("./front/images/")
    {
        Err(e) => panic!("Could not load contents: {}", e),
        Ok(val) => val
    };

    for path in paths
    {
        let path = match path
        {
            Err(e) => panic!("Could not get path: {}", e),
            Ok(val) => val
        };

        let name = String::from(path.file_name().to_str().unwrap());
        if !name.contains(".")
        {
            continue;
        }
        println!("| Found: {}", name);

        let path = format!("./front/images/{}", &*name);

        for elem in EXT_ASSETS.iter()
        {
            if name.ends_with(elem)
            {
                continue;
            }
            assets.push(&*name.trim_end_matches(elem), read_bytes(&*path));
            println!("| ^ as asset.");
        }
    }

    rocket::ignite().mount("/", routes!
    [
        asset_css, assets, asset_favicon, pages
    ])
        .manage(assets)
        .manage(pages)
        .manage(css)
        .launch();
}
