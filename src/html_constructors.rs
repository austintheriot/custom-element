/// Constructors for every HTML element interface, for creating customized
/// built-in custom elements.
///
/// Each function returns the browser global of the same name, for example
/// [`html_paragraph_element_constructor`](constructors::html_paragraph_element_constructor) returns `window.HTMLParagraphElement`.
/// A function returns `undefined` cast to [`js_sys::Function`] when the
/// browser does not define that global.
pub mod constructors {
    macro_rules! element_constructors {
        ($($global:ident => $function:ident,)*) => {
            #[allow(non_upper_case_globals)]
            mod globals {
                use js_sys::Function;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    $(
                        #[wasm_bindgen(thread_local_v2, js_namespace = window)]
                        pub static $global: Function;
                    )*
                }
            }

            $(
                #[doc = concat!("Returns the `window.", stringify!($global), "` constructor.")]
                pub fn $function() -> js_sys::Function {
                    globals::$global.with(js_sys::Function::clone)
                }
            )*
        };
    }

    element_constructors! {
        HTMLAnchorElement => html_anchor_element_constructor,
        HTMLAreaElement => html_area_element_constructor,
        HTMLAudioElement => html_audio_element_constructor,
        HTMLBRElement => html_br_element_constructor,
        HTMLBaseElement => html_base_element_constructor,
        HTMLBodyElement => html_body_element_constructor,
        HTMLButtonElement => html_button_element_constructor,
        HTMLCanvasElement => html_canvas_element_constructor,
        HTMLDListElement => html_d_list_element_constructor,
        HTMLDataElement => html_data_element_constructor,
        HTMLDataListElement => html_data_list_element_constructor,
        HTMLDetailsElement => html_details_element_constructor,
        HTMLDialogElement => html_dialog_element_constructor,
        HTMLDirectoryElement => html_directory_element_constructor,
        HTMLDivElement => html_div_element_constructor,
        HTMLElement => html_element_constructor,
        HTMLEmbedElement => html_embed_element_constructor,
        HTMLFieldSetElement => html_field_set_element_constructor,
        HTMLFontElement => html_font_element_constructor,
        HTMLFormElement => html_form_element_constructor,
        HTMLFrameElement => html_frame_element_constructor,
        HTMLFrameSetElement => html_frame_set_element_constructor,
        HTMLHRElement => html_hr_element_constructor,
        HTMLHeadElement => html_head_element_constructor,
        HTMLHeadingElement => html_heading_element_constructor,
        HTMLHtmlElement => html_html_element_constructor,
        HTMLIFrameElement => html_iframe_element_constructor,
        HTMLImageElement => html_image_element_constructor,
        HTMLInputElement => html_input_element_constructor,
        HTMLLIElement => html_li_element_constructor,
        HTMLLabelElement => html_label_element_constructor,
        HTMLLegendElement => html_legend_element_constructor,
        HTMLLinkElement => html_link_element_constructor,
        HTMLMapElement => html_map_element_constructor,
        HTMLMarqueeElement => html_marquee_element_constructor,
        HTMLMediaElement => html_media_element_constructor,
        HTMLMenuElement => html_menu_element_constructor,
        HTMLMetaElement => html_meta_element_constructor,
        HTMLMeterElement => html_meter_element_constructor,
        HTMLModElement => html_mod_element_constructor,
        HTMLOListElement => html_olist_element_constructor,
        HTMLObjectElement => html_object_element_constructor,
        HTMLOptGroupElement => html_opt_group_element_constructor,
        HTMLOptionElement => html_option_element_constructor,
        HTMLOutputElement => html_output_element_constructor,
        HTMLParagraphElement => html_paragraph_element_constructor,
        HTMLParamElement => html_param_element_constructor,
        HTMLPictureElement => html_picture_element_constructor,
        HTMLPreElement => html_pre_element_constructor,
        HTMLProgressElement => html_progress_element_constructor,
        HTMLQuoteElement => html_quote_element_constructor,
        HTMLScriptElement => html_script_element_constructor,
        HTMLSelectElement => html_select_element_constructor,
        HTMLSlotElement => html_slot_element_constructor,
        HTMLSourceElement => html_source_element_constructor,
        HTMLSpanElement => html_span_element_constructor,
        HTMLStyleElement => html_style_element_constructor,
        HTMLTableCaptionElement => html_table_caption_element_constructor,
        HTMLTableCellElement => html_table_cell_element_constructor,
        HTMLTableColElement => html_table_col_element_constructor,
        HTMLTableElement => html_table_element_constructor,
        HTMLTableRowElement => html_table_row_element_constructor,
        HTMLTableSectionElement => html_table_section_element_constructor,
        HTMLTemplateElement => html_template_element_constructor,
        HTMLTextAreaElement => html_textarea_element_constructor,
        HTMLTimeElement => html_time_element_constructor,
        HTMLTitleElement => html_title_element_constructor,
        HTMLTrackElement => html_track_element_constructor,
        HTMLUListElement => html_u_list_element_constructor,
        HTMLUnknownElement => html_unknown_element_constructor,
        HTMLVideoElement => html_video_element_constructor,
    }
}
