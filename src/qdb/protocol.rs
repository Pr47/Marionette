use std::io::{Read, Write};

use crate::util::{
    QResult,
    read_byte,
    read_option_string,
    read_string,
    write_byte,
    write_option_string,
    write_string
};

pub const QDB_REQUEST_READ: u8 = 1;
pub const QDB_REQUEST_WRITE: u8 = 2;
pub const QDB_REQUEST_DELETE: u8 = 3;
pub const QDB_REQUEST_CREATE_NAMESPACE: u8 = 4;
pub const QDB_REQUEST_DELETE_NAMESPACE: u8 = 5;

pub enum QDBRequestBody {
    Read { key: String },
    Write { key: String, value: String, overwrite: bool },
    Delete { key: String },
    CreateNamespace,
    DeleteNamespace
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

    pub fn create_namespace(namespace: String) -> Self {
        Self {
            namespace,
            body: QDBRequestBody::CreateNamespace
        }
    }

    pub fn delete_namespace(namespace: String) -> Self {
        Self {
            namespace,
            body: QDBRequestBody::DeleteNamespace
        }
    }

    pub fn read<R: Read>(mut input: R) -> QResult<Self> {
        let namespace: String = read_string(&mut input)?;
        let kind: u8 = read_byte(&mut input)?;
        match kind {
            QDB_REQUEST_READ => {
                let key: String = read_string(&mut input)?;
                Ok(Self::read_request(namespace, key))
            },
            QDB_REQUEST_WRITE => {
                let key: String = read_string(&mut input)?;
                let value: String = read_string(&mut input)?;
                let overwrite: bool = read_byte(&mut input)? == 0;
                Ok(Self::write_request(namespace, key, value, overwrite))
            },
            QDB_REQUEST_DELETE => {
                let key: String = read_string(&mut input)?;
                Ok(Self::delete_request(namespace, key))
            },
            QDB_REQUEST_CREATE_NAMESPACE => {
                Ok(Self::create_namespace(namespace))
            },
            QDB_REQUEST_DELETE_NAMESPACE => {
                Ok(Self::delete_namespace(namespace))
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
            QDBRequestBody::CreateNamespace => {
                write_byte(&mut output, QDB_REQUEST_CREATE_NAMESPACE)?;
            }
            QDBRequestBody::DeleteNamespace => {
                write_byte(&mut output, QDB_REQUEST_DELETE_NAMESPACE)?;
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

    pub fn write<W: Write>(&self, mut output: W) -> QResult<()> {
        write_byte(&mut output, self.success as u8)?;
        write_option_string(&mut output, &self.message)?;
        write_option_string(&mut output, &self.result)?;
        Ok(())
    }
}
