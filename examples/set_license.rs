use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Set license with filename
    pdf.set_license("Aspose.PDF.RustViaCPP.lic")?;

    // Now you can work with the licensed PDF document
    // ...

    Ok(())
}
