use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Add page number to a PDF-document
    pdf.add_page_num()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_add_page_num.pdf")?;

    Ok(())
}
