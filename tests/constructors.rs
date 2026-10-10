#![cfg(target_arch = "wasm32")]

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

mod common;

wasm_bindgen_test_configure!(run_in_browser);

fn assert_is_autonomous_element_constructor(value: &JsValue) {
    assert!(value.is_function());
    let prototype = Object::get_prototype_of(value);
    assert!(prototype.is_function());
    let name = Reflect::get(&prototype, &JsValue::from_str("name")).unwrap();
    assert_eq!(name, "HTMLElement");
}

fn assert_is_customized_built_in_constructor(value: &JsValue) {
    assert!(value.is_function());
    let prototype = Object::get_prototype_of(value);
    assert!(prototype.is_function());
    let name = Reflect::get(&prototype, &JsValue::from_str("name")).unwrap();
    assert_ne!(name, "HTMLElement");
    assert!(name.as_string().unwrap().contains("Element"))
}

pub mod autonomous_custom_elements {
    use super::assert_is_autonomous_element_constructor;
    use crate::common::constructors;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_create_constructor() {
        let constructor = constructors::create_autonomous_custom_element_constructor();
        assert_is_autonomous_element_constructor(&constructor);
    }
}

pub mod customized_built_in_custom_elements {
    use super::assert_is_customized_built_in_constructor;
    use crate::common::constructors;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_create_constructor() {
        let constructor = constructors::create_customized_built_in_element_constructor();
        assert_is_customized_built_in_constructor(&constructor);
    }
}

#[wasm_bindgen_test]
fn every_element_constructor_is_the_browser_global_it_names() {
    use custom_element::constructors::*;
    let table: [(fn() -> js_sys::Function, &str); 71] = [
        (html_anchor_element_constructor, "HTMLAnchorElement"),
        (html_area_element_constructor, "HTMLAreaElement"),
        (html_audio_element_constructor, "HTMLAudioElement"),
        (html_br_element_constructor, "HTMLBRElement"),
        (html_base_element_constructor, "HTMLBaseElement"),
        (html_body_element_constructor, "HTMLBodyElement"),
        (html_button_element_constructor, "HTMLButtonElement"),
        (html_canvas_element_constructor, "HTMLCanvasElement"),
        (html_d_list_element_constructor, "HTMLDListElement"),
        (html_data_element_constructor, "HTMLDataElement"),
        (html_data_list_element_constructor, "HTMLDataListElement"),
        (html_details_element_constructor, "HTMLDetailsElement"),
        (html_dialog_element_constructor, "HTMLDialogElement"),
        (html_directory_element_constructor, "HTMLDirectoryElement"),
        (html_div_element_constructor, "HTMLDivElement"),
        (html_element_constructor, "HTMLElement"),
        (html_embed_element_constructor, "HTMLEmbedElement"),
        (html_field_set_element_constructor, "HTMLFieldSetElement"),
        (html_font_element_constructor, "HTMLFontElement"),
        (html_form_element_constructor, "HTMLFormElement"),
        (html_frame_element_constructor, "HTMLFrameElement"),
        (html_frame_set_element_constructor, "HTMLFrameSetElement"),
        (html_hr_element_constructor, "HTMLHRElement"),
        (html_head_element_constructor, "HTMLHeadElement"),
        (html_heading_element_constructor, "HTMLHeadingElement"),
        (html_html_element_constructor, "HTMLHtmlElement"),
        (html_iframe_element_constructor, "HTMLIFrameElement"),
        (html_image_element_constructor, "HTMLImageElement"),
        (html_input_element_constructor, "HTMLInputElement"),
        (html_li_element_constructor, "HTMLLIElement"),
        (html_label_element_constructor, "HTMLLabelElement"),
        (html_legend_element_constructor, "HTMLLegendElement"),
        (html_link_element_constructor, "HTMLLinkElement"),
        (html_map_element_constructor, "HTMLMapElement"),
        (html_marquee_element_constructor, "HTMLMarqueeElement"),
        (html_media_element_constructor, "HTMLMediaElement"),
        (html_menu_element_constructor, "HTMLMenuElement"),
        (html_meta_element_constructor, "HTMLMetaElement"),
        (html_meter_element_constructor, "HTMLMeterElement"),
        (html_mod_element_constructor, "HTMLModElement"),
        (html_olist_element_constructor, "HTMLOListElement"),
        (html_object_element_constructor, "HTMLObjectElement"),
        (html_opt_group_element_constructor, "HTMLOptGroupElement"),
        (html_option_element_constructor, "HTMLOptionElement"),
        (html_output_element_constructor, "HTMLOutputElement"),
        (html_paragraph_element_constructor, "HTMLParagraphElement"),
        (html_param_element_constructor, "HTMLParamElement"),
        (html_picture_element_constructor, "HTMLPictureElement"),
        (html_pre_element_constructor, "HTMLPreElement"),
        (html_progress_element_constructor, "HTMLProgressElement"),
        (html_quote_element_constructor, "HTMLQuoteElement"),
        (html_script_element_constructor, "HTMLScriptElement"),
        (html_select_element_constructor, "HTMLSelectElement"),
        (html_slot_element_constructor, "HTMLSlotElement"),
        (html_source_element_constructor, "HTMLSourceElement"),
        (html_span_element_constructor, "HTMLSpanElement"),
        (html_style_element_constructor, "HTMLStyleElement"),
        (
            html_table_caption_element_constructor,
            "HTMLTableCaptionElement",
        ),
        (html_table_cell_element_constructor, "HTMLTableCellElement"),
        (html_table_col_element_constructor, "HTMLTableColElement"),
        (html_table_element_constructor, "HTMLTableElement"),
        (html_table_row_element_constructor, "HTMLTableRowElement"),
        (
            html_table_section_element_constructor,
            "HTMLTableSectionElement",
        ),
        (html_template_element_constructor, "HTMLTemplateElement"),
        (html_textarea_element_constructor, "HTMLTextAreaElement"),
        (html_time_element_constructor, "HTMLTimeElement"),
        (html_title_element_constructor, "HTMLTitleElement"),
        (html_track_element_constructor, "HTMLTrackElement"),
        (html_u_list_element_constructor, "HTMLUListElement"),
        (html_unknown_element_constructor, "HTMLUnknownElement"),
        (html_video_element_constructor, "HTMLVideoElement"),
    ];
    let missing: Vec<&str> = table
        .iter()
        .filter(|(constructor, name)| constructor().name() != *name)
        .map(|(_, name)| *name)
        .collect();
    assert!(
        missing.is_empty(),
        "constructors that do not match their global: {missing:?}"
    );
}
