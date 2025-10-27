use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a PDF-document from file
    let pdf = Document::open("sample.pdf")?;

    // Specify the page number (1-based index)
    let page_number = 1;

    // Return page is blank in PDF-document
    let is_blank = pdf.page_is_blank(page_number)?;

    // Print if the specified page is blank
    println!("Is page {} blank? {}", page_number, is_blank);

    Ok(())
}
