use asposepdf::{Document, Permissions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a password-protected PDF-document
    let pdf = Document::open_with_password("sample_with_permissions.pdf", "ownerpass")?;

    // Get current permissions of PDF-document
    let permissions: Permissions = pdf.get_permissions()?;

    // Print permissions
    println!("Permissions: {}", permissions);

    Ok(())
}
