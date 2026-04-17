use serde::Serialize;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum LoamError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(String),

    #[error("path error: {0}")]
    Path(String),
}

impl Serialize for LoamError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type LoamResult<T> = Result<T, LoamError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_errors_convert() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "nope");
        let loam_err: LoamError = io_err.into();
        assert!(matches!(loam_err, LoamError::Io(_)));
        assert!(loam_err.to_string().contains("nope"));
    }

    #[test]
    fn serialize_as_string() {
        let err = LoamError::Path("bad".into());
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"path error: bad\"");
    }
}
