mod document;
mod document_gen_fn;
mod enums;
mod errors;
mod extern_c;
mod permissions;
mod product_info;
mod utils;

pub use document::Document;
pub use enums::{CryptoAlgorithm, PageSize, Rotation};
pub use errors::PdfError;
pub use permissions::Permissions;
pub use product_info::ProductInfo;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pdf_about() -> Result<(), Box<dyn std::error::Error>> {
        let pdf = Document::new()?;
        let info = pdf.about()?;
        assert!(!info.product.is_empty(), "product is empty");
        assert!(!info.family.is_empty(), "family is empty");
        assert!(!info.version.is_empty(), "version is empty");
        assert!(!info.release_date.is_empty(), "release_date is empty");
        assert!(!info.producer.is_empty(), "producer is empty");
        assert!(
            matches!(info.is_licensed, true | false),
            "is_licensed is not a boolean"
        );
        Ok(())
    }

    #[test]
    fn pdf_new_and_save() -> Result<(), Box<dyn std::error::Error>> {
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
    fn pdf_append() -> Result<(), Box<dyn std::error::Error>> {
        // Create the first PDF-document
        let pdf1 = Document::new()?;
        for _ in 0..2 {
            pdf1.page_add()?;
        }

        // Verify the first document has 2 pages
        let page_count1 = pdf1.page_count()?;
        assert_eq!(page_count1, 2);

        // Create the second PDF-document
        let pdf2 = Document::new()?;
        for _ in 0..2 {
            pdf2.page_add()?;
        }

        // Verify the second document has 2 pages
        let page_count2 = pdf2.page_count()?;
        assert_eq!(page_count2, 2);

        // Append pdf2 into pdf1
        pdf1.append(&pdf2)?;

        // Verify the first document now has 4 pages
        let page_count1_after_append = pdf1.page_count()?;
        assert_eq!(page_count1_after_append, 4);

        Ok(())
    }

    #[test]
    fn pdf_append_pages() -> Result<(), Box<dyn std::error::Error>> {
        // Create a base document with exactly 4 pages
        let pdf4pages = Document::new()?;
        for _ in 0..4 {
            pdf4pages.page_add()?;
        }

        let page_count = pdf4pages.page_count()?;
        assert_eq!(page_count, 4, "Expected 4 pages in base document");

        // Define test cases for different page ranges
        let test_cases = vec![
            ("EmptyRangeMeansAll", "", 4),
            ("DashMeansAll", "-", 4),
            ("FirstThreePages", "-3", 3),
            ("SecondToEnd", "2-", 3),
            ("SpecificPages134", "1,3,4", 3),
            ("OnlyPage2", "2", 1),
            ("Range2To3", "2-3", 2),
            ("NonSequential", "1,2,4", 3),
            ("AllPagesExplicit", "1,2,3,4", 4),
        ];

        for (name, pagerange, want_pages) in test_cases {
            // Running subtest
            let test_doc = Document::new()?;
            test_doc.append_pages(&pdf4pages, pagerange)?;

            let count = test_doc.page_count()?;
            assert_eq!(
                count, want_pages,
                "Subtest '{}': expected {} pages, got {}",
                name, want_pages, count
            );
        }

        Ok(())
    }

    #[test]
    fn pdf_merge_documents() -> Result<(), Box<dyn std::error::Error>> {
        // Create the first PDF-document
        let pdf1 = Document::new()?;
        pdf1.page_add()?;

        // Create the second PDF-document
        let pdf2 = Document::new()?;
        pdf2.page_add()?;

        // Create the third PDF-document
        let pdf3 = Document::new()?;
        pdf3.page_add()?;
        pdf3.page_add()?; // Add second page

        // Merge all three documents
        let merged = Document::merge_documents(&[&pdf1, &pdf2, &pdf3])?;

        // Check the page count in the merged document (should be 4 pages)
        let page_count = merged.page_count()?;
        assert_eq!(page_count, 4);

        Ok(())
    }

    #[test]
    fn pdf_split_document() -> Result<(), Box<dyn std::error::Error>> {
        let pdf = Document::new()?;
        for _ in 0..4 {
            pdf.page_add()?;
        }

        let pdfs = Document::split_document(&pdf, "1-2;3;4-")?;

        assert_eq!(pdfs.len(), 3);

        let count1 = pdfs[0].page_count()?;
        assert_eq!(count1, 2);

        let count2 = pdfs[1].page_count()?;
        assert_eq!(count2, 1);

        let count3 = pdfs[2].page_count()?;
        assert_eq!(count3, 1);

        Ok(())
    }

    #[test]
    fn pdf_split() -> Result<(), Box<dyn std::error::Error>> {
        let pdf = Document::new()?;
        for _ in 0..4 {
            pdf.page_add()?;
        }

        let pdfs = pdf.split("1-2;3;4-")?;

        assert_eq!(pdfs.len(), 3);

        let count1 = pdfs[0].page_count()?;
        assert_eq!(count1, 2);

        let count2 = pdfs[1].page_count()?;
        assert_eq!(count2, 1);

        let count3 = pdfs[2].page_count()?;
        assert_eq!(count3, 1);

        Ok(())
    }

    #[test]
    fn pdf_split_at_page() -> Result<(), Box<dyn std::error::Error>> {
        let pdf = Document::new()?;
        for _ in 0..4 {
            pdf.page_add()?;
        }

        let (left, right) = Document::split_at_page(&pdf, 2)?;

        let count_left = left.page_count()?;
        assert_eq!(count_left, 2);

        let count_right = right.page_count()?;
        assert_eq!(count_right, 2);

        Ok(())
    }

    #[test]
    fn pdf_split_at() -> Result<(), Box<dyn std::error::Error>> {
        let pdf = Document::new()?;
        for _ in 0..4 {
            pdf.page_add()?;
        }

        let (left, right) = pdf.split_at(2)?;

        let count_left = left.page_count()?;
        assert_eq!(count_left, 2);

        let count_right = right.page_count()?;
        assert_eq!(count_right, 2);

        Ok(())
    }

    #[test]
    fn pdf_pages_operations() -> Result<(), Box<dyn std::error::Error>> {
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
            (
                "optimize_file_size",
                Box::new(|doc| doc.optimize_file_size(20)),
            ),
            ("grayscale", Box::new(|doc| doc.grayscale())),
            ("flatten", Box::new(|doc| doc.flatten())),
            ("embed_fonts", Box::new(|doc| doc.embed_fonts())),
            ("unembed_fonts", Box::new(|doc| doc.unembed_fonts())),
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
                "add_watermark",
                Box::new(|doc| {
                    doc.add_watermark(
                        "WATERMARK",
                        "Arial",
                        16.0,
                        "#010101",
                        100,
                        100,
                        45,
                        true,
                        0.5,
                    )
                }),
            ),
            (
                "remove_annotations",
                Box::new(|doc| doc.remove_annotations()),
            ),
            (
                "remove_attachments",
                Box::new(|doc| doc.remove_attachments()),
            ),
            (
                "remove_blank_pages",
                Box::new(|doc| doc.remove_blank_pages()),
            ),
            ("remove_bookmarks", Box::new(|doc| doc.remove_bookmarks())),
            (
                "remove_hidden_text",
                Box::new(|doc| doc.remove_hidden_text()),
            ),
            ("remove_images", Box::new(|doc| doc.remove_images())),
            (
                "remove_javascripts",
                Box::new(|doc| doc.remove_javascripts()),
            ),
            ("remove_tables", Box::new(|doc| doc.remove_tables())),
            ("remove_watermarks", Box::new(|doc| doc.remove_watermarks())),
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
            (
                "page_add_watermark",
                Box::new(|doc| {
                    doc.page_add_watermark(
                        1,
                        "WATERMARK",
                        "Arial",
                        16.0,
                        "#010101",
                        100,
                        100,
                        45,
                        true,
                        0.5,
                    )
                }),
            ),
            (
                "page_remove_annotations",
                Box::new(|doc| doc.page_remove_annotations(1)),
            ),
            (
                "page_remove_hidden_text",
                Box::new(|doc| doc.page_remove_hidden_text(1)),
            ),
            (
                "page_remove_images",
                Box::new(|doc| doc.page_remove_images(1)),
            ),
            (
                "page_remove_tables",
                Box::new(|doc| doc.page_remove_tables(1)),
            ),
            (
                "page_remove_watermarks",
                Box::new(|doc| doc.page_remove_watermarks(1)),
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
    fn pdf_core_stat_operations() -> Result<(), Box<dyn std::error::Error>> {
        // Create a new PDF-document
        let doc = Document::new()?;

        // Add a new blank page
        doc.page_add()?;

        // Return word count in PDF-document
        assert_eq!(doc.word_count()?, 0);

        // Return character count in PDF-document
        assert_eq!(doc.character_count()?, 0);

        // Return word count on specified page in PDF-document
        assert_eq!(doc.page_word_count(1)?, 0);

        // Return character count on specified page in PDF-document
        assert_eq!(doc.page_character_count(1)?, 0);

        // Return page is blank in PDF-document
        assert_eq!(doc.page_is_blank(1)?, true);

        Ok(())
    }

    #[test]
    fn pdf_core_bytes() -> Result<(), Box<dyn std::error::Error>> {
        // Create a new PDF-document
        let doc = Document::new()?;

        // Return the contents of the PDF-document as a byte vector
        let data = doc.bytes()?;

        // Assert that the byte vector is not empty
        assert_ne!(data.len(), 0, "Expected non-empty PDF byte data");

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
            ("save_svg_zip", Box::new(|doc, path| doc.save_svg_zip(path))),
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
    fn pdf_page_operations() -> Result<(), Box<dyn std::error::Error>> {
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

    #[test]
    fn pdf_encrypt_decrypt() -> Result<(), Box<dyn std::error::Error>> {
        // Path to temporary file
        let filename = format!("{}/pdf_encrypt_decrypt.pdf", std::env::temp_dir().display());

        // Create new PDF
        let pdf = Document::new()?;
        pdf.save_as(&filename)?;

        // Encryption settings
        let user_pass = "user123";
        let owner_pass = "owner123";
        let permissions =
            Permissions::PRINT_DOCUMENT | Permissions::MODIFY_CONTENT | Permissions::FILL_FORM;

        // Encrypt
        pdf.encrypt(
            user_pass,
            owner_pass,
            permissions,
            CryptoAlgorithm::AESx128,
            true,
        )?;

        pdf.save_as(&filename)?;

        // Opening without password must fail
        let try_open = Document::open(&filename);
        assert!(
            try_open.is_err(),
            "Opening encrypted PDF without password must fail"
        );

        // Opening with password must succeed
        let pdf2 = Document::open_with_password(&filename, owner_pass)?;
        pdf2.decrypt()?;
        pdf2.save_as(&filename)?;

        // Now opening without password must succeed
        Document::open(&filename)?;

        Ok(())
    }

    #[test]
    fn pdf_permissions() -> Result<(), Box<dyn std::error::Error>> {
        // Path to temporary file
        let filename = format!("{}/pdf_permissions.pdf", std::env::temp_dir().display());

        let pdf = Document::new()?;
        pdf.save_as(&filename)?;

        let user_pass = "user123";
        let owner_pass = "owner123";

        let expected = Permissions::EXTRACT_CONTENT
            | Permissions::MODIFY_TEXT_ANNOTATIONS
            | Permissions::PRINTING_QUALITY;

        //Set permissions
        pdf.set_permissions(user_pass, owner_pass, expected)?;

        pdf.save_as(&filename)?;

        // Open with password
        let pdf2 = Document::open_with_password(&filename, user_pass)?;

        //Get permissions
        let perms = pdf2.get_permissions()?;

        assert_eq!(perms, expected);

        Ok(())
    }

    #[test]
    fn pdf_open_with_password_wrong_pass() -> Result<(), Box<dyn std::error::Error>> {
        // Path to temporary file
        let filename = format!("{}/pdf_permissions.pdf", std::env::temp_dir().display());

        let user_pass = "user123";
        let owner_pass = "owner123";

        let pdf = Document::new()?;

        pdf.encrypt(
            user_pass,
            owner_pass,
            Permissions::PRINT_DOCUMENT,
            CryptoAlgorithm::AESx128,
            false,
        )?;

        pdf.save_as(&filename)?;

        let bad = Document::open_with_password(&filename, "badpass");

        assert!(
            bad.is_err(),
            "open_with_password() must fail on wrong password"
        );

        Ok(())
    }

    #[test]
    fn pdf_permissions_combination() -> Result<(), Box<dyn std::error::Error>> {
        let all = Permissions::PRINT_DOCUMENT
            | Permissions::MODIFY_CONTENT
            | Permissions::EXTRACT_CONTENT
            | Permissions::MODIFY_TEXT_ANNOTATIONS
            | Permissions::FILL_FORM
            | Permissions::EXTRACT_CONTENT_WITH_DISABILITIES
            | Permissions::ASSEMBLE_DOCUMENT
            | Permissions::PRINTING_QUALITY;
        let user_pass = "user123";
        let owner_pass = "owner123";

        let pdf = Document::new()?;

        pdf.set_permissions(user_pass, owner_pass, all)?;

        let got = pdf.get_permissions()?;

        assert_eq!(got, all);

        Ok(())
    }

    #[test]
    fn pdf_encrypt_algorithms() -> Result<(), Box<dyn std::error::Error>> {
        // Path to temporary file
        let filename = format!("{}/pdf_cryptoall.pdf", std::env::temp_dir().display());

        let user_pass = "user123";
        let owner_pass = "owner123";

        let tests = [
            (CryptoAlgorithm::RC4x40, false),
            (CryptoAlgorithm::RC4x128, false),
            (CryptoAlgorithm::AESx128, false),
            (CryptoAlgorithm::AESx128, true),
            (CryptoAlgorithm::AESx256, false),
            (CryptoAlgorithm::AESx256, true),
        ];

        let base = Document::new()?;
        base.save_as(&filename)?;

        for (algorithm, use_pdf20) in tests {
            let pdf = Document::open(&filename)?;

            // Encrypt
            pdf.encrypt(
                user_pass,
                owner_pass,
                Permissions::PRINT_DOCUMENT,
                algorithm,
                use_pdf20,
            )?;

            let filename_out = format!(
                "{}/pdf_cryptoall_{:?}_{:?}.pdf",
                std::env::temp_dir().display(),
                algorithm,
                use_pdf20
            );

            pdf.save_as(&filename_out)?;

            // Open with password
            Document::open_with_password(&filename_out, user_pass)?;
        }

        Ok(())
    }
}
