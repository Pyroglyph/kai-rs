pub mod v1;

use std::fmt;

pub enum APIVersion {
    V1,
}

impl fmt::Display for APIVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIVersion::V1 => write!(f, "v1"),
        }
    }
}
