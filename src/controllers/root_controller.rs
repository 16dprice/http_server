use std::fs;

use crate::controllers::Response;

pub fn root_controller() -> Response {
    return Response {
        status_code: 200,
        contents: fs::read_to_string("templates/hello.html").unwrap(),
    };
}
