use serde::{Serialize, Deserialize};

use crate::{HttpRequest, HttpResponse};
use crate::util::{QResult, ResponseUtil, uuid};
use crate::service::user::USER_SERVICE;

#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct LoginRequest {
    user_name: String,
    password: String
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct LoginResponse {
    user_id: String,
    access_token: String
}

pub fn user_login(http_request: HttpRequest) -> QResult<HttpResponse> {
    let login_request = serde_json::from_str::<LoginRequest>(&http_request.body)?;
    let mut user_service = USER_SERVICE.lock().unwrap();
    Ok(
        if let Some(user) = user_service.get_user(&login_request.user_name) {
            if user.password == login_request.password {
                let access_token = uuid();
                let resp = LoginResponse {
                    user_id: user.name.clone(),
                    access_token: access_token.clone()
                };
                user_service.user_login(login_request.user_name, access_token);
                HttpResponse::new(vec![], ResponseUtil::data(resp))
            } else {
                HttpResponse::new(vec![], ResponseUtil::error("用户名或密码错误"))
            }
        } else {
            HttpResponse::new(vec![], ResponseUtil::error("用户名或密码错误"))
        }
    )
}