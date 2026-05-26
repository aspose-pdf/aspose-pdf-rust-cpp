use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Remove PDF/UA compliance from a PDF-document
    pdf.remove_pdfua_compliance()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_remove_pdfua_compliance.pdf")?;

    Ok(())
}
