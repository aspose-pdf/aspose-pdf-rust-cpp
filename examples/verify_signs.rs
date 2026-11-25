use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document named "sample_with_sign.pdf"
    let pdf = Document::open("sample_with_sign.pdf")?;

    // Verify signs from PDF-document
    let signs = pdf.verify_signs()?;

    // Print their status
    for sign in signs {
        println!("Signature: {}, Status: {}", sign.name, sign.status);
    }

    Ok(())
}
