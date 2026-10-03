use std::{fs, thread, time::Duration};

use crate::controllers::Response;

pub fn sleep_controller() -> Response {
    thread::sleep(Duration::from_secs(5));

    return Response {
        status_code: 200,
        contents: fs::read_to_string("templates/hello.html").unwrap(),
    };
}
