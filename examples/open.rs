use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document named "sample.pdf"
    let pdf = Document::open("sample.pdf")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_open.pdf")?;

    Ok(())
}
