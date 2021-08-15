use crate::{HttpRequest, HttpResponse};
use crate::util::QResult;

pub fn route_and_handle(_request: HttpRequest) -> QResult<HttpResponse> {
    Ok(HttpResponse::new(
        vec![
            ("X-Require-Certificate".into(), "1".into())
        ],
        "Hello from Rust, via DCGI!".into()
    ))
}
