use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Delete specified page in PDF-document
    pdf.page_delete(1)?;

    // Save the previously opened PDF-document
    pdf.save()?;

    Ok(())
}
