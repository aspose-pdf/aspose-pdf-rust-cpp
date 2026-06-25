use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Set meta information value of PDF-document
    pdf.set_meta_info("Author", "Aspose")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_set_meta_info.pdf")?;

    Ok(())
}
