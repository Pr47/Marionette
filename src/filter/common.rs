use crate::{HttpMethod, HttpRequest, HttpResponse};
use crate::util::{QResult, content_type_text};

pub fn get_intercept(http_request: &HttpRequest) -> QResult<Option<HttpResponse>> {
    if http_request.method == HttpMethod::Post {
        Ok(Some(HttpResponse::with_code(
            403,
            vec![content_type_text()],
            format!("cannot POST {}", http_request.query_path)
        )))
    } else {
        Ok(None)
    }
}

pub fn post_intercept(http_request: &HttpRequest) -> QResult<Option<HttpResponse>> {
    if http_request.method == HttpMethod::Get {
        Ok(Some(HttpResponse::with_code(
            403,
            vec![content_type_text()],
            format!("cannot GET {}", http_request.query_path)
        )))
    } else {
        Ok(None)
    }
}
