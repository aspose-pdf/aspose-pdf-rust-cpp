use asposepdf::Document;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new PDF-document
    let pdf1 = Document::new()?;

    // Open a PDF-document named "sample.pdf"
    let pdf2 = Document::open("sample.pdf")?;

    // Create a new PDF-document by merging the provided PDF-documents
    let pdf_merged = Document::merge_documents(&[&pdf1, &pdf2])?;

    // Save the previously opened PDF-document with new filename
    pdf_merged.save_as("sample_merge_documents.pdf")?;

    Ok(())
}
