use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Optimize size of PDF-document with image compression quality
    pdf.optimize_file_size(50)?;

    // Save the previously opened PDF-document with new filename
    pdf.save_as("sample_optimize_file_size.pdf")?;

    Ok(())
}
