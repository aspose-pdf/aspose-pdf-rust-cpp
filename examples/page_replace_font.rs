use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Replace font in page
    pdf.page_replace_font(1, "Times-BoldItalic", "Helvetica-Bold")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_replace_font.pdf")?;

    Ok(())
}
