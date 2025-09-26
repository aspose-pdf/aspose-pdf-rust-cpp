use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Remove annotations from PDF-document
    pdf.remove_annotations()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_remove_annotations.pdf")?;

    Ok(())
}
