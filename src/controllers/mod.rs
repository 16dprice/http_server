pub mod root_controller;
pub mod sleep_controller;

pub struct Response {
    pub status_code: usize,
    pub contents: String,
}

pub type Controller = fn() -> Response;
