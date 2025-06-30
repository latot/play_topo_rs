use pgrx::spi::SpiError;

#[derive(Debug, PartialEq)]
pub enum Error {
        SpiError(SpiError),
        NotOneRow,
        NoRows,
        MissingField(String),
}

impl From<SpiError> for Error {
        fn from(value: SpiError) -> Self {
                Error::SpiError(value)
        }
}

impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                        f,
                        "{}",
                        match self {
                                Error::NoRows => String::from("No rows found"),
                                Error::NotOneRow => String::from("More than one row found"),
                                Error::SpiError(x) => format!("{:?}", x),
                                Error::MissingField(x) => format!("Missing field: {}", x),
                        }
                )
        }
}

impl std::error::Error for Error {}
