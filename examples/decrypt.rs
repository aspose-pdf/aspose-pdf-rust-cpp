use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a password-protected PDF-document
    let pdf = Document::open_with_password("sample_with_password.pdf", "ownerpass")?;

    // Decrypt PDF-document
    pdf.decrypt()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_decrypt.pdf")?;

    Ok(())
}
