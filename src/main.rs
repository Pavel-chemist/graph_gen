use fltk::{
    app,
    enums::{self, Color, ColorDepth, Event, FrameType},
    image::RgbImage,
    prelude::*,
    *,
};
use rand::random;

const WIND_LABEL: &str = "Graphics Generator";
// const WIND_WIDTH: i32 = 1820;
const WIND_WIDTH: i32 = 800;
// const WIND_HEIGHT: i32 = 1000;
const WIND_HEIGHT: i32 = 600;
// const MAIN_IMAGE_WIDTH: i32 = 940;
const MAIN_IMAGE_WIDTH: i32 = 512;
// const MAIN_IMAGE_HEIGHT: i32 = 940;
const MAIN_IMAGE_HEIGHT: i32 = 512;
const MAIN_IMAGE_FRAME_THICKNESS: i32 = 4;
const MAIN_IMAGE_X_POS: i32 = 10;
const MAIN_IMAGE_Y_POS: i32 = 10;
const MENU_HEIGHT: i32 = 32;

struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
enum Message {
    Quit,
    ImageEvent,
    WBev,
    BBev,
    GBev,
    LGBev,
    MouseDown(i32, i32),
    MouseDrag(i32, i32),
    MouseMove(i32, i32),
}

enum Colour {
    Black,
    Grey,
    LightGrey,
    White,
}

fn main() {
    let application = app::App::default();
    let (s, r) = app::channel();

    let mut wind = window::Window::new(0, 0, WIND_WIDTH, WIND_HEIGHT, WIND_LABEL);

    let mut menu = menu::SysMenuBar::default().with_size(wind.width(), MENU_HEIGHT);
    menu.set_frame(enums::FrameType::FlatBox);
    menu.set_color(enums::Color::Light2);

    menu.add_emit(
        "&File/Quit\t",
        enums::Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Quit,
    );

    let mut framing_frame = frame::Frame::default()
        .with_pos(MAIN_IMAGE_X_POS, MAIN_IMAGE_Y_POS + MENU_HEIGHT)
        .with_size(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_FRAME_THICKNESS * 2,
            MAIN_IMAGE_HEIGHT + MAIN_IMAGE_FRAME_THICKNESS * 2,
        );
    framing_frame.set_frame(FrameType::EngravedBox);

    let mut image_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_X_POS + MAIN_IMAGE_FRAME_THICKNESS,
            MAIN_IMAGE_Y_POS + MAIN_IMAGE_FRAME_THICKNESS + MENU_HEIGHT,
        )
        .with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    let mut image_data: Vec<u8> =
        generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::LightGrey);
    let mut image: RgbImage = RgbImage::new(
        &image_data,
        MAIN_IMAGE_WIDTH,
        MAIN_IMAGE_HEIGHT,
        ColorDepth::Rgb8,
    )
    .unwrap();
    image_frame.set_image(Some(image));
    image_frame.emit(s.clone(), Message::ImageEvent);

    let _side_panel_title_frame = frame::Frame::default()
        .with_pos(
            MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
            MENU_HEIGHT + MAIN_IMAGE_Y_POS,
        )
        .with_size(200, 40)
        .with_label("Change background color:");

    let mut b_white = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 50,
        40,
        40,
        "",
    );
    b_white.set_color(Color::White);
    b_white.emit(s.clone(), Message::WBev);

    let mut b_grey = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 1,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 50,
        40,
        40,
        "",
    );
    b_grey.set_color(Color::rgb_color(127, 127, 127));
    b_grey.emit(s.clone(), Message::GBev);

    let mut b_light_grey = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 2,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 50,
        40,
        40,
        "",
    );
    b_light_grey.set_color(Color::FrameDefault);
    b_light_grey.emit(s.clone(), Message::LGBev);

    let mut b_black = button::Button::new(
        MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20 + (40 + 20) * 3,
        MENU_HEIGHT + MAIN_IMAGE_Y_POS + 50,
        40,
        40,
        "",
    );
    b_black.set_color(Color::Black);
    b_black.emit(s.clone(), Message::BBev);

    wind.end();
    wind.show();

    image_frame.handle(move |_, event: Event| match event {
        Event::Push => {
            let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
            let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
            s.send(Message::MouseDown(x, y));
            true
        }
        Event::Drag => {
            let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
            let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
            if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                s.send(Message::MouseDrag(x, y));
            }
            true
        }
        Event::Move => {
            let x = app::event_x() - MAIN_IMAGE_X_POS - MAIN_IMAGE_FRAME_THICKNESS;
            let y = app::event_y() - MAIN_IMAGE_Y_POS - MAIN_IMAGE_FRAME_THICKNESS - MENU_HEIGHT;
            if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                s.send(Message::MouseMove(x, y));
            }
            true
        }
        _ => false,
    });

    while application.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => {
                    println!("quitting the app...");
                    fltk::app::quit();
                }
                Message::ImageEvent => {
                    println!("some event on image...");
                }
                Message::WBev => {
                    image_data = generate_image_background(
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        Colour::White,
                    );
                    image = RgbImage::new(
                        &image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        ColorDepth::Rgb8,
                    )
                    .unwrap();
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
                Message::LGBev => {
                    image_data = generate_image_background(
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        Colour::LightGrey,
                    );
                    image = RgbImage::new(
                        &image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        ColorDepth::Rgb8,
                    )
                    .unwrap();
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
                Message::GBev => {
                    image_data = generate_image_background(
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        Colour::Grey,
                    );
                    image = RgbImage::new(
                        &image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        ColorDepth::Rgb8,
                    )
                    .unwrap();
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
                Message::BBev => {
                    image_data = generate_image_background(
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        Colour::Black,
                    );
                    image = RgbImage::new(
                        &image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        ColorDepth::Rgb8,
                    )
                    .unwrap();
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
                Message::MouseDown(x, y) => {
                    println!("The image was clicked at coordinates x={}, y={}", x, y);

                    radial_gradient(
                        &mut image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        x,
                        y,
                        16.0,
                        RGBColor {
                            r: random(),
                            g: random(),
                            b: random(),
                        },
                    );
                    image = match unsafe {
                        RgbImage::from_data(
                            &image_data,
                            MAIN_IMAGE_WIDTH,
                            MAIN_IMAGE_HEIGHT,
                            ColorDepth::Rgb8,
                        )
                    } {
                        Ok(image) => image,
                        Err(e) => {
                            eprintln!("Error creating image from data: {:?}", e);
                            continue;
                        }
                    };
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
                Message::MouseDrag(x, y) => {
                    println!("There was Drag event at coordinates x={}, y={}", x, y);

                    place_square(
                        &mut image_data,
                        MAIN_IMAGE_WIDTH,
                        x,
                        y,
                        5,
                        RGBColor {
                            r: random(),
                            g: random(),
                            b: random(),
                        }
                    );

                    image = match unsafe {
                        RgbImage::from_data(
                            &image_data,
                            MAIN_IMAGE_WIDTH,
                            MAIN_IMAGE_HEIGHT,
                            ColorDepth::Rgb8,
                        )
                    } {
                        Ok(image) => image,
                        Err(e) => {
                            eprintln!("Error creating image from data: {:?}", e);
                            continue;
                        }
                    };
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
                Message::MouseMove(x, y) => {
                    println!("There was Move event at coordinates x={}, y={}", x, y);
                } /* _ => {
                      println!("yet undefined event");
                  } */
            };
        }
    }

    application.run().unwrap();
}

fn generate_image_background(width: i32, height: i32, colour: Colour) -> Vec<u8> {
    let num_pix: usize = (width * height) as usize;

    let data_array: Vec<u8>;

    match colour {
        Colour::Black => data_array = vec![0; num_pix * 3],
        Colour::Grey => data_array = vec![127; num_pix * 3],
        Colour::LightGrey => data_array = vec![191; num_pix * 3],
        Colour::White => data_array = vec![255; num_pix * 3],
    }

    return data_array;
}

fn place_square(image_data: &mut Vec<u8>, width: i32, x: i32, y: i32, size: i32, color: RGBColor) {
    let mut index_r: usize;

    for j in (y - size / 2)..(y + size / 2) {
        for i in (x - size / 2)..(x + size / 2) {
            if i >= 0 && i < MAIN_IMAGE_WIDTH && j > 0 && j < MAIN_IMAGE_HEIGHT {
                index_r = (width * 3 * j + i * 3) as usize;

                image_data[index_r] = color.r;
                image_data[index_r + 1] = color.g;
                image_data[index_r + 2] = color.b;
            }
        }
    }
}

fn radial_gradient(
    image_data: &mut Vec<u8>,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    parameter: f32,
    color: RGBColor,
) {
    let mut index_r: usize;
    let mut squared_distance: f32;
    let mut brightness: f32;
    let mut r_br: f32;
    let mut g_br: f32;
    let mut b_br: f32;

    for j in 0..height {
        for i in 0..width {
            index_r = (width * 3 * j + i * 3) as usize;
            squared_distance =
                (((x - i) * (x - i) + (y - j) * (y - j)) as f32) / (parameter * parameter);
            if squared_distance <= 1.0 {
                brightness = 1.0;
            } else {
                brightness = 1.0 / squared_distance;
            }

            r_br = brightness * (color.r as f32) + (image_data[index_r] as f32);
            g_br = brightness * (color.g as f32) + (image_data[index_r + 1] as f32);
            b_br = brightness * (color.b as f32) + (image_data[index_r + 2] as f32);

            if r_br > 255.0 {
                r_br = 255.0;
            }

            if b_br > 255.0 {
                b_br = 255.0;
            }

            if g_br > 255.0 {
                g_br = 255.0;
            }

            image_data[index_r] = r_br as u8;
            image_data[index_r + 1] = g_br as u8;
            image_data[index_r + 2] = b_br as u8;
        }
    }
}
