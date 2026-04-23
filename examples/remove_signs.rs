use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document named "sample_with_sign.pdf"
    let pdf = Document::open("sample_with_sign.pdf")?;

    // Remove signs from PDF-document
    pdf.remove_signs("sample_remove_signs.pdf")?;

    Ok(())
}
