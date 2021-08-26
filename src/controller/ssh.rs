use directories::BaseDirs;

use crate::{HttpRequest, HttpResponse};
use crate::util::{QResult, ResponseUtil, content_type_json};

pub fn get_ssh_public_key(_request: HttpRequest) -> QResult<HttpResponse> {
    if let Some(base_dirs) = BaseDirs::new() {
        let ssh_public_key_path = base_dirs.home_dir().join(".ssh/id_rsa.pub");
        if let Ok(ssh_public_key_content) = std::fs::read_to_string(ssh_public_key_path) {
            Ok(HttpResponse::new(
                vec![content_type_json()],
                ResponseUtil::data(ssh_public_key_content)
            ))
        } else {
            Ok(HttpResponse::new(
                vec![content_type_json()],
                ResponseUtil::error("服务器上不存在公钥文件 .ssh/id_rsa.pub")
            ))
        }
    } else {
        Err("服务器内部错误".into())
    }
}
