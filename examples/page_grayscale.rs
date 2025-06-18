use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Convert page to black and white
    pdf.page_grayscale(1)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_grayscale.pdf")?;

    Ok(())
}
