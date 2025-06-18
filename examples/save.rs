use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document named "sample.pdf"
    let pdf = Document::open("sample.pdf")?;

    // Save the previously opened PDF-document
    pdf.save()?;

    Ok(())
}
