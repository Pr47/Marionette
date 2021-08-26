use crate::{HttpRequest, HttpResponse};
use crate::util::QResult;

macro_rules! intercept {
    ($request:ident, $filter:expr) => {
        if let Some(response) = $filter(&$request)? {
            return Ok(response);
        }
    }
}

pub fn route_and_handle(request: HttpRequest) -> QResult<HttpResponse> {
    match &request.query_path as &str {
        "/api/login" => {
            intercept!(request, crate::filter::common::post_intercept);
            crate::controller::user::user_login(request)
        },
        "/api/ssh" => {
            intercept!(request, crate::filter::common::get_intercept);
            intercept!(request, crate::filter::user::login_intercept);
            crate::controller::ssh::get_ssh_public_key(request)
        },
        _ => Ok(HttpResponse::with_code(404, vec![], "Not found".into()))
    }
}
