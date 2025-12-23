use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a password-protected PDF-document
    let _pdf = Document::open_with_password("sample_with_password.pdf", "ownerpass")?;

    // working...

    Ok(())
}
