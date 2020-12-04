use num_cpus;


const CONFIG_FILENAME: &str = "conf/Poem.toml"


struct BasicConfig {
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    pub database: Option<Database>,
    pub workers: Option<u16>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,

}

impl BasicConfig {

    pub fn new(env: Environment) -> Self {
        self::default(env);
    }

    //默认值
    pub(crate) fn default(env: Environment) -> Self {


        //默认线程
        let default_workers = (num_cpus::get() * 2 ) as u16;

        match env {
            Development => {
                BasicConfig {
                    environment: Development,
                    address: "localhost".to_string(),
                    port:8000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
                }
            }

            Staging => {
                BasicConfig {
                    environment: Staging,
                    address: "0.0.0.0".to_string(),
                    port:9000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
                }

            }
            
            Production => {
                BasicConfig {
                    environment: Production,
                    address: "0.0.0.0".to_string(),
                    port:9000,
                    database: None,
                    workers: Some(default_workers),
                    config_file_path: None,
                    root_path: None,
                }
            }
        }
    }


    pub fn set_root(P: AsRef<Path> ->(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    //默认从文件路径读配置
    fn default_from<P>( env: Environment, path: P ) -> Result<Self> where P: AsRef<Path> {

        let mut  config = BasicConfig::default();
        
        let config_file_path = path.as_ref().to_path_buf();

        if let some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must in dir";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }

        config.config_file_path = Some(config_file_path)
        OK(config)

    }



    impl PartialEq for BasicConfig {
        fn eq(&self, other: &BasicConfig) -> bool {
            self.address == other.address
                && self.port == other.port
                && self.workers == other.workers
        }
    }

}

#[derive(Debug)]
struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: u32,
}



#[doc(hidden)]
#[derive(Debug, PartialEq)]
struct PoemConfig {

    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,

}


impl PoemConfig {

    pub fn read_config() -> Result<PoemConfig> {

        let file = PoemConfig::find()?;
        

    }


    fn find() -> Result<PathBuf> {
        
    }
}

