use anyhow::Result;
use clap::{App, Arg};
use std::convert::TryFrom;
use thiserror::Error;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Error, Debug, Clone)]
pub enum MyError {
    #[error("パラメータが不正です。")]
    InvalidArgument,
}

pub enum Output {
    ULID,
    UUID,
}

impl TryFrom<&str> for Output {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        let res = match value {
            "ulid" => Self::ULID,
            "uuid" => Self::UUID,
            _ => return Err(MyError::InvalidArgument.into()),
        };
        Ok(res)
    }
}

fn main() -> Result<()> {
    let output: Arg = Arg::new("output")
        .short('o')
        .long("output")
        .default_value("ulid")
        .help("-o: This parameter determines the output type. ulid or uuid.");
    let try_count: Arg = Arg::new("count")
        .short('c')
        .long("count")
        .default_value("10")
        .help("-c: This parameter determines the output length.");
    let app: App = App::new("ID Generator")
        .author("Takumi Kobayashi")
        .version("v1.0.0")
        .about("This application generates ULID or UUID.")
        .args([output, try_count]);
    let inputs = app.try_get_matches()?;
    let out = match inputs.value_of("output") {
        Some(out) => Output::try_from(out)?,
        _ => return Err(MyError::InvalidArgument.into()),
    };
    let count = match inputs.value_of("count") {
        Some(count) => count.parse::<i32>()?,
        _ => return Err(MyError::InvalidArgument.into()),
    };
    for _ in 0..count {
        match out {
            Output::ULID => {
                println!("{}", Ulid::new().to_string());
            }
            Output::UUID => {
                println!("{}", Uuid::new_v4().to_string());
            }
        }
    }
    Ok(())
}
