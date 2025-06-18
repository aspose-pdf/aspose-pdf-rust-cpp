use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Convert and save the specified page as Tiff-image
    pdf.page_to_tiff(1, 100, "sample_page1.tiff")?;

    Ok(())
}
