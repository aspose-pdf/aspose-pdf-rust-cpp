use asposepdf::{Document, PdfFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Validate a PDF-document for compliance with the PDF format
    let (ok, log) = pdf.validate(PdfFormat::PDF_A_2A)?;

    // Print validation result and full log
    println!("Validate PDF/A result: {}", ok);
    println!("Validate PDF/A log:\n{}", log);

    Ok(())
}
