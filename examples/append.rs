use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the primary PDF-document
    let pdf = Document::open("sample.pdf")?;

    // Open another PDF-document to append
    let another_pdf = Document::open("sample1page.pdf")?;

    // Append pages from another PDF-document
    pdf.append(&another_pdf)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_append.pdf")?;

    Ok(())
}
