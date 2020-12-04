
pub const CONFIG_ENV: &str = "POEM_ENV"


#[derive(Hash, PartialEq,Eq, Debug, Clone,Copy)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

