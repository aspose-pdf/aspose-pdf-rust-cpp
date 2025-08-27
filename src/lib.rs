mod document;
mod document_gen_fn;
mod enums;
mod errors;
mod extern_c;
mod utils;
pub use crate::document::*;
//pub use crate::{document::*, enums::*, errors::*};

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn pdf_new_and_save() -> Result<(), Box<dyn Error>> {
        let pdf = Document::new()?;
        let pdf_name = format!(
            "{}/test_pdf_new_and_save.pdf",
            std::env::temp_dir().display()
        );
        pdf.save_as(&pdf_name)?;

        let metadata = std::fs::metadata(&pdf_name)?;
        assert_ne!(metadata.len(), 0);

        Ok(())
    }

    #[test]
    fn pdf_pages_operations() -> Result<(), Box<dyn Error>> {
        let pdf = Document::new()?;

        // Add page
        pdf.page_add()?;

        // Set page size
        pdf.page_set_size(1, PageSize::A1)?;

        // Insert page at position 1 (i.e. before page 1, shifting it to position 2)
        pdf.page_insert(1)?;

        // Delete first page
        pdf.page_delete(1)?;

        // Page count should be 1
        let page_count = pdf.page_count()?;
        assert_eq!(page_count, 1);

        Ok(())
    }

    #[test]
    fn pdf_extract_text() -> Result<(), Box<dyn std::error::Error>> {
        let pdf = Document::new()?;

        // Add a page
        pdf.page_add()?;

        // Add text to the first page
        let expected_text = "This is a test text for extraction";
        pdf.page_add_text(1, expected_text)?;

        // Save the document to apply the changes
        pdf.save()?;

        // Extract text from the document
        let extracted = pdf.extract_text()?;

        // Check that the extracted text is not empty
        assert!(!extracted.is_empty(), "Extracted text is empty");

        // Check that the expected substring is found in the extracted text
        assert!(
            extracted.contains("test text"),
            "Extracted text does not contain expected substring. Got: {}",
            extracted
        );

        Ok(())
    }

    #[test]
    fn pdf_organize_operations() -> Result<(), Box<dyn std::error::Error>> {
        let actions: Vec<(
            &str,
            Box<dyn Fn(&Document) -> Result<(), crate::errors::PdfError>>,
        )> = vec![
            ("optimize", Box::new(|doc| doc.optimize())),
            ("optimize_resource", Box::new(|doc| doc.optimize_resource())),
            ("grayscale", Box::new(|doc| doc.grayscale())),
            (
                "rotate",
                Box::new(|doc| doc.rotate(crate::enums::Rotation::On90)),
            ),
            (
                "set_background",
                Box::new(|doc| doc.set_background(255, 255, 200)),
            ),
            (
                "replace_text",
                Box::new(|doc| doc.replace_text("PDF", "TXT")),
            ),
            ("add_page_num", Box::new(|doc| doc.add_page_num())),
            (
                "add_text_header",
                Box::new(|doc| doc.add_text_header("HEADER")),
            ),
            (
                "add_text_footer",
                Box::new(|doc| doc.add_text_footer("FOOTER")),
            ),
            (
                "page_rotate",
                Box::new(|doc| doc.page_rotate(1, crate::enums::Rotation::On180)),
            ),
            (
                "page_set_size",
                Box::new(|doc| doc.page_set_size(1, crate::enums::PageSize::A3)),
            ),
            ("page_grayscale", Box::new(|doc| doc.page_grayscale(1))),
            (
                "page_add_text",
                Box::new(|doc| doc.page_add_text(1, "Test Text")),
            ),
            (
                "page_replace_text",
                Box::new(|doc| doc.page_replace_text(1, "PDF", "TXT")),
            ),
            (
                "page_add_page_num",
                Box::new(|doc| doc.page_add_page_num(1)),
            ),
            (
                "page_add_text_header",
                Box::new(|doc| doc.page_add_text_header(1, "HEADER")),
            ),
            (
                "page_add_text_footer",
                Box::new(|doc| doc.page_add_text_footer(1, "FOOTER")),
            ),
        ];

        for (name, action_fn) in actions {
            println!("Testing organize action: {}", name);

            let doc = Document::new()?;
            doc.page_add()?;
            action_fn(&doc)?;

            let out_path = format!("{}/{}_organized.pdf", std::env::temp_dir().display(), name);
            doc.save_as(&out_path)?;

            let metadata = std::fs::metadata(&out_path)?;
            assert_ne!(metadata.len(), 0, "Organized file {} is empty", out_path);
        }

        Ok(())
    }

    #[test]
    fn pdf_repair_operation() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = std::env::temp_dir();
        let file_path = tmp_dir.join("repair_input.pdf");

        // Step 1: Create and save a valid PDF
        {
            let doc = Document::new()?;
            doc.page_add()?;
            doc.save_as(file_path.to_str().unwrap())?;
        }

        // Step 2: Reopen and repair
        let reopened = Document::open(file_path.to_str().unwrap())?;
        reopened.repair()?; // Should succeed even on already-valid files
        Ok(())
    }

    #[test]
    fn pdf_core_operations() -> Result<(), Box<dyn std::error::Error>> {
        // Create a new PDF document
        let doc = Document::new()?;
        doc.page_add()?;

        // Save with new name
        let path = format!("{}/core_ops.pdf", std::env::temp_dir().display());
        doc.save_as(&path)?;

        // Check file was saved
        let metadata = std::fs::metadata(&path)?;
        assert_ne!(metadata.len(), 0);

        // Reopen saved file
        let reopened = Document::open(&path)?;
        reopened.save()?; // Save again to confirm

        Ok(())
    }

    #[test]
    fn pdf_convert_from_pdf() -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::metadata;

        let conversions: Vec<(
            &str,
            Box<dyn Fn(&Document, &str) -> Result<(), crate::PdfError>>,
        )> = vec![
            ("save_docx", Box::new(|doc, path| doc.save_docx(path))),
            (
                "save_docx_enhanced",
                Box::new(|doc, path| doc.save_docx_enhanced(path)),
            ),
            ("save_doc", Box::new(|doc, path| doc.save_doc(path))),
            ("save_xlsx", Box::new(|doc, path| doc.save_xlsx(path))),
            ("save_txt", Box::new(|doc, path| doc.save_txt(path))),
            ("save_pptx", Box::new(|doc, path| doc.save_pptx(path))),
            ("save_xps", Box::new(|doc, path| doc.save_xps(path))),
            ("save_tex", Box::new(|doc, path| doc.save_tex(path))),
            ("save_epub", Box::new(|doc, path| doc.save_epub(path))),
            (
                "save_markdown",
                Box::new(|doc, path| doc.save_markdown(path)),
            ),
            ("save_booklet", Box::new(|doc, path| doc.save_booklet(path))),
            ("save_n_up", Box::new(|doc, path| doc.save_n_up(path, 2, 2))),
            ("save_tiff", Box::new(|doc, path| doc.save_tiff(150, path))),
            ("export_fdf", Box::new(|doc, path| doc.export_fdf(path))),
            ("export_xfdf", Box::new(|doc, path| doc.export_xfdf(path))),
            ("export_xml", Box::new(|doc, path| doc.export_xml(path))),
            (
                "page_to_jpg",
                Box::new(|doc, path| doc.page_to_jpg(1, 150, path)),
            ),
            (
                "page_to_png",
                Box::new(|doc, path| doc.page_to_png(1, 150, path)),
            ),
            (
                "page_to_bmp",
                Box::new(|doc, path| doc.page_to_bmp(1, 150, path)),
            ),
            (
                "page_to_tiff",
                Box::new(|doc, path| doc.page_to_tiff(1, 150, path)),
            ),
            (
                "page_to_svg",
                Box::new(|doc, path| doc.page_to_svg(1, path)),
            ),
            (
                "page_to_pdf",
                Box::new(|doc, path| doc.page_to_pdf(1, path)),
            ),
            (
                "page_to_dicom",
                Box::new(|doc, path| doc.page_to_dicom(1, 150, path)),
            ),
        ];

        for (name, conv_fn) in conversions {
            let temp_dir = std::env::temp_dir();
            let output_path = temp_dir.join(format!("{}.out", name));

            let doc = Document::new()?;
            doc.page_add()?;
            doc.page_add_text(1, &format!("Test for {}", name))?;
            doc.save()?; // Persist content before converting

            conv_fn(&doc, output_path.to_str().unwrap())
                .map_err(|e| format!("{} failed: {:?}", name, e))?;

            let meta = metadata(&output_path)?;
            assert_ne!(meta.len(), 0, "{} produced empty file", name);
        }

        Ok(())
    }

    #[test]
    fn test_page_operations() -> Result<(), Box<dyn std::error::Error>> {
        let doc = Document::new()?; // Create a new document

        // Initially, page count should be zero
        assert_eq!(doc.page_count()?, 0);

        // Add a new blank page
        doc.page_add()?;
        assert_eq!(doc.page_count()?, 1);

        // Insert a page at position 1 (1-based indexing)
        doc.page_insert(1)?;
        assert_eq!(doc.page_count()?, 2);

        // Add some text to the first page
        doc.page_add_text(1, "Hello, Page 1!")?;

        // Rotate first page by 90 degrees
        doc.page_rotate(1, Rotation::On90)?;

        // Set size of second page
        doc.page_set_size(2, PageSize::PageLetter)?;

        // Convert second page to grayscale
        doc.page_grayscale(2)?;

        // Delete the first page
        doc.page_delete(1)?;
        assert_eq!(doc.page_count()?, 1);

        // Check that only one page remains
        assert_eq!(doc.page_count()?, 1);

        Ok(())
    }
}
