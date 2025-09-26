use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Remove annotations in page
    pdf.page_remove_annotations(1)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_remove_annotations.pdf")?;

    Ok(())
}
