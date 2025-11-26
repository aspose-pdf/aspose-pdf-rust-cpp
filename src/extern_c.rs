use std::ffi::{c_char, c_double, c_int, c_void};
extern "C" {
    pub fn PDFDocument_Open(filename: *const c_char, error: *mut *const c_char) -> *const c_void;
    pub fn PDFDocument_New(error: *mut *const c_char) -> *const c_void;
    pub fn PDFDocument_Release(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_About(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    ) -> *const c_char;
    pub fn PDFDocument_set_License(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_Save_As(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Memory(
        pdfdocumentclass: *const c_void,
        buffer_out: *mut *mut u8,
        size_out: *mut c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Append(
        pdfdocumentclass: *const c_void,
        otherpdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_AppendPages(
        pdfdocumentclass: *const c_void,
        otherpdfdocumentclass: *const c_void,
        pagerange: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_ExtractText(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    ) -> *const c_char;
    pub fn PDFDocument_get_WordCount(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    ) -> c_int;
    pub fn PDFDocument_get_CharacterCount(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    ) -> c_int;
    pub fn PDFDocument_Optimize(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_OptimizeResource(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_OptimizeFileSize(
        pdfdocumentclass: *const c_void,
        imageQuality: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Repair(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_Grayscale(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_EmbedFonts(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_UnembedFonts(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_set_Background(
        pdfdocumentclass: *const c_void,
        r: c_int,
        g: c_int,
        b: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Rotate(
        pdfdocumentclass: *const c_void,
        rotation: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_ReplaceText(
        pdfdocumentclass: *const c_void,
        findText: *const c_char,
        replaceText: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_AddPageNum(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_AddTextHeader(
        pdfdocumentclass: *const c_void,
        header: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_AddTextFooter(
        pdfdocumentclass: *const c_void,
        footer: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_AddWatermark(
        pdfdocumentclass: *const c_void,
        text: *const c_char,
        fontName: *const c_char,
        fontSize: c_double,
        foregroundColor: *const c_char,
        xPosition: c_int,
        yPosition: c_int,
        rotation: c_int,
        isBackground: c_int,
        opacity: c_double,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Flatten(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_RemoveAnnotations(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_RemoveAttachments(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_RemoveBlankPages(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_RemoveBookmarks(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_RemoveHiddenText(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_RemoveImages(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_RemoveJavaScripts(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_RemoveTables(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_RemoveWatermarks(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_Page_get_Count(
        pdfdocumentclass: *const c_void,
        error: *mut *const c_char,
    ) -> c_int;
    pub fn PDFDocument_Page_get_WordCount(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    ) -> c_int;
    pub fn PDFDocument_Page_get_CharacterCount(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    ) -> c_int;
    pub fn PDFDocument_Page_is_Blank(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    ) -> c_int;
    pub fn PDFDocument_Page_to_Jpg(
        pdfdocumentclass: *const c_void,
        num: c_int,
        resolutionDPI: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_to_Png(
        pdfdocumentclass: *const c_void,
        num: c_int,
        resolutionDPI: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_to_Bmp(
        pdfdocumentclass: *const c_void,
        num: c_int,
        resolutionDPI: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_to_Tiff(
        pdfdocumentclass: *const c_void,
        num: c_int,
        resolutionDPI: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_to_Svg(
        pdfdocumentclass: *const c_void,
        num: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_to_Pdf(
        pdfdocumentclass: *const c_void,
        num: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_to_DICOM(
        pdfdocumentclass: *const c_void,
        num: c_int,
        resolutionDPI: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_DocX(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Doc(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_XlsX(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_PptX(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Xps(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Txt(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Epub(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_TeX(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Markdown(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Booklet(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_NUp(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        columns: c_int,
        rows: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_Tiff(
        pdfdocumentclass: *const c_void,
        resolutionDPI: c_int,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_DocXEnhanced(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Save_SvgZip(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Export_Fdf(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Export_Xfdf(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Export_Xml(
        pdfdocumentclass: *const c_void,
        filename: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_Add(pdfdocumentclass: *const c_void, error: *mut *const c_char);
    pub fn PDFDocument_Page_Insert(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_Delete(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_Grayscale(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_AddText(
        pdfdocumentclass: *const c_void,
        num: c_int,
        add_text: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_Rotate(
        pdfdocumentclass: *const c_void,
        num: c_int,
        rotation: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_set_Size(
        pdfdocumentclass: *const c_void,
        num: c_int,
        page_size: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_ReplaceText(
        pdfdocumentclass: *const c_void,
        num: c_int,
        findText: *const c_char,
        replaceText: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_AddPageNum(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_AddTextHeader(
        pdfdocumentclass: *const c_void,
        num: c_int,
        header: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_AddTextFooter(
        pdfdocumentclass: *const c_void,
        num: c_int,
        footer: *const c_char,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_AddWatermark(
        pdfdocumentclass: *const c_void,
        num: c_int,
        text: *const c_char,
        fontName: *const c_char,
        fontSize: c_double,
        foregroundColor: *const c_char,
        xPosition: c_int,
        yPosition: c_int,
        rotation: c_int,
        isBackground: c_int,
        opacity: c_double,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_RemoveAnnotations(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_RemoveHiddenText(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_RemoveImages(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_RemoveTables(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
    pub fn PDFDocument_Page_RemoveWatermarks(
        pdfdocumentclass: *const c_void,
        num: c_int,
        error: *mut *const c_char,
    );
}

extern "C" {
    pub fn c_free_string(str: *mut c_char);
    pub fn c_free_buffer(ptr: *mut c_void);
}
