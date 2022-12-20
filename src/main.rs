mod cpu;
mod input_controller;
mod parser;
mod register;
mod token;
mod delegate;

use cpu::Cpu;

use druid::{
    widget::{Align, Container, Flex, Label, TextBox},
    AppLauncher, Color, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc, FontDescriptor, FontFamily, FontWeight,
};

const FONT: FontDescriptor = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(20.).with_weight(FontWeight::SEMI_BOLD);

fn main() {
    let window = WindowDesc::new(build)
        .title("my app")
        .window_size((940., 550.));

    let cpu = Cpu::default();

    let state = AppState {
        output: cpu.registers_str(),
        ..AppState::default()
    };
    
    AppLauncher::with_window(window)
        .delegate(delegate::Delegate)
        .launch(state)
        .unwrap();
}

#[derive(Clone, Debug, Data, Lens, Default)]
struct AppState {
    input: String,
    output: String,
    cpu: Cpu
}

fn build() -> impl Widget<AppState> {
    let input = TextBox::multiline()
        .with_font(FONT)
        .fix_width(450.)
        .fix_height(500.)
        .lens(AppState::input);

    let input_label = Label::new("Input");

    let input = Flex::column()
        .with_child(input_label)
        .with_flex_child(input, 1.0);

    let output = Container::new(Label::raw().with_font(FONT).lens(AppState::output))
        .background(Color::from_hex_str("#6039b3").unwrap_or(Color::grey8(0x55)))
        .controller(input_controller::InputController)
        .fix_width(450.)
        .fix_height(500.);

    let output_label = Label::new("Output");

    let output = Flex::column().with_child(output_label).with_child(output);

    let input = Align::new(UnitPoint::TOP_LEFT, input).padding(10.);

    let output = Align::new(UnitPoint::TOP_RIGHT, output).padding(10.);

    Flex::row().with_child(input).with_child(output)
}
