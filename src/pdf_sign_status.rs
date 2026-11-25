use crate::enums::SignStatus;
use serde::Deserialize;
/// PdfSignStatus contains status of signature in PDF-document.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PdfSignStatus {
    /// name of signature
    #[serde(rename = "sign_name")]
    pub name: String,

    /// status of signature
    #[serde(rename = "sign_status")]
    pub status: SignStatus,
}
