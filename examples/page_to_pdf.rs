use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Convert and save the specified page as PDF-document
    pdf.page_to_pdf(1, "sample_page1.pdf")?;

    Ok(())
}
