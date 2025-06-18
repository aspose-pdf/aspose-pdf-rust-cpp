use asposepdf::{Document, Rotation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Rotate page
    pdf.page_rotate(1, Rotation::On180)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_rotate.pdf")?;

    Ok(())
}
