use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Set PDF-document background color using RGB values
    pdf.set_background(200, 100, 101)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_set_background.pdf")?;

    Ok(())
}
