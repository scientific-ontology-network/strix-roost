use clap::Parser;
use serde::Serialize;
use serde_json::json;
use crate::util::error::StrixError;

pub trait Runnable<T> : Parser{

    fn run(&self)  -> T;
    fn try_run() -> Result<T, StrixError> {
        {
            let cli = Self::try_parse();
            match cli {
                Ok(o) => Ok(o.run()),
                Err(e) => Err(StrixError::Error {message: e.to_string()}),
            }
        }
    }

}
