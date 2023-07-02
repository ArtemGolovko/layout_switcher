use crate::winapi::WCHAR;
use widestring::{U16CStr};
use widestring::error::{Utf16Error, MissingNulTerminator};
use std::error::Error;
use std::fmt::{self, Formatter};

pub trait FromWide {
    type Error;

    fn from_wide(_: &[WCHAR]) -> Result<String, Self::Error>;
}

#[derive(Debug)]
pub enum FromWideError {
    MissingNulTerminator(MissingNulTerminator),
    Utf16Error(Utf16Error)
}

impl fmt::Display for FromWideError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingNulTerminator(e) => e.fmt(f),
            Self::Utf16Error(e) => e.fmt(f)
        }
    }
}

impl Error for FromWideError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::MissingNulTerminator(e) => e,
            Self::Utf16Error(e) => e
        })
    }
}

impl From<MissingNulTerminator> for FromWideError {
    fn from(value: MissingNulTerminator) -> Self {
        Self::MissingNulTerminator(value)
    }
}

impl From<Utf16Error> for FromWideError {
    fn from(value: Utf16Error) -> Self {
        Self::Utf16Error(value)
    }
}

impl FromWide for String {
    type Error = FromWideError;
    
    fn from_wide(wstr: &[WCHAR]) -> Result<String, Self::Error> {
        Ok(U16CStr::from_slice_truncate(wstr)?.to_string()?)
    }
}
