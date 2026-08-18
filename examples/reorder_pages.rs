use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Reorder pages in PDF-document
    pdf.reorder_pages(&[2, 1])?;

    // Save the modified PDF-document with a new filename
    pdf.save_as("sample_reorder_pages.pdf")?;

    Ok(())
}
