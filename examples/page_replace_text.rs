use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Replace text on page
    pdf.page_replace_text(1, "PDF", "TXT")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_replace_text.pdf")?;

    Ok(())
}
