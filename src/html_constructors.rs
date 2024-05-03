/// These are all the valid HTMLElement constructors, exported
/// for convenience. These can be used to created cutomized built-in
/// custom elements.
#[allow(missing_docs)]
pub mod constructors {
    use js_sys::Function;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        /// Constructor for the [`web_sys::HTMLAnchorElement`]
        #[wasm_bindgen(js_name = HTMLAnchorElement, js_namespace = window)]
        pub static HTML_ANCHOR_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLAreaElement`]
        #[wasm_bindgen(js_name = HTMLAreaElement, js_namespace = window)]
        pub static HTML_AREA_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLAudioElement`]
        #[wasm_bindgen(js_name = HTMLAudioElement, js_namespace = window)]
        pub static HTML_AUDIO_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLBRElement`]
        #[wasm_bindgen(js_name = HTMLBRElement, js_namespace = window)]
        pub static HTML_BR_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLBaseElement`]
        #[wasm_bindgen(js_name = HTMLBaseElement, js_namespace = window)]
        pub static HTML_BASE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLBodyElement`]
        #[wasm_bindgen(js_name = HTMLBodyElement, js_namespace = window)]
        pub static HTML_BODY_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLButtonElement`]
        #[wasm_bindgen(js_name = HTMLButtonElement, js_namespace = window)]
        pub static HTML_BUTTON_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLCanvasElement`]
        #[wasm_bindgen(js_name = HTMLCanvasElement, js_namespace = window)]
        pub static HTML_CANVAS_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDListElement`]
        #[wasm_bindgen(js_name = HTMLDListElement, js_namespace = window)]
        pub static HTML_D_LIST_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDataElement`]
        #[wasm_bindgen(js_name = HTMLDataElement, js_namespace = window)]
        pub static HTML_DATA_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDataListElement`]
        #[wasm_bindgen(js_name = HTMLDataListElement, js_namespace = window)]
        pub static HTML_DATA_LIST_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDetailsElement`]
        #[wasm_bindgen(js_name = HTMLDetailsElement, js_namespace = window)]
        pub static HTML_DETAILS_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDialogElement`]
        #[wasm_bindgen(js_name = HTMLDialogElement, js_namespace = window)]
        pub static HTML_DIALOG_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDirectoryElement`]
        #[wasm_bindgen(js_name = HTMLDirectoryElement, js_namespace = window)]
        pub static HTML_DIRECTORY_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDivElement`]
        #[wasm_bindgen(js_name = HTMLDivElement, js_namespace = window)]
        pub static HTML_DIV_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLDocument`]
        #[wasm_bindgen(js_name = HTMLDocument, js_namespace = window)]
        pub static HTML_DOCUMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLElement`]
        #[wasm_bindgen(js_name = HTMLElement, js_namespace = window)]
        pub static HTML_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLEmbedElement`]
        #[wasm_bindgen(js_name = HTMLEmbedElement, js_namespace = window)]
        pub static HTML_EMBED_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLFieldSetElement`]
        #[wasm_bindgen(js_name = HTMLFieldSetElement, js_namespace = window)]
        pub static HTML_FIELD_SET_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLFontElement`]
        #[wasm_bindgen(js_name = HTMLFontElement, js_namespace = window)]
        pub static HTML_FONT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLFormElement`]
        #[wasm_bindgen(js_name = HTMLFormElement, js_namespace = window)]
        pub static HTML_FORM_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLFrameElement`]
        #[wasm_bindgen(js_name = HTMLFrameElement, js_namespace = window)]
        pub static HTML_FRAME_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLFrameSetElement`]
        #[wasm_bindgen(js_name = HTMLFrameSetElement, js_namespace = window)]
        pub static HTML_FRAME_SET_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLHRElement`]
        #[wasm_bindgen(js_name = HTMLHRElement, js_namespace = window)]
        pub static HTML_HR_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLHeadElement`]
        #[wasm_bindgen(js_name = HTMLHeadElement, js_namespace = window)]
        pub static HTML_HEAD_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLHeadingElement`]
        #[wasm_bindgen(js_name = HTMLHeadingElement, js_namespace = window)]
        pub static HTML_HEADING_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLHtmlElement`]
        #[wasm_bindgen(js_name = HTMLHtmlElement, js_namespace = window)]
        pub static HTML_HTML_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLIFrameElement`]
        #[wasm_bindgen(js_name = HTMLIFrameElement, js_namespace = window)]
        pub static HTML_IFRAME_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLImageElement`]
        #[wasm_bindgen(js_name = HTMLImageElement, js_namespace = window)]
        pub static HTML_IMAGE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLInputElement`]
        #[wasm_bindgen(js_name = HTMLInputElement, js_namespace = window)]
        pub static HTML_INPUT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLLIElement`]
        #[wasm_bindgen(js_name = HTMLLIElement, js_namespace = window)]
        pub static HTML_LI_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLLabelElement`]
        #[wasm_bindgen(js_name = HTMLLabelElement, js_namespace = window)]
        pub static HTML_LABEL_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLLegendElement`]
        #[wasm_bindgen(js_name = HTMLLegendElement, js_namespace = window)]
        pub static HTML_LEGEND_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLLinkElement`]
        #[wasm_bindgen(js_name = HTMLLinkElement, js_namespace = window)]
        pub static HTML_LINK_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLMapElement`]
        #[wasm_bindgen(js_name = HTMLMapElement, js_namespace = window)]
        pub static HTML_MAP_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLMarqueeElement`]
        #[wasm_bindgen(js_name = HTMLMarqueeElement, js_namespace = window)]
        pub static HTML_MARQUEE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLMediaElement`]
        #[wasm_bindgen(js_name = HTMLMediaElement, js_namespace = window)]
        pub static HTML_MEDIA_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLMenuElement`]
        #[wasm_bindgen(js_name = HTMLMenuElement, js_namespace = window)]
        pub static HTML_MENU_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLMetaElement`]
        #[wasm_bindgen(js_name = HTMLMetaElement, js_namespace = window)]
        pub static HTML_META_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLMeterElement`]
        #[wasm_bindgen(js_name = HTMLMeterElement, js_namespace = window)]
        pub static HTML_METER_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLModElement`]
        #[wasm_bindgen(js_name = HTMLModElement, js_namespace = window)]
        pub static HTML_MOD_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLOListElement`]
        #[wasm_bindgen(js_name = HTMLOListElement, js_namespace = window)]
        pub static HTML_OLIST_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLObjectElement`]
        #[wasm_bindgen(js_name = HTMLObjectElement, js_namespace = window)]
        pub static HTML_OBJECT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLOptGroupElement`]
        #[wasm_bindgen(js_name = HTMLOptGroupElement, js_namespace = window)]
        pub static HTML_OPT_GROUP_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLOptionElement`]
        #[wasm_bindgen(js_name = HTMLOptionElement, js_namespace = window)]
        pub static HTML_OPTION_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLOutputElement`]
        #[wasm_bindgen(js_name = HTMLOutputElement, js_namespace = window)]
        pub static HTML_OUTPUT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLParagraphElement`]
        #[wasm_bindgen(js_name = HTMLParagraphElement, js_namespace = window)]
        pub static HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLParamElement`]
        #[wasm_bindgen(js_name = HTMLParamElement, js_namespace = window)]
        pub static HTML_PARAM_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLPictureElement`]
        #[wasm_bindgen(js_name = HTMLPictureElement, js_namespace = window)]
        pub static HTML_PICTURE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLPreElement`]
        #[wasm_bindgen(js_name = HTMLPreElement, js_namespace = window)]
        pub static HTML_PRE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLProgressElement`]
        #[wasm_bindgen(js_name = HTMLProgressElement, js_namespace = window)]
        pub static HTML_PROGRESS_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLQuoteElement`]
        #[wasm_bindgen(js_name = HTMLQuoteElement, js_namespace = window)]
        pub static HTML_QUOTE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLScriptElement`]
        #[wasm_bindgen(js_name = HTMLScriptElement, js_namespace = window)]
        pub static HTML_SCRIPT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLSelectElement`]
        #[wasm_bindgen(js_name = HTMLSelectElement, js_namespace = window)]
        pub static HTML_SELECT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLSlotElement`]
        #[wasm_bindgen(js_name = HTMLSlotElement, js_namespace = window)]
        pub static HTML_SLOT_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLSourceElement`]
        #[wasm_bindgen(js_name = HTMLSourceElement, js_namespace = window)]
        pub static HTML_SOURCE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLSpanElement`]
        #[wasm_bindgen(js_name = HTMLSpanElement, js_namespace = window)]
        pub static HTML_SPAN_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLStyleElement`]
        #[wasm_bindgen(js_name = HTMLStyleElement, js_namespace = window)]
        pub static HTML_STYLE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTableCaptionElement`]
        #[wasm_bindgen(js_name = HTMLTableCaptionElement, js_namespace = window)]
        pub static HTML_TABLE_CAPTION_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTableCellElement`]
        #[wasm_bindgen(js_name = HTMLTableCellElement, js_namespace = window)]
        pub static HTML_TABLE_CELL_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTableColElement`]
        #[wasm_bindgen(js_name = HTMLTableColElement, js_namespace = window)]
        pub static HTML_TABLE_COL_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTableElement`]
        #[wasm_bindgen(js_name = HTMLTableElement, js_namespace = window)]
        pub static HTML_TABLE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTableRowElement`]
        #[wasm_bindgen(js_name = HTMLTableRowElement, js_namespace = window)]
        pub static HTML_TABLE_ROW_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTableSectionElement`]
        #[wasm_bindgen(js_name = HTMLTableSectionElement, js_namespace = window)]
        pub static HTML_TABLE_SECTION_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTemplateElement`]
        #[wasm_bindgen(js_name = HTMLTemplateElement, js_namespace = window)]
        pub static HTML_TEMPLATE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTextAreaElement`]
        #[wasm_bindgen(js_name = HTMLTextAreaElement, js_namespace = window)]
        pub static HTML_TEXTAREA_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTimeElement`]
        #[wasm_bindgen(js_name = HTMLTimeElement, js_namespace = window)]
        pub static HTML_TIME_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTitleElement`]
        #[wasm_bindgen(js_name = HTMLTitleElement, js_namespace = window)]
        pub static HTML_TITLE_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLTrackElement`]
        #[wasm_bindgen(js_name = HTMLTrackElement, js_namespace = window)]
        pub static HTML_TRACK_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLUListElement`]
        #[wasm_bindgen(js_name = HTMLUListElement, js_namespace = window)]
        pub static HTML_U_LIST_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLUnknownElement`]
        #[wasm_bindgen(js_name = HTMLUnknownElement, js_namespace = window)]
        pub static HTML_UNKNOWN_ELEMENT_CONSTRUCTOR: Function;

        /// Constructor for the [`web_sys::HTMLVideoElement`]
        #[wasm_bindgen(js_name = HTMLVideoElement, js_namespace = window)]
        pub static HTML_VIDEO_ELEMENT_CONSTRUCTOR: Function;
    }
}
