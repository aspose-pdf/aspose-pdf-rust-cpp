use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the primary PDF-document
    let pdf = Document::open("sample1page.pdf")?;

    // Open another PDF-document to append
    let another_pdf = Document::open("sample.pdf")?;

    // Append specific pages (1 and 3) from another PDF-document
    pdf.append_pages(&another_pdf, "1,3")?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_append_pages.pdf")?;

    Ok(())
}
