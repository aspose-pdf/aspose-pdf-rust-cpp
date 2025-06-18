use asposepdf::{Document, Rotation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Rotate PDF-document
    pdf.rotate(Rotation::On270)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_rotate.pdf")?;

    Ok(())
}
