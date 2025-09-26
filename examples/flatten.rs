use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Flatten PDF-document
    pdf.flatten()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_flatten.pdf")?;

    Ok(())
}
