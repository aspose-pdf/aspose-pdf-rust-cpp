use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Remove attachments from PDF-document
    pdf.remove_attachments()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_remove_attachments.pdf")?;

    Ok(())
}
