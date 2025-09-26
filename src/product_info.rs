use serde::Deserialize;
use std::fmt;

/// ProductInfo contains metadata about the Aspose.PDF for Rust via C++.
#[derive(Debug, Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "product")]
    pub product: String,

    #[serde(rename = "family")]
    pub family: String,

    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "releasedate")]
    pub release_date: String,

    #[serde(rename = "producer")]
    pub producer: String,

    #[serde(rename = "islicensed")]
    pub is_licensed: bool,
}

impl fmt::Display for ProductInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Product:     {}\n\
             Family:      {}\n\
             Version:     {}\n\
             ReleaseDate: {}\n\
             Producer:    {}\n\
             IsLicensed:  {}",
            self.product,
            self.family,
            self.version,
            self.release_date,
            self.producer,
            self.is_licensed
        )
    }
}
