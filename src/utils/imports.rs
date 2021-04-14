#[path = "files.rs"]
pub mod files;
pub use files::*;

#[path = "../responses/contents.rs"]
pub mod contents;
pub use contents::*;

#[path = "../responses/routes.rs"]
pub mod routes;
pub use routes::*;

pub use derive_getters::Getters;
pub use rocket::config::{Config, Environment};
pub use rocket::response::content::{Html, Css};
pub use rocket::http::{RawStr, Status};

pub use std::fs;
pub use std::fs::{File, ReadDir};
pub use std::io::{BufReader, prelude::*, Error};
pub use std::path::Path;
pub use std::collections::HashMap;
pub use std::ops::{Deref, DerefMut};
pub use rocket::State;
pub use std::str::FromStr;
