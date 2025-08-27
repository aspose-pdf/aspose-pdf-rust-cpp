use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Add text in Footer of a PDF-document
    pdf.add_text_footer("FOOTER")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_add_text_footer.pdf")?;

    Ok(())
}
