use std::io::Cursor;

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage, ImageOutputFormat};

use base64::encode;

pub enum EngineState {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

pub struct Engine {
    x: i32, // current x
    y: i32, // current y,
    state: EngineState,
    logo: DynamicImage, // just the logo image as an asset
    x_offset: u32,
    y_offset: u32,
    render_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Engine {
    pub fn new(rust_logo: DynamicImage) -> Engine {
        let view = (400, 400);

        let (logo_width, logo_height) = rust_logo.dimensions();
        let x_offset = (view.0 - logo_width) / 2;
        let y_offset = (view.1 - logo_height) / 2;

        let mut white_background = RgbaImage::new(view.0, view.1);
        for pixel in white_background.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]); // RGBA format (white color)
        }

        Engine {
            x: 0,
            y: 0,
            state: EngineState::RIGHT,
            logo: rust_logo,
            x_offset: x_offset,
            y_offset: y_offset,
            render_buffer: white_background,
        }
    }

    pub fn change_state(&mut self, new_state: EngineState) {
        self.state = new_state
    }

    fn render_bg(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // let mut white_background = RgbaImage::new(self.view.0, self.view.1);
        for pixel in self.render_buffer.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]); // RGBA format (white color)
        }

        self.render_buffer.clone()
    }

    fn update_state(&mut self) {
        match self.state {
            EngineState::RIGHT => {
                if self.x >= 100 {
                    self.state = EngineState::LEFT;
                } else {
                    self.x += 1;
                }
            }
            EngineState::LEFT => {
                if self.x <= -100 {
                    self.state = EngineState::RIGHT;
                } else {
                    self.x -= 1;
                }
            }
            EngineState::UP => {
                if self.y <= -100 {
                    self.state = EngineState::DOWN;
                } else {
                    self.y -= 1;
                }
            }
            EngineState::DOWN => {
                if self.y >= 100 {
                    self.state = EngineState::UP;
                } else {
                    self.y += 1;
                }
            }
        }
    }

    pub fn next_frame(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // update state first
        self.update_state();

        // calculate offsets
        let new_y_offset = ((self.y_offset as i32) + self.y) as u32;
        let new_x_offset = ((self.x_offset as i32) + self.x) as u32;

        // render image
        let mut bg = self.render_bg();
        bg.copy_from(&self.logo, new_x_offset, new_y_offset)
            .unwrap();
        bg.clone()
    }
}

pub fn image_to_base64(img: &DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = encode(image_data);
    format!("data:image/png;base64,{}", res_base64)
}