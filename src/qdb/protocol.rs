use std::io::{Read, Write};

use crate::util::{QResult, read_string, read_byte, write_string, write_byte, read_option_string};

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

    pub fn read<R: Read>(mut input: R) -> QResult<Self> {
        let namespace: String = read_string(&mut input)?;
        let kind: u8 = read_byte(&mut input)?;
        match kind {
            0 => {
                let key: String = read_string(&mut input)?;
                Ok(Self::read_request(namespace, key))
            },
            1 => {
                let key: String = read_string(&mut input)?;
                let value: String = read_string(&mut input)?;
                let overwrite: bool = read_byte(&mut input)? == 0;
                Ok(Self::write_request(namespace, key, value, overwrite))
            },
            2 => {
                let key: String = read_string(&mut input)?;
                Ok(Self::delete_request(namespace, key))
            },
            _ => {
                Err("[qdb/protocol] Invalid request type".to_string().into())
            }
        }
    }

    pub fn write<W: Write>(&self, mut output: W) -> QResult<()> {
        write_string(&mut output, &self.namespace)?;
        match &self.body {
            QDBRequestBody::Read { key } => {
                write_byte(&mut output, QDB_REQUEST_READ)?;
                write_string(&mut output, key)?;
            },
            QDBRequestBody::Write { key, value, overwrite } => {
                write_byte(&mut output, QDB_REQUEST_WRITE)?;
                write_string(&mut output, key)?;
                write_string(&mut output, value)?;
                write_byte(&mut output, *overwrite as u8)?;
            }
            QDBRequestBody::Delete { key } => {
                write_byte(&mut output, QDB_REQUEST_DELETE)?;
                write_string(&mut output, key)?;
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

    pub fn read<R: Read>(mut input: R) -> QResult<Self> {
        let success: bool = read_byte(&mut input)? != 0;
        let message: Option<String> = read_option_string(&mut input)?;
        let result: Option<String> = read_option_string(&mut input)?;
        Ok(Self { success, message, result })
    }
}
