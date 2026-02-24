use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Crop a page
    pdf.page_crop(1, 1.0)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_crop.pdf")?;

    Ok(())
}
