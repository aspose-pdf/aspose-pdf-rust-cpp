use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Remove tables in page
    pdf.page_remove_tables(1)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_remove_tables.pdf")?;

    Ok(())
}
