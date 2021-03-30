#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[path = "utils/imports.rs"]
mod imports;
use imports::*;

const EXT_ASSETS: [&str; 3] = [".jpg", ".png", ".ico"];
const EXT_PAGES: [&str; 1] = [".html"];
const EXT_CSS: [&str; 1] = [".css"];

fn main(){

    println!("Initializing...");

    let mut assets: Storage<Vec<u8>> = Storage::init();
    let mut css: Storage<Css<String>> = Storage::init();
    let mut pages: Storage<Html<String>> = Storage::init();

    let paths = match fs::read_dir("./front/") {
        Err(e) => panic!("Could not load contents: {}", e),
        Ok(val) => val
    };

    for path in paths {
        let path = match path {
            Err(e) => panic!("Could not get path: {}", e),
            Ok(val) => val
        };

        let name: String = String::from(path.file_name().to_str().unwrap());

        println!("| Found: {}", name);

        let path = format!("./front/{}", &*name);

        for elem in EXT_PAGES.iter() {
            if name.contains(elem) {
                pages.push(&*name, Html(read_string(&*path)));
                println!("| ^ as page.");
            }
        }

        for elem in EXT_CSS.iter() {
            if name.contains(elem) {
                css.push(&*name, Css(read_string(&*path)));
                println!("| ^ as css asset.");
            }
        }

        for elem in EXT_ASSETS.iter() {
            if name.contains(elem) {
                assets.push(&*name, read_bytes(&*path));
                println!("| ^ as asset.");
            }
        }

    }

    println!("Pages and Assets has been loaded.");

    let config = Config::build(Environment::Production)
    .address("0.0.0.0")
    .port(80)
    .workers(2)
    .finalize().unwrap();

    rocket::custom(config).mount("/", routes!
    [
        asset_css, assets, asset_favicon, page_home, pages
    ])
        .manage(assets)
        .manage(pages)
        .manage(css)
        .launch();

}
