use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document with filename
    let pdf = Document::open("sample.pdf")?;

    // Get meta information value of PDF-document
    let author = pdf.get_meta_info("Author")?;

    // Print the result
    println!("Author: {}", author);

    Ok(())
}
