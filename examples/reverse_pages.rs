use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Reverse the order of pages in PDF-document
    pdf.reverse_pages()?;

    // Save the modified PDF-document with a new filename
    pdf.save_as("sample_reverse_pages.pdf")?;

    Ok(())
}
