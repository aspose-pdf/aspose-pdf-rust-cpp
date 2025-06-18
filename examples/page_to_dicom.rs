use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Convert and save the specified page as DICOM-image
    pdf.page_to_dicom(1, 100, "sample_page1.dcm")?;

    Ok(())
}
