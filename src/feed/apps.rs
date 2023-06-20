use std::process::{Child, Command};

use crate::common::result::{Error, ErrorConvert, ErrorType};

pub struct Com();

impl Com {
    pub fn spawn(command: &str, args: Option<Vec<&str>>) -> Result<(), Error> {
        Com::common(command, args)?
            .wait()
            .res_ignore(Some("falied to start session for command"))
    }
    pub fn results(command: &str, args: Option<Vec<&str>>) -> Result<String, Error> {
        let data = Com::common(command, args)?
            .wait_with_output()
            .res_msg(Some("failed to get results of command"))?;

        match (to_string_ignore(data.stdout), to_string_ignore(data.stderr)) {
            a if a.0.is_empty() && !a.1.is_empty() => Err(ErrorType::Command(a.1).into()),
            a if a.1.is_empty() && !a.0.is_empty() => Ok(a.0),
            _ => Err(ErrorType::Command(String::from("illogical command error")).into()),
        }
    }
    pub fn common(command: &str, args: Option<Vec<&str>>) -> Result<Child, Error> {
        let mut cmd = Command::new(command);

        if let Some(args) = args {
            cmd.args(args);
        };
        cmd.spawn().res_msg(Some("failed to start command"))
    }
}

fn to_string_ignore(input: Vec<u8>) -> String {
    String::from_utf8(input).unwrap_or_default()
}
