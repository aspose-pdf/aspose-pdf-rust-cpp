use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Remove hidden text from PDF-document
    pdf.remove_hidden_text()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_remove_hidden_text.pdf")?;

    Ok(())
}
