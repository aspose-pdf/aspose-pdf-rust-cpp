use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Insert new page at the specified position in PDF-document
    pdf.page_insert(1)?;

    // Save the previously opened PDF-document
    pdf.save()?;

    Ok(())
}
