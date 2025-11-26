use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Add watermark to PDF-document
    pdf.add_watermark(
        "WATERMARK",
        "Arial",
        16.0,
        "#010101",
        100,
        100,
        45,
        true,
        0.5,
    )?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_add_watermark.pdf")?;

    Ok(())
}
