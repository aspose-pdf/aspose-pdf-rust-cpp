use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a password-protected PDF-document
    let pdf = Document::open_with_password("sample_with_password.pdf", "ownerpass")?;

    // Get encrypted status of PDF-document
    if pdf.is_encrypted()? {
        println!("The document is encrypted.");
    }

    Ok(())
}
