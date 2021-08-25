use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use log::error;
use serde::{Serialize, Deserialize};

use crate::util::QResult;

#[derive(Serialize, Deserialize)]
#[serde(rename="camelCase")]
pub struct User {
    pub name: String,
    pub password: String,
    pub level: i32
}

pub struct UserService {
    registered_users: HashMap<String, User>,
    login_users: HashMap<String, String>
}

impl UserService {
    pub fn init() -> QResult<Self> {
        let file_content = std::fs::read_to_string("users.json")?;
        let registered_users
            = serde_json::from_str::<HashMap<String, User>>(&file_content)?;
        Ok(Self {
            registered_users,
            login_users: HashMap::new()
        })
    }

    pub fn check_user_login(&self, user_name: String, user_token: String) -> bool {
        self.login_users
            .get(&user_name)
            .map_or(false, |token| token == &user_token)
    }

    pub fn user_login(&mut self, user_name: String, user_token: String) {
        self.login_users.insert(user_name, user_token);
    }

    pub fn get_user(&self, user_name: &String) -> Option<&User> {
        self.registered_users.get(user_name)
    }

    pub fn add_user(&mut self, user: User) {
        self.registered_users.insert(user.name.clone(), user);
        let user_config = serde_json::to_string(&self.registered_users).unwrap();
        let _ = std::fs::write("users.json", user_config).map_err(|e| {
            error!("failed persisting user information: {}", e)
        });
    }

    pub fn update_user(&mut self, user: User) {
        let _ = self.registered_users.insert(user.name.clone(), user);
        let user_config = serde_json::to_string(&self.registered_users).unwrap();
        let _ = std::fs::write("users.json", user_config).map_err(|e| {
            error!("failed persisting user information: {}", e)
        });
    }
}

lazy_static! {
    pub static ref USER_SERVICE: Mutex<UserService> = Mutex::new(UserService::init().unwrap());
}
