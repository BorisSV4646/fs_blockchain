use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    TomlParse(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    IgdSearch(igd::SearchError),
    IgdAddPort(igd::AddPortError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            AppError::TomlParse(e) => write!(f, "TOML parse error: {}", e),
            AppError::TomlSerialize(e) => write!(f, "TOML serialize error: {}", e),
            AppError::IgdSearch(e) => write!(f, "UPnP search error: {}", e),
            AppError::IgdAddPort(e) => write!(f, "UPnP add port error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<igd::SearchError> for AppError {
    fn from(e: igd::SearchError) -> Self {
        AppError::IgdSearch(e)
    }
}

impl From<igd::AddPortError> for AppError {
    fn from(e: igd::AddPortError) -> Self {
        AppError::IgdAddPort(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Reqwest(e)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        AppError::TomlParse(e)
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(e: toml::ser::Error) -> Self {
        AppError::TomlSerialize(e)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
