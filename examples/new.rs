use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new PDF-document
    let pdf = Document::new()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_new.pdf")?;

    Ok(())
}
