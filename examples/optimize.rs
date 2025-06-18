use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Optimize PDF-document content
    pdf.optimize()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_optimize.pdf")?;

    Ok(())
}
