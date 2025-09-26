use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Remove images in page
    pdf.page_remove_images(1)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_remove_images.pdf")?;

    Ok(())
}
