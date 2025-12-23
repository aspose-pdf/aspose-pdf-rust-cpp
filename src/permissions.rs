use bitflags::bitflags;
use std::fmt;

bitflags! {
    /// Bitflag set representing PDF permission capabilities.
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Permissions: i32 {
        const PRINT_DOCUMENT                    = 1 << 2;  // 4
        const MODIFY_CONTENT                    = 1 << 3;  // 8
        const EXTRACT_CONTENT                   = 1 << 4;  // 16
        const MODIFY_TEXT_ANNOTATIONS           = 1 << 5;  // 32
        const FILL_FORM                         = 1 << 8;  // 256
        const EXTRACT_CONTENT_WITH_DISABILITIES = 1 << 9;  // 512
        const ASSEMBLE_DOCUMENT                 = 1 << 10; // 1024
        const PRINTING_QUALITY                  = 1 << 11; // 2048
    }
}

/// Human-readable representation of PDF permissions.
impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mapping: &[(Permissions, &str)] = &[
            (Permissions::PRINT_DOCUMENT, "Allow printing"),
            (
                Permissions::MODIFY_CONTENT,
                "Allow modifying content (except forms/annotations)",
            ),
            (
                Permissions::EXTRACT_CONTENT,
                "Allow copying/extracting text and graphics",
            ),
            (
                Permissions::MODIFY_TEXT_ANNOTATIONS,
                "Allow adding/modifying text annotations",
            ),
            (Permissions::FILL_FORM, "Allow filling interactive forms"),
            (
                Permissions::EXTRACT_CONTENT_WITH_DISABILITIES,
                "Allow content extraction for accessibility",
            ),
            (
                Permissions::ASSEMBLE_DOCUMENT,
                "Allow inserting/rotating/deleting pages or changing structure",
            ),
            (
                Permissions::PRINTING_QUALITY,
                "Allow high-quality / faithful printing",
            ),
        ];

        let mut parts = Vec::new();
        for (flag, text) in mapping {
            if self.contains(*flag) {
                parts.push(*text);
            }
        }

        if parts.is_empty() {
            write!(f, "No permissions")
        } else {
            write!(f, "{}", parts.join(", "))
        }
    }
}

/// Debug output listing only the enabled flags in a Rust-like form.
impl fmt::Debug for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mapping: &[(Permissions, &str)] = &[
            (Permissions::PRINT_DOCUMENT, "PRINT_DOCUMENT"),
            (Permissions::MODIFY_CONTENT, "MODIFY_CONTENT"),
            (Permissions::EXTRACT_CONTENT, "EXTRACT_CONTENT"),
            (
                Permissions::MODIFY_TEXT_ANNOTATIONS,
                "MODIFY_TEXT_ANNOTATIONS",
            ),
            (Permissions::FILL_FORM, "FILL_FORM"),
            (
                Permissions::EXTRACT_CONTENT_WITH_DISABILITIES,
                "EXTRACT_CONTENT_WITH_DISABILITIES",
            ),
            (Permissions::ASSEMBLE_DOCUMENT, "ASSEMBLE_DOCUMENT"),
            (Permissions::PRINTING_QUALITY, "PRINTING_QUALITY"),
        ];

        let mut parts = Vec::new();
        for (flag, name) in mapping {
            if self.contains(*flag) {
                parts.push(*name);
            }
        }

        if parts.is_empty() {
            write!(f, "Permissions {:?}", self)
        } else {
            write!(f, "Permissions {{ {} }}", parts.join(" | "))
        }
    }
}

/// Converts an `i32` bitmask into `Permissions`, keeping only known bits.
impl From<i32> for Permissions {
    fn from(value: i32) -> Self {
        Permissions::from_bits_truncate(value)
    }
}
