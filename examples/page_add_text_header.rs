use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Add text in page header
    pdf.page_add_text_header(1, "HEADER")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_add_text_header.pdf")?;

    Ok(())
}
