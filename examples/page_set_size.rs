use asposepdf::{Document, PageSize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Set the size of a page in the PDF-document
    pdf.page_set_size(1, PageSize::A1)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_page1_set_size_A1.pdf")?;

    Ok(())
}
