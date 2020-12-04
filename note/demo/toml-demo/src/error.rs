#[derive(Debug)]
//错误定义
pub enum ConfigError {

    NotFound,
    IOError,
    BadFilePath(PathBuf,&'static str),
    BadEnv(String),
    BadEntry(String, PathBuf),
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ParseError(String,PathBuf,String,Option<usize,usize>),
}

impl Error for ConfigError {

    fn description(&self) -> &str {
        match *self {
            NotFound => "config file not found",
            IOError => "I/O error",
            BadFilePath(..) => "file  path is invalid",
            BadEnv(..) => "",
            BadEntry(..) => "",
            BadType(..) => "",
            ParseError(..) => "",
        }
    }
}


impl fmt::Display for ConfigFile {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f, "config not found"),
            IOError(ref e) => write!(f,"IOError"),
            BadFilePath(ref p, _) => write!(f,"{:?}", p),
            BadEnv(ref e) => write!(f,"{:?}", e),
            BadEntry(ref e, _) => write!(f,"{:?}", e),
            BadType(ref n,e,a, _) => write!(f,"{} {} {}", n,e,a),
            ParseError(..) => write!(f,"parseError")

        }
    }
}