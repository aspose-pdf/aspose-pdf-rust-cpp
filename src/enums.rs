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
