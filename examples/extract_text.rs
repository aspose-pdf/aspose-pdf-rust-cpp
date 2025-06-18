use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Return the PDF-document contents as plain text
    let txt = pdf.extract_text()?;

    // Print extracted text
    println!("Extracted text:\n{}", txt);

    Ok(())
}
