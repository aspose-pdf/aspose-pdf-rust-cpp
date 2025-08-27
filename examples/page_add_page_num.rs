use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Add page number on page
    pdf.page_add_page_num(1)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_add_page_num.pdf")?;

    Ok(())
}
