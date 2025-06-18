use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Add text on page
    pdf.page_add_text(1, "added text")?;

    // Save the previously opened PDF-document
    pdf.save()?;

    Ok(())
}
