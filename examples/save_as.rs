use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new PDF-document
    let pdf = Document::new()?;

    // Save the PDF-document with new filename
    pdf.save_as("sample_save_as.pdf")?;

    Ok(())
}
