use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Export from the previously opened PDF-document with AcroForm to FDF-document
    pdf.export_fdf("sample.fdf")?;

    Ok(())
}
