use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::process::Command;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct ConnectError();

impl Display for ConnectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cant connect to the server.")
    }
}

impl std::error::Error for ConnectError {}

#[derive(Debug)]
pub enum CurlErrors{
    Connection(ConnectError),
    Output(std::io::Error),
    Utf8Error(FromUtf8Error),
}

impl Display for CurlErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CurlErrors::Connection(c) => format!("{}", c),
            CurlErrors::Output(o) => format!("{}", o),
            CurlErrors::Utf8Error(u) => format!("{}", u),
        })
    }
}

impl std::error::Error for CurlErrors {}

impl From<ConnectError> for CurlErrors{
    fn from(obj: ConnectError) -> Self {
        CurlErrors::Connection(obj)
    }
}

impl From<std::io::Error> for CurlErrors{
    fn from(obj: std::io::Error) -> Self {
        CurlErrors::Output(obj)
    }
}

impl From<FromUtf8Error> for CurlErrors{
    fn from(obj: FromUtf8Error) -> Self {
        CurlErrors::Utf8Error(obj)
    }
}

pub fn curl<I, S>(args: I) -> Result<String, CurlErrors>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>{
    let curl = Command::new("curl")
        .args(args)
        .output()?;
    if !curl.status.success() {
        return Err(ConnectError {}.into());
    }

    Ok(String::from_utf8(curl.stdout)?)
}