use std::collections::HashMap;
use std::error::Error;

use serde::Serialize;

pub type QError = Box<dyn Error + 'static>;

pub type QResult<T> = Result<T, QError>;

pub fn to_headers_map(input: Vec<(String, String)>) -> HashMap<String, String> {
    let mut ret = HashMap::new();
    for (k, v) in input.into_iter() {
        ret.insert(k.to_lowercase(), v);
    }
    ret
}

#[derive(Serialize)]
pub struct ResponseNoData {
    success: bool,
    message: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseData<T: Serialize> {
    success: bool,
    result: T
}

pub struct ResponseUtil();

impl ResponseUtil {
    pub fn success<>() -> String {
        serde_json::to_string(&ResponseNoData {
            success: true,
            message: None
        }).unwrap()
    }

    pub fn data<T: Serialize>(result: T) -> String {
        serde_json::to_string(&ResponseData {
            success: true,
            result
        }).unwrap()
    }

    pub fn error(message: impl ToString) -> String {
        serde_json::to_string(&ResponseNoData {
            success: false,
            message: Some(message.to_string())
        }).unwrap()
    }
}

pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
