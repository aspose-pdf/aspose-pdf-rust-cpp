/// An enumeration of possible rotation values.
#[derive(Debug, Clone)]
#[repr(C)]
pub enum Rotation {
    /// Non-rotated.
    None = 0,
    /// Rotated on 90 degrees clockwise.
    On90 = 1,
    /// Rotated on 180 degrees.
    On180 = 2,
    /// Rotated on 270 degrees clockwise.
    On270 = 3,
    /// Rotated on 360 degrees clockwise.
    On360 = 4,
}

/// An enumeration of possible page size values.
#[derive(Debug, Clone)]
#[repr(C)]
pub enum PageSize {
    /// A0 size.
    A0 = 0,
    /// A1 size.
    A1 = 1,
    /// A2 size.
    A2 = 2,
    /// A3 size.
    A3 = 3,
    /// A4 size.
    A4 = 4,
    /// A5 size.
    A5 = 5,
    /// A6 size.
    A6 = 6,
    /// B5 size.
    B5 = 7,
    /// PageLetter size.
    PageLetter = 8,
    /// PageLegal size.
    PageLegal = 9,
    /// PageLedger size.
    PageLedger = 10,
    /// P11x17 size.
    P11x17 = 11,
}

/// An enumeration of possible crypto algorithms.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum CryptoAlgorithm {
    /// RC4 with key length 40.
    RC4x40 = 0,
    /// RC4 with key length 128.
    RC4x128 = 1,
    /// AES with key length 128.
    AESx128 = 2,
    /// AES with key length 256.
    AESx256 = 3,
}

/// An enumeration of possible PDF format standards.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum PdfFormat {
    /// PDF/A-1a format.
    PDF_A_1A = 0,
    /// PDF/A-1b format.
    PDF_A_1B = 1,
    /// PDF/A-2a format.
    PDF_A_2A = 2,
    /// PDF/A-3a format.
    PDF_A_3A = 3,
    /// PDF/A-2b format.
    PDF_A_2B = 4,
    /// PDF/A-2u format.
    PDF_A_2U = 5,
    /// PDF/A-3b format.
    PDF_A_3B = 6,
    /// PDF/A-3u format.
    PDF_A_3U = 7,
    /// Adobe version 1.0.
    V_1_0 = 8,
    /// Adobe version 1.1.
    V_1_1 = 9,
    /// Adobe version 1.2.
    V_1_2 = 10,
    /// Adobe version 1.3.
    V_1_3 = 11,
    /// Adobe version 1.4.
    V_1_4 = 12,
    /// Adobe version 1.5.
    V_1_5 = 13,
    /// Adobe version 1.6.
    V_1_6 = 14,
    /// Adobe version 1.7.
    V_1_7 = 15,
    /// ISO Standard PDF 2.0.
    V_2_0 = 16,
    /// PDF/UA-1 format.
    PDF_UA_1 = 17,
    /// PDF/X-1a:2001 format.
    PDF_X_1A_2001 = 18,
    /// PDF/X-1a format.
    PDF_X_1A = 19,
    /// PDF/X-3 format.
    PDF_X_3 = 20,
    /// ZUGFeRD format.
    ZUGFeRD = 21,
    /// PDF/A-4 format.
    PDF_A_4 = 22,
    /// PDF/A-4e format.
    PDF_A_4E = 23,
    /// PDF/A-4f format.
    PDF_A_4F = 24,
    /// PDF/X-4 format.
    PDF_X_4 = 25,
    /// PDF/E-1 (PDF 1.6) format.
    PDF_E_1 = 26,
}

/// An enumeration of actions to take when a conversion error occurs.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum ConvertErrorAction {
    /// Delete non-conforming elements.
    Delete = 0,
    /// Do nothing, keep non-conforming elements.
    None = 1,
}
