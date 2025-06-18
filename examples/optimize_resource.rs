use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Optimize resources of PDF-document
    pdf.optimize_resource()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_optimize_resource.pdf")?;

    Ok(())
}
