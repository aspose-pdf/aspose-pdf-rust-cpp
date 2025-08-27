use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Replace text in PDF-document
    pdf.replace_text("PDF", "TXT")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_replace_text.pdf")?;

    Ok(())
}
