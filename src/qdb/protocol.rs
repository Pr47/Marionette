use std::error::Error;
use std::io::Write;

pub const QDB_REQUEST_READ: u8 = 1;
pub const QDB_REQUEST_WRITE: u8 = 2;
pub const QDB_REQUEST_DELETE: u8 = 3;

pub enum QDBRequestBody {
    Read { key: String },
    Write { key: String, value: String, overwrite: bool },
    Delete { key: String }
}

pub struct QDBRequest {
    pub namespace: String,
    pub body: QDBRequestBody
}

impl QDBRequest {
    pub fn read_request(namespace: String, key: String) -> Self {
        Self {
            namespace,
            body: QDBRequestBody::Read { key }
        }
    }

    pub fn write_request(namespace: String, key: String, value: String, overwrite: bool) -> Self {
        Self {
            namespace,
            body: QDBRequestBody::Write { key, value, overwrite }
        }
    }

    pub fn delete_request(namespace: String, key: String) -> Self {
        Self {
            namespace,
            body: QDBRequestBody::Delete { key }
        }
    }

    pub fn write<W: Write>(&self, mut output: W) -> Result<(), Box<dyn Error + 'static>> {
        let namespace_len_buf: [u8; 8] = (self.namespace.len() as u64).to_le_bytes();
        output.write_all(&namespace_len_buf)?;

        match &self.body {
            QDBRequestBody::Read { key } => {
                output.write_all(&[QDB_REQUEST_READ])?;
                let key_len_buf: [u8; 8] = (key.len() as u64).to_le_bytes();
                output.write_all(&key_len_buf)?;
                output.write_all(key.as_bytes())?;
            },
            QDBRequestBody::Write { key, value, overwrite } => {
                output.write_all(&[QDB_REQUEST_READ])?;
                let key_len_buf: [u8; 8] = (key.len() as u64).to_le_bytes();
                output.write_all(&key_len_buf)?;
                output.write_all(key.as_bytes())?;
                let value_len_buf: [u8; 8] = (value.len() as u64).to_le_bytes();
                output.write_all(&value_len_buf)?;
                output.write_all(value.as_bytes())?;
                output.write_all(&[*overwrite as u8])?;
            }
            QDBRequestBody::Delete { key } => {
                output.write_all(&[QDB_REQUEST_DELETE])?;
                let key_len_buf: [u8; 8] = (key.len() as u64).to_le_bytes();
                output.write_all(&key_len_buf)?;
                output.write_all(key.as_bytes())?;
            }
        }

        Ok(())
    }
}

pub struct QDBResponse {
    pub success: bool,
    pub message: Option<String>,
    pub result: Option<String>
}

impl QDBResponse {
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            result: None
        }
    }

    pub fn success_with_result(result: String) -> Self {
        Self {
            success: true,
            message: None,
            result: Some(result)
        }
    }

    pub fn failure(reason: String) -> Self {
        Self {
            success: false,
            message: Some(reason),
            result: None
        }
    }
}
