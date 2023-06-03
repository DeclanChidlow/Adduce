use std::process::{Child, Command};

use crate::common::result::{Error, ErrorConvert};

pub struct Com();

impl Com {
    pub fn spawn(command: &str, args: Option<Vec<&str>>) -> Result<(), Error> {
        Com::common(command, args)?.wait().res()?;
        Ok(())
    }
    pub fn results(command: &str, args: Option<Vec<&str>>) -> Result<String, Error> {
        let data = Com::common(command, args)?.wait_with_output().res()?;

        match (to_string_ignore(data.stdout), to_string_ignore(data.stderr)) {
            a if a.0.is_empty() && !a.1.is_empty() => Err(Error::Command(a.1)),
            a if a.1.is_empty() && !a.0.is_empty() => Ok(a.0),
            _ => Err(Error::Command(String::from("illogical command error"))),
        }
    }
    pub fn common(command: &str, args: Option<Vec<&str>>) -> Result<Child, Error> {
        let mut cmd = Command::new(command);

        if let Some(args) = args {
            cmd.args(args);
        };
        cmd.spawn().res()
    }
}

fn to_string_ignore(input: Vec<u8>) -> String {
    String::from_utf8(input).unwrap_or_default()
}
