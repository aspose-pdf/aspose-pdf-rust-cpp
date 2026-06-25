use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Get a value indicating whether document is linearized
    if pdf.is_linearized()? {
        println!("The PDF-document is linearized.");
    } else {
        println!("The PDF-document is non-linearized.");
    }

    Ok(())
}
