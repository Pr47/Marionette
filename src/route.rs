use crate::{HttpRequest, HttpResponse};
use crate::util::QResult;

pub fn route_and_handle(request: HttpRequest) -> QResult<HttpResponse> {
    match &request.query_path as &str {
        "/api/login" => crate::controller::user::user_login(request),
        _ => Ok(HttpResponse::with_code(404, vec![], "Not found".into()))
    }
}
