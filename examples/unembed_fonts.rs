use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Unembed fonts a PDF-document
    pdf.unembed_fonts()?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_unembed_fonts.pdf")?;

    Ok(())
}
