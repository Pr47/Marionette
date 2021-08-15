use std::error::Error;

use crate::{HttpRequest, HttpResponse};

pub fn route_and_handle(_request: HttpRequest) -> Result<HttpResponse, Box<dyn Error + 'static>> {
    Ok(HttpResponse::new(
        vec![
            ("X-Require-Certificate".into(), "1".into())
        ],
        "Hello from Rust, via DCGI!".into()
    ))
}
