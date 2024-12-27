use iced::{
    self,
    window,
    color,
    Theme,
    // Background,
    // Border,
    border,
    Element,
    // Padding,
    // padding,
    Fill,
    Size,
    widget::{
        container,
        mouse_area,
        column, 
        row,
        text,
        text_input,
        button,
        Button,
        Text,
        Row,
    },
    alignment::Horizontal,
    theme::Palette,
    mouse::ScrollDelta,
};

const NUM_BITS: usize = 8;
const PRIM_COLOR: u32 = 0xE0E0E0;
const WINDOW_SIZE: (f32, f32) = (750.0, 200.0);

pub struct State {
    value: u8,
    bit_strings: Vec<String>,
    hex_string: String,
    int_string: String,
    uint_string: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            value: 0,
            bit_strings: vec!["0".to_string(); 8],
            hex_string: "00".to_string(),
            int_string: "0".to_string(),
            uint_string: "0".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(usize),
    Clear,
    Invert,
    HexChanged(String),
    IntChanged(String),
    UintChanged(String),
    Scrolled(ScrollDelta),
}

fn theme(_state: &State) -> Theme {
    let mut palette = Palette::DARK;
    // palette.primary = color!(0xB14040);
    palette.primary = color!(PRIM_COLOR);
    palette.text = color!(PRIM_COLOR);

    // let mut palette = Palette::LIGHT;
    // palette.primary = color!(0x262626);
    Theme::custom(
        "Test".to_string(),
        palette,
    )
}

pub fn run() -> iced::Result {
    // iced::run("Byte Converter", update, view)
    // let mut settings = window::Settings::default();
    // settings.size = Size::new(800.0, 400.0);

    let window_settings = window::Settings {
        size: Size::new(
            WINDOW_SIZE.0,
            WINDOW_SIZE.1
        ),
        resizable: false,
        ..Default::default()
    };
    iced::application("Byte Converter", update, view)
        .window(window_settings)
        .theme(theme)
        .run()
}

// fn button_style(theme: &Theme, _status: button::Status) -> button::Style {
//     let background = match {
//         button::Status::Active => Some(Background::Color(theme.palette().background)),
//         button::Status::Hovered => Some(Background::Color(theme.palette().background)),
//         button::Status::Pressed => Some(Background::Color(theme.palette().background)),
//         _ => Some(Background::Color(theme.palette().background))
//     };
//     let text_color = theme.palette().text;
//     let border = Border::default().color(theme.palette().text).width(1.0);
//     button::Style {
//         background,
//         text_color,
//         border,
//         ..Default::default()
//     }
// }

fn button_style(theme: &Theme, status: button::Status) -> button::Style {
    // let mut style = button::secondary(theme, status);
    // style.border.radius = border::Radius::new(0);
    // style
    button::secondary(theme, status)
}

// pub fn button_style(theme: &Theme, status: button::Status) -> button::Style {
//     let palette = theme.extended_palette();
//     // let base = button::styled(palette.secondary.base);
//     let field = palette.success;
//     let pair = field.base;
//     let base = button::Style {
//         background: Some(Background::Color(pair.color)),
//         text_color: pair.text,
//         border: border::rounded(2),
//         ..button::Style::default()
//     };

//     match status {
//         button::Status::Active | button::Status::Pressed => base,
//         button::Status::Hovered => button::Style {
//             background: Some(Background::Color(field.strong.color)),
//             ..base
//         },
//         button::Status::Disabled => button::Style {
//             background: base
//                 .background
//                 .map(|background| background.scale_alpha(0.5)),
//             text_color: base.text_color.scale_alpha(0.5),
//             ..base
//         },
//     }
// }


// fn text_entry<'a>(text: &'a str, content: &'a String, message: Message + 'a) -> Row<Message + 'a> {
//     row![
//         container(
//             text(text)
//             .size(30)
//         ).height().center_y(Fill),
//         text_input(content, content)
//             .size(30)
//             .width(100)
//             .on_input(message)
//     ].spacing(10)
// }



fn view(state: &State) -> Element<Message> {
    let mut button_row = Row::new().spacing(5);
    for i in 0..NUM_BITS {
        let col  = column![
            container(Button::new(container(Text::new(&*state.bit_strings[NUM_BITS-1-i]).size(35)).center_x(Fill).center_y(Fill))
                .width(Fill)
                .height(Fill)
                .style(button_style)
                // .style(button::secondary)
                .on_press(Message::Toggle(NUM_BITS-1-i))).center_x(Fill),
            container(text(format!("Bit {}", 7-i))).center_x(Fill)
        ];
        button_row = button_row.push(col);
    }
    let command_width = 70;
    let commands = column![
        Button::new(container("Clear").center_x(Fill).center_y(Fill))
            .style(button_style)
            .width(command_width)
            .height(Fill)
            .on_press(Message::Clear),
        Button::new(container("Invert").center_x(Fill).center_y(Fill))
            .style(button_style)
            .width(command_width)
            .height(Fill)
            .on_press(Message::Invert),
    ]
    // .padding(padding::left(5))
    .spacing(5);
    button_row = button_row.push(commands);
    let col = column![
        container(button_row)
            .padding(10)
            .height(100)
            .center_x(Fill),
        container(
            row![
                row![
                    container(
                        text("Hex:")
                            .size(20)
                    ).center_y(Fill),
                    container(
                        text_input(&state.hex_string, &state.hex_string)
                            .size(25)
                            .width(75)
                            .align_x(Horizontal::Center)
                            .on_input(Message::HexChanged)
                    ).center_y(Fill),
                ].spacing(10),
                
                row![
                    container(
                        text("Uint:")
                            .size(20)
                    ).center_y(Fill),
                    container(
                        text_input(&state.uint_string, &state.uint_string)
                            .size(25)
                            .width(75)
                            .align_x(Horizontal::Center)
                            .on_input(Message::UintChanged)
                    ).center_y(Fill),
                ].spacing(10),
                
                row![
                    container(
                        text("Int:")
                            .size(20),
                    ).center_y(Fill),
                    container(
                        text_input(&state.int_string, &state.int_string)
                            .size(25)
                            .width(75)
                            .align_x(Horizontal::Center)
                            .on_input(Message::IntChanged)
                    ).center_y(Fill),
                ].spacing(10),
            ]
            .spacing(20)
        ).center_x(Fill)
    ];
    // container(
    //     mouse_area(
    //         col
    //     )
    //     .on_scroll(Message::Scrolled)
    // )
    // .padding(10)
    // .center_x(Fill)
    // .center_y(Fill)
    // .into()

    mouse_area(
        container(col)
            .padding(10)
            .center_x(Fill)
            .center_y(Fill)
    )
    .on_scroll(Message::Scrolled)
    .into()
}

fn update_strings(state: &mut State) {
    for i in 0..NUM_BITS {
        state.bit_strings[i] = ((state.value >> i) & 1).to_string();
    }
    state.hex_string = format!("{:02X}", state.value);
    state.int_string = format!("{}", state.value as i8);
    state.uint_string = format!("{}", state.value);
}

fn set_from_hex(state: &mut State, new_string: String) {
    if new_string.len() <= 2 && new_string.chars().all(|c| c.is_digit(16)) {
        match u8::from_str_radix(&new_string, 16) {
            Ok(value) => state.value = value,
            _ => state.value = 0,
        }
        update_strings(state);
        state.hex_string = new_string;
    }
}

fn set_from_int(state: &mut State, new_string: String) {
    if new_string.len() <= 4 && (new_string.parse::<i8>().is_ok() || new_string.is_empty() || new_string == "-".to_string()) {
        match new_string.parse::<i8>() {
            Ok(value) => state.value = value as u8,
            _ => state.value = 0,
        }
        update_strings(state);
        state.int_string = new_string.clone();
    }
}

fn set_from_uint(state: &mut State, new_string: String) {
    if new_string.len() <= 3 && (new_string.parse::<u8>().is_ok() || new_string.is_empty()) {
        match new_string.parse::<u8>() {
            Ok(value) => state.value = value,
            _ => state.value = 0,
        }
        update_strings(state);
        state.uint_string = new_string.clone();
    }
}

fn set_from_scroll(state: &mut State, delta: ScrollDelta) {
    let delta_y = match delta {
        ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => y,
    };
    if delta_y > 0.0 {
        state.value = state.value.wrapping_add(1);
    } else {
        state.value = state.value.wrapping_sub(1);
    }
    update_strings(state);
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Toggle(i) => {
            state.value ^= 1 << i;
            update_strings(state);
        }
        Message::Clear => {
            state.value = 0;
            update_strings(state);
        }
        Message::Invert => {
            state.value = !state.value;
            update_strings(state);
        }
        Message::HexChanged(new_string) => set_from_hex(state, new_string),
        Message::IntChanged(new_string) => set_from_int(state, new_string),
        Message::UintChanged(new_string) => set_from_uint(state, new_string),
        Message::Scrolled(delta) => set_from_scroll(state, delta),
    }
}