use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Add new page in PDF-document
    pdf.page_add()?;

    // Save the previously opened PDF-document
    pdf.save()?;

    Ok(())
}
