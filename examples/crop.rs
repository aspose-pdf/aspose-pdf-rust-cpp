use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Crop pages of a PDF-document
    pdf.crop(10.5)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_crop.pdf")?;

    Ok(())
}
