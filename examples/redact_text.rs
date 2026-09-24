use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Redact permanently and blacks out sensitive text in PDF-document
    pdf.redact_text("aspose|pdf")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_redact_text.pdf")?;

    Ok(())
}
