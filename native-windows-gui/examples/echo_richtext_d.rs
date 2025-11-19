/*!
    Small example that shows how to append text to a rich text box with styling
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::fs;

#[derive(Default, NwgUi)]
pub struct EchoApp {
    #[nwg_control(size: (1000, 420), position: (300, 300), title: "Echo", accept_files: true)]
    #[nwg_events( 
        OnInit: [EchoApp::init_text],
        OnWindowClose: [nwg::stop_thread_dispatch()], 
        OnFileDrop: [EchoApp::load_text(SELF, EVT_DATA)], 
        OnKeyEnter: [EchoApp::submit], 
    )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    font: nwg::Font,

    #[nwg_control(font: Some(&data.font), readonly: true, flags: "VSCROLL|HSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP|SAVE_SELECTION")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0, col_span: 7, row_span: 4)]
    rich_text_box: nwg::RichTextBox,

    #[nwg_control(focus: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 4, col_span: 7)]
    text_input: nwg::TextInput,

    #[nwg_control(text: "Clear")]
    #[nwg_layout_item(layout: grid, col: 7, row: 0, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::clear] )]
    b1: nwg::Button,
    #[nwg_control(text: "Underline")]
    #[nwg_layout_item(layout: grid, col: 7, row: 1, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::underline] )]
    b2: nwg::Button,
    #[nwg_control(text: "Bullet")]
    #[nwg_layout_item(layout: grid, col: 7, row: 2, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::bullet] )]
    b3: nwg::Button,
    #[nwg_control(text: "Header")]
    #[nwg_layout_item(layout: grid, col: 7, row: 3, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::header] )]
    b4: nwg::Button,
    #[nwg_control(text: "Submit")]
    #[nwg_layout_item(layout: grid, col: 7, row: 4, col_span: 2)]
    #[nwg_events( OnButtonClick: [EchoApp::submit] )]
    b5: nwg::Button,
}

const UNDERLINE: nwg::CharFormat = nwg::CharFormat{
    effects: Some(nwg::CharEffects::UNDERLINE),
    height: None,
    y_offset: None,
    text_color: None,
    font_face_name: None,
    underline_type: None,
};

const BULLET: nwg::ParaFormat = nwg::ParaFormat{
    numbering: None,
    numbering_style: None,
    numbering_tab: None,
    alignment: None,
    space_before: None,
    space_after: None,
    start_indent: Some(300),
    right_indent: Some(300),
    offset: None,
    line_spacing: Some(nwg::ParaLineSpacing::Double),
    rtl: None,
};

const HEADER_CHF: nwg::CharFormat = nwg::CharFormat{
    effects: nwg::CharEffects::from_bits(nwg::CharEffects::BOLD.bits() | nwg::CharEffects::ITALIC.bits()),
    height: Some(350),
    text_color: Some([150, 50, 50]),
    y_offset: None,
    font_face_name: None,
    underline_type: None,
};

const HEADER_PF: nwg::ParaFormat = nwg::ParaFormat{
    alignment: Some(nwg::ParaAlignment::Center),
    numbering: None,
    numbering_style: None,
    numbering_tab: None,
    space_before: None,
    space_after: None,
    start_indent: None,
    right_indent: None,
    offset: None,
    line_spacing: None,
    rtl: None,
};

impl EchoApp {
    pub fn load_text(&self, data: &nwg::EventData) {
        let drop = data.on_file_drop();

        let mut text = String::with_capacity(1000);

        for file in drop.files() {
            text.push_str(&fs::read_to_string(file).unwrap_or("Invalid file".into()));
        }
        
        self.rich_text_box.appendln(&text);
    }

    pub fn init_text(&self) {
        self.rich_text_box.set_text_unix2dos("Each button on the right does one of the following:\n");
        self.rich_text_box.append("\r\nClears the rich text box, removing all text and associated formatting.\r\n");

        self.rich_text_box.appendln_styled("Echoes the text in the input box with underline styling.", Some(UNDERLINE), None);
        self.rich_text_box.appendln_styled("Echoes the text in the input box as a bullet point.", None, Some(BULLET));
        
        self.rich_text_box.appendln_styled("Echoes the text in the input box with header styling.", Some(HEADER_CHF), Some(HEADER_PF));
        self.rich_text_box.appendln("Echoes the text in the input box with no styling.");
    }

    pub fn clear(&self) {
        self.rich_text_box.clear();
    }

    pub fn underline(&self) {
        self.rich_text_box.appendln_styled(&self.text_input.text(), Some(UNDERLINE), None);
        self.text_input.set_text("");
    }

    pub fn bullet(&self) {
        self.rich_text_box.appendln_styled(&self.text_input.text(), None, Some(BULLET));
        self.text_input.set_text("");
    }

    pub fn header(&self) {
        self.rich_text_box.appendln_styled(&self.text_input.text(), Some(HEADER_CHF), Some(HEADER_PF));
        self.text_input.set_text("");
    }

    pub fn submit(&self) {
        self.rich_text_box.appendln(&self.text_input.text());
        self.text_input.set_text("");
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Courier New").expect("Failed to set default font");

    let _app = EchoApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
