use asposepdf::{Document, Rotation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Load stamp image data
    let image_data = std::fs::read("stamp.png")?;

    // Add stamp to the PDF-document
    pdf.add_stamp(
        "Stamp",
        &image_data,
        true,
        100,
        100,
        50,
        50,
        Rotation::On180,
        0.5,
    )?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_add_stamp.pdf")?;

    Ok(())
}
