use std::error::Error;
use std::io::{Read, Write};

pub type QError = Box<dyn Error + 'static>;

pub type QResult<T> = Result<T, QError>;

pub fn read_byte<R: Read>(mut input: R) -> QResult<u8> {
    let mut dest: [u8; 1] = [0];
    input.read_exact(&mut dest)?;
    Ok(dest[0])
}

pub fn write_byte<W: Write>(mut write: W, byte: u8) -> QResult<()> {
    write.write_all(&[byte])?;
    Ok(())
}

pub fn read_string<R: Read>(mut input: R) -> QResult<String> {
    let mut len_buf: [u8; 8] = [0; 8];
    input.read_exact(&mut len_buf)?;
    let len: usize = u64::from_le_bytes(len_buf) as usize;

    let mut buf: Vec<u8> = vec![0; len];
    input.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf)?)
}

pub fn write_string<W: Write>(mut output: W, string: &str) -> QResult<()> {
    let len_buf: [u8; 8] = (string.len() as u64).to_le_bytes();
    output.write_all(&len_buf)?;
    output.write_all(string.as_bytes())?;
    Ok(())
}

pub fn read_option_string<R: Read>(mut input: R) -> QResult<Option<String>> {
    let is_some: bool = read_byte(&mut input)? != 0;
    if is_some {
        let string: String = read_string(&mut input)?;
        Ok(Some(string))
    } else {
        Ok(None)
    }
}

pub fn write_option_string<W: Write>(mut output: W, option: &Option<String>) -> QResult<()> {
    if let Some(string) = option.as_ref() {
        write_byte(&mut output, 0)?;
        write_string(&mut output, string)?;
    } else {
        write_byte(&mut output, 0)?;
    }
    Ok(())
}
