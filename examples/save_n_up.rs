use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Convert and save the previously opened PDF-document as N-Up PDF-document
    pdf.save_n_up("sample_n_up.pdf", 2, 2)?;

    Ok(())
}
