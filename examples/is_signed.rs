use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document named "sample_with_sign.pdf"
    let pdf = Document::open("sample_with_sign.pdf")?;

    // Get signed status of PDF-document
    if pdf.is_signed()? {
        println!("The document is signed.");
    }

    Ok(())
}
