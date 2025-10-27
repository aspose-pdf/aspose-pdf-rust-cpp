use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document named "sample.pdf"
    let pdf_split = Document::open("sample.pdf")?;

    // Split the PDF-document into two new PDF-documents
    let (left, right) = Document::split_at_page(&pdf_split, 2)?;

    // Save each split part as a separate PDF-document
    left.save_as("sample_split_at_page_left.pdf")?;
    right.save_as("sample_split_at_page_right.pdf")?;

    Ok(())
}
