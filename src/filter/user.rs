use crate::{HttpRequest, HttpResponse};
use crate::service::user::USER_SERVICE;
use crate::util::{QResult, to_headers_map, content_type_text};

pub fn login_intercept(http_request: &HttpRequest) -> QResult<Option<HttpResponse>> {
    let headers_map = to_headers_map(&http_request.headers);
    let user_id = headers_map.get("x-fe-user-id");
    let access_token = headers_map.get("x-fe-access-token");
    if user_id.is_none() || access_token.is_none() {
        return Ok(Some(
            HttpResponse::with_code(
                401,
                vec![content_type_text()],
                "请先登录".to_string()
            )
        ))
    }

    let user_id = user_id.unwrap();
    let access_token = access_token.unwrap();

    let user_service = USER_SERVICE.lock().unwrap();
    if user_service.check_user_login(user_id, access_token) {
        Ok(None)
    } else {
        Ok(Some(
            HttpResponse::with_code(
                401,
                vec![content_type_text()],
                "登录信息已过期".to_string()
            )
        ))
    }
}
