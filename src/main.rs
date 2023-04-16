use fltk::{app, prelude::*, *, enums::{self, ColorDepth, FrameType, Event}, image::RgbImage};

const WIND_LABEL: &str = "Graphics Generator";
// const WIND_WIDTH: i32 = 1820;
const WIND_WIDTH: i32 = 800;
// const WIND_HEIGHT: i32 = 1000;
const WIND_HEIGHT: i32 = 600;
const MAIN_IMAGE_WIDTH: i32 = 512;
const MAIN_IMAGE_HEIGHT: i32 = 512;
const MAIN_IMAGE_X_POS: i32 = 4;
const MAIN_IMAGE_Y_POS: i32 = 4;
const MENU_HEIGHT: i32 = 32;

#[derive(Clone)]
enum Message {
    Quit,
    ImageEvent,
    ButtonEvent,
    MouseDown(i32, i32),
    MouseDrag(i32, i32),
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

    menu.add_emit(
        "&File/Quit\t",
        enums::Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Quit,
    );

    let mut framing_frame = frame::Frame::default()
    .with_pos(0, MENU_HEIGHT).with_size(MAIN_IMAGE_WIDTH + 8, MAIN_IMAGE_HEIGHT + 8);
    framing_frame.set_frame(FrameType::DownBox);

    let mut image_frame = frame::Frame::default()
        .with_pos(MAIN_IMAGE_X_POS, MAIN_IMAGE_Y_POS + MENU_HEIGHT).with_size(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT);

    let mut image_data: Vec<u8> = generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::LightGrey);
    let mut image: RgbImage = RgbImage::new(&image_data, MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, ColorDepth::Rgb8).unwrap();
    image_frame.set_image(Some(image));
    image_frame.emit(s.clone(), Message::ImageEvent);

    let mut butt = button::Button::new(MAIN_IMAGE_WIDTH + MAIN_IMAGE_X_POS + 20, MENU_HEIGHT + MAIN_IMAGE_Y_POS, 200, 40, "Make the square white");
    butt.emit(s.clone(), Message::ButtonEvent);
    

    wind.end(); 
    wind.show();

    image_frame.handle(move |_, event: Event| match event {
       
        Event::Push => {
            let x = app::event_x() - MAIN_IMAGE_X_POS;
            let y = app::event_y() - MAIN_IMAGE_Y_POS - MENU_HEIGHT;
            s.send(Message::MouseDown(x, y));
            true
        },
        Event::Drag => {
            let x = app::event_x() - MAIN_IMAGE_X_POS;
            let y = app::event_y() - MAIN_IMAGE_Y_POS - MENU_HEIGHT;
            if x >= 0 && x < MAIN_IMAGE_WIDTH && y >= 0 && y < MAIN_IMAGE_HEIGHT {
                s.send(Message::MouseDrag(x, y));
            }
            true
        },
        _ => false,
    });

    while application.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => {
                    println!("quitting the app...");
                    fltk::app::quit();
                },
                Message::ImageEvent => {
                    println!("some event on image...");
                },
                Message::ButtonEvent => {
                    println!("button was clicked...");

                    image_data = generate_image_background(MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, Colour::White);
                    image = RgbImage::new(&image_data, MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, ColorDepth::Rgb8).unwrap();
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                },
                Message::MouseDown(x, y) => {
                    println!("The image was clicked at coordinates x={}, y={}", x, y);

                    place_square(&mut image_data, MAIN_IMAGE_WIDTH, x, y, 9);
                    image = match unsafe { RgbImage::from_data(
                        &image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        ColorDepth::Rgb8) } {
                            Ok(image) => image,
                            Err(e) => {
                                eprintln!("Error creating image from data: {:?}", e);
                                continue;
                            }
                        };
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                },
                Message::MouseDrag(x, y) => {
                    println!("The image was dragged at coordinates x={}, y={}", x, y);

                    place_square(&mut image_data, MAIN_IMAGE_WIDTH, x, y, 5);
                    // image = RgbImage::new(&image_data, MAIN_IMAGE_WIDTH, MAIN_IMAGE_HEIGHT, ColorDepth::Rgb8).unwrap();
                    // image_frame.set_image(Some(image));

                    image = match unsafe { RgbImage::from_data(
                        &image_data,
                        MAIN_IMAGE_WIDTH,
                        MAIN_IMAGE_HEIGHT,
                        ColorDepth::Rgb8) } {
                            Ok(image) => image,
                            Err(e) => {
                                eprintln!("Error creating image from data: {:?}", e);
                                continue;
                            }
                        };
                    image_frame.set_image(Some(image));
                    image_frame.redraw();
                }
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

fn place_square(image_data: &mut Vec<u8>, width: i32, x: i32, y: i32, size: i32) {
    let mut index_r: usize;

    for j in (y-size/2)..(y+size/2) {
        for i in (x-size/2)..(x+size/2) {
            if i >= 0 && i < MAIN_IMAGE_WIDTH && j > 0 && j < MAIN_IMAGE_HEIGHT {
                index_r = (width * 3 * j + i * 3) as usize;

                image_data[index_r] = 0;
                image_data[index_r + 1] = 0;
                image_data[index_r + 2] = 0;
            }
        }
    }
}
