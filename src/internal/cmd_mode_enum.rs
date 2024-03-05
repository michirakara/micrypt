use thiserror::Error;

#[derive(Debug)]
pub enum CmdMode {
    Enc,
    Dec,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("mode {0} not available")]
    UnavailableMode(String),
}

impl std::str::FromStr for CmdMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enc" => Ok(CmdMode::Enc),
            "dec" => Ok(CmdMode::Dec),
            _ => Err(Error::UnavailableMode(s.to_string())),
        }
    }
}
