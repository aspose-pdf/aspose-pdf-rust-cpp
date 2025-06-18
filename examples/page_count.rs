use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Return page count in PDF-document
    let count = pdf.page_count()?;

    // Print the page count
    println!("Count: {}", count);

    Ok(())
}
