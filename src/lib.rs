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
            ("crop", Box::new(|doc| doc.crop(10.5))),
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
            (
                "replace_font",
                Box::new(|doc| doc.replace_font("Helvetica", "Times")),
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
                "remove_text_headers",
                Box::new(|doc| doc.remove_text_headers()),
            ),
            (
                "remove_text_footers",
                Box::new(|doc| doc.remove_text_footers()),
            ),
            ("page_crop", Box::new(|doc| doc.page_crop(1, 0.0))),
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
                "page_replace_font",
                Box::new(|doc| doc.page_replace_font(1, "Courier", "Helvetica")),
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
            (
                "page_remove_text_headers",
                Box::new(|doc| doc.page_remove_text_headers(1)),
            ),
            (
                "page_remove_text_footers",
                Box::new(|doc| doc.page_remove_text_footers(1)),
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

        // Check initial encryption status
        assert!(!pdf.is_encrypted()?, "New PDF should not be encrypted");

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

        // Check encryption status after encrypt call
        assert!(
            pdf.is_encrypted()?,
            "PDF should be encrypted after calling encrypt()"
        );

        pdf.save_as(&filename)?;

        // Opening without password must fail
        let try_open = Document::open(&filename);
        assert!(
            try_open.is_err(),
            "Opening encrypted PDF without password must fail"
        );

        // Opening with password must succeed
        let pdf2 = Document::open_with_password(&filename, owner_pass)?;
        // Verify status of opened encrypted file
        assert!(
            pdf2.is_encrypted()?,
            "Opened encrypted PDF should report is_encrypted as true"
        );

        pdf2.decrypt()?;
        pdf2.save_as(&filename)?;

        // Now opening without password must succeed
        let pdf3 = Document::open(&filename)?;
        assert!(
            !pdf3.is_encrypted()?,
            "Final saved PDF should not be encrypted"
        );

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

    #[test]
    fn test_digital_signatures() -> Result<(), Box<dyn std::error::Error>> {
        // Hex strings for certificate and appearance image
        let cert_hex = "308204710201033082043706092a864886f70d010701a082042804820424308204203082023f06092a864886f70d010706a08202303082022c0201003082022506092a864886f70d010701301c060a2a864886f70d010c0106300e0408e2d8d11dbf8e30bb02020800808201f8e00f7570e687eeaa4f28c792e9a8d9eb82c3812f459f2a4a16bf0005987aca59b3c980f5d2272d6337dc7783aa4b4711c6683dffc2096a90a29440e913c5018227f62a11f2cdcf85ddcb9a830f6f9d9228932f5a3446469a4bca8e2008d65fbf7eddb95f3e98a4d2c835b5c2bb47b199ef2e55079b2f3e39c1bccec0005fa6301d332469f965a512a40749263b56e84af1c4a7920fb14f2ee74569176d4e5d8355ec3e5ce16d8ed46ad5d1ae470fa3b239b1c98e3fae7cf17b9e1faa12e3cdfdbf219b5e32fac64fa15491bf0b47f8ef15543c672565fecbe8c684284bc626146119e104c5ba5fe7195bd6dca5764890575074f580d7fa2939d4e0b87957405d51f893675143ada29c17d74dee9ddc8f1a5e49197bcbaebea21f668ed46635a7602c7757b169075fc054329c2c6566b29a09cd8906ce497a6e7fc1637aadfabe9f5e6db18be4da04cff94fe27256bcc29d4db2aa9318e7b474024d1f79033b0c4a55be6053e7ce61b15623fea26774dd978d56ebaac4e06c63f77ee86091352942b74a5c49b4bd2e2b973764df7230961da5cf64d8743554eb2cb942ab9a9b8e732261a45e17a2c1f13324313c515503f4cd8206f70efddcaf641e8be26a1789f8280a98098e6ed0efb2ebe47d220d0be7308bfe7f5c5b23f262aaaf107e080b1a94a07d39a8224831d88d95ce70d4f6145ca692452ccad1308201d906092a864886f70d010701a08201ca048201c6308201c2308201be060b2a864886f70d010c0a0102a082018630820182301c060a2a864886f70d010c0103300e040816153022c45f5b3c0202080004820160c02e00f91f57db175b75fcf420ac367fba5ede225d80b2893d88940d5d9c19f42733852e40abbda88677940cc1b50705d3d5b2ea9c3053ade754788667acbd3ee9d9677b48336e08d088b54393eba61bd3e33ef25cfa88facbf9dc0873c294c9d971c94117f9dcf5fb57cba3bf793c95f050ea06317c5324ad375995f8967a85994a514c294ce66917c11558ade41454344819af249f671cb150ba68f8acd0fdbe4ad19ebdc7a1af8ff8f6d601241931035a19a31028ec3c23feb160ccc81d713b11e29898262ed48773b869744ff1b9105510bc619452359588f77db59b9cc37378b1be3816a892d6e9afd91bb1644ce7af2f2f1755ed765891aa0611df921144d1f98c726a4fc31eca643bfa386a98ce876e14e860aa7ea3e5e2e7314ccbce898a686ebc4fb9bfeca903d7074344b2a34f8dc61db13d72d589183313b46f585e87c4d66ade92624d5d2e2d70755d2219ff0b4c89aacf35286d93417ebc9b8f3125302306092a864886f70d0109153116041441182549e4def6ba468dac951eb60e11a74ff8d430313021300906052b0e03021a05000414f154fff15c435fb4b1de9d394cedc13677b9617604089866783010a7004d02020800";
        let img_hex = "89504e470d0a1a0a0000000d49484452000000640000003208020000002557e9e9000000017352474200aece1ce90000000467414d410000b18f0bfc6105000000097048597300000ec300000ec301c76fa864000004fa494441546843ed9a6d4c5b5518805bcaa52b6dd7402d103f82db98133b36279b110503d9704520030bc10d0c4e5193f949cc1297f8c37f33f1c78c1f8b7338ad8355024c16672832e30fd1928cb1cd59198389cc3416909152a074fdc0b79e9323ccb65cf59edb73139f1fc07de1d793f73cbd3717f9e2e2a2ec7ff89180bf4b934030d0ef1cc017f491b6acfd3daf1759ca7ac7ecf89a32123e869d83a72b5b6be1074d92baabb6233f330fcde921d5cdfa657a6cefa97d2a5de8a1526d50e52969318bb05f929405a9aa6eabf729a78beb9253b639b7ef4946be6cc33df82fe8204959902ac7dcb9fb7624850c13e1ebf409e4abdc5ad3eee8fcf34fa8203d5990aa2383efafdd1a4cc9b98e4740fa84a941c969fdbb3b9ea2e74b628187546d3952b0da38b9a97c018f96c079f4b6269fdfc359cdc7aa8c15782a1c52da2c92aa7b1f588d47cbf16ba71e7decd6402808fb353275154f85434ab25eecdabf2c5591704dccc1d7ba4d3559fa756822209291f5d9a576cb70d3cda95a8ee27ada40af7b63daf6bb256fe191a04843169ca967be78c990155853e0c5a3489cff6e46e1d542b0344a0d1e098a04642df81720557067102d55886b76f5e44822ecd4c6f47bf048682420abb1fbc090f762ec54cd0cea87fbe4356bea9edc127e00a204ebb278a6eaaccdb74e69a4942a02d3b2fe51aadaaa3fa5942a02bbb2667db395adb53c5375b4fc9dbb0d77e11135d895057755577d0e3ea9aa5fdff0784e151ed18451599f9c6f691d6de693aa0daacd87761ec423cab028ebc7f19f60ad78a7cab28a5b854794614e16a40a9eecf8a78ac6634d34a8cb823581bb4a7cc1033ea99abe942a66aa087465b53b3ab71d2d2c3ebe0bf6058f62c2275509936903676e88992a024559600a0ed442c0d77badafa4c5bca22f9ea9bad02776aa08b46421539cd66fde9bb1b9903be7fe3eb62f9e7755a3dfaac44f15818a2c62cad4a0f4655eb9a370caf870426c5f70a7ee0c8eac98aa9ffb15e2a78a20bcaca5a6fcda2934ccc89d0111c897cb338e86840fce7e74ead73666534510585644530810b1f591b0af224be9525f177efba1b1fb00cba9220829abf9626b3453089d31ec6bcc3f447cc1a9ac6eab97abbd7c5275bcf2c3b8a48a20982c5890273e7f36100a16141b229a42dce48b7faa9ecb7ebe22bb0c8fe28490afc2deb61f8603a5d28576beb018e0c22f0ea2e176a4f67f7543edbf658efb1d5215e30042aa7a9ae78dea5cfbd367121589781a27843c86afe4ed83fa7add09ddefc9b9051d9e4602ed1798e2932aa52f055215775380f02f59c97e452b17c13b6cd0e8e5c1d4a80710520507f084c91af70388a0f2463ac667227f205570af00a93a647a138fe20dadd7f7ffd11753a922087f538aa8325658cdc7fc1eced6e4e33c7a3ce50d53a922d09205fc6b5fe8aeeae35d87ef4cc9c42336a0280b58ea0b4e169ec6849dbbaabf23c6bf1c9d1eb241bf822acff63dc9b2f4a89f7d009ba922d0dd2c44d90653576d073cd97d7d625e361e6bbfd84c15410c59407e66de8abec803206ba92288240b88ed0ba5eae59c57610df1883dc468d6527ac7ec252de670bf7627cb3270bf50aa72750f7e53ff259b071021de6621feda2febbc6fd48086902a78a886cf4d964d01626f16e2f2e495224ba95be1babf30c5e99a765e969d2c3b695a5f8c7fcd2af19105205faed9f0497c2dbff1e08e37d09c65e2260b40beb252d7329e2a423c6501e04b93a4be5d771bbe669b38cb921232d91fea07e55484a439cc0000000049454e44ae426082";

        let cert_bytes = hex::decode(cert_hex)?;
        let img_bytes = hex::decode(img_hex)?;
        let password = "Pa$$w0rd2023";

        let tmp_dir = std::env::temp_dir();

        // Sub-test 1: Standard PKCS7 Signature
        {
            let pdf = Document::new()?;
            pdf.page_add()?;

            let output_path = tmp_dir.join("pkcs7_standard.pdf");
            let output_str = output_path.to_str().unwrap();

            pdf.sign_pkcs7(
                1,
                &cert_bytes,
                password,
                100,
                100,
                60,
                100,
                "Standard PKCS7",
                "Contact Info",
                "Location Info",
                true,
                &img_bytes,
                output_str,
            )?;

            let pdf_sign = Document::open(output_str)?;
            assert!(pdf_sign.is_signed()?, "PDF should be signed (standard)");

            let output_path_unsign = tmp_dir.join("pkcs7_standard_without_sign.pdf");
            let output_unsign_str = output_path_unsign.to_str().unwrap();

            pdf_sign.remove_signs(output_unsign_str)?;

            let pdf_unsign = Document::open(output_unsign_str)?;
            assert!(
                !pdf_unsign.is_signed()?,
                "PDF should not be signed after removal"
            );
        }

        // Sub-test 2: PKCS7 Detached Signature
        {
            let pdf = Document::new()?;
            pdf.page_add()?;

            let output_path = tmp_dir.join("pkcs7_detached.pdf");
            let output_str = output_path.to_str().unwrap();

            pdf.sign_pkcs7_detached(
                1,
                &cert_bytes,
                password,
                100,
                100,
                200,
                100,
                "Detached PKCS7",
                "Contact Info",
                "Location Info",
                true,
                &img_bytes,
                output_str,
            )?;

            let pdf_sign = Document::open(output_str)?;
            assert!(pdf_sign.is_signed()?, "PDF should be signed (detached)");

            let output_path_unsign = tmp_dir.join("pkcs7_detached_without_sign.pdf");
            let output_unsign_str = output_path_unsign.to_str().unwrap();

            pdf_sign.remove_signs(output_unsign_str)?;

            let pdf_unsign = Document::open(output_unsign_str)?;
            assert!(
                !pdf_unsign.is_signed()?,
                "PDF should not be signed after removal (detached)"
            );
        }

        Ok(())
    }
}
