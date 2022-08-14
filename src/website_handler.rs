use super::server::Handler;
use super::http::{Request, Response, Status, Method};
use std::fs;
extern crate dunce;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match dunce::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        dbg!(request);
        match request.method() {
            Method::GET => match request.path() {
                path => match self.read_file(path) {
                    Some(contents) => Response::new(Status::Ok, Some(contents)),
                    None => Response::new(Status::NotFound, None),
                }
            },
            _ => Response::new(Status::NotFound, None),
        }
    }
}