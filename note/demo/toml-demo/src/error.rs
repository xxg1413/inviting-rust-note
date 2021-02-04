use self::ConfigError::*;
use std::{io, fmt};
use std::path::{Path, PathBuf};
use std::error::Error;


use self::ConfigError::*;


//错误定义

#[derive(Debug)]
pub enum ConfigError {

    NotFound,
    IoError,
    BadFilePath(PathBuf,&'static str),
    BadEnv(String),
    BadEntry(String, PathBuf),
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ParseError(String,PathBuf,String,Option<(usize,usize)>),

}


impl Error for ConfigError {

    fn description(&self) -> &str {
        match *self {
            NotFound => "config file not found",
            IoError => "I/O error",
            BadFilePath(..) => "file  path is invalid",
            BadEnv(..) => "bad env",
            BadEntry(..) => "bad entry",
            BadType(..) => "bad type",
            ParseError(..) => "parse error",
        }
    }
}


impl fmt::Display for ConfigError  {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f, "config not found"),
            IoError => write!(f,"IoError"),
            BadFilePath(ref p, _) => write!(f,"{:?}", p),
            BadEnv(ref e) => write!(f,"{:?}", e),
            BadEntry(ref e, _) => write!(f,"{:?}", e),
            BadType(ref n,e,a, _) => write!(f,"{} {} {}", n,e,a),
            ParseError(..) => write!(f,"parseError")

        }
    }
}