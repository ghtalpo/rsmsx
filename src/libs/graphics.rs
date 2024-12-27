use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use super::vdp::{SCREEN0, SCREEN1, SCREEN2};

const MSX_W1: u16 = 320;
const MSX_W2: u16 = 256;
const MSX_H: u16 = 192;
const SCALE: f32 = 2.0;
const WIN_W: f32 = MSX_W2 as f32 * SCALE;
const WIN_H: f32 = MSX_H as f32 * SCALE;

#[derive(Clone, Debug)]
enum ActiveTexture {
    Tex256,
    Tex320,
}

pub enum GraphicsType {
    None,
    Normal,
}

impl GraphicsType {
    pub fn create(self, quality: bool) -> Rc<RefCell<dyn GraphicsDriver>> {
        match self {
            GraphicsType::None => Rc::new(RefCell::new(NullGraphics::new(quality))),
            GraphicsType::Normal => Rc::new(RefCell::new(Graphics::new(quality))),
        }
    }
}

pub trait GraphicsDriver {
    fn init(&mut self);
    fn render(&mut self);
    fn set_logical_resolution(&mut self, scr_mode: u8);
    fn draw_pixel(&mut self, x: u32, y: u32, color: usize);
}

/// null driver for test without graphics
#[derive(Clone)]
pub struct NullGraphics {}
impl NullGraphics {
    pub fn new(_quality: bool) -> Self {
        Self {}
    }
}
impl GraphicsDriver for NullGraphics {
    fn init(&mut self) {}
    fn render(&mut self) {}
    fn set_logical_resolution(&mut self, _scr_mode: u8) {}
    fn draw_pixel(&mut self, _x: u32, _y: u32, _color: usize) {}
}

#[derive(Clone)]
pub struct Graphics {
    quality: bool,
    colors: [Color; 16],
    graphics_tex256: Texture2D,
    graphics_tex320: Texture2D,
    graphics_image256: Image,
    graphics_image320: Image,
    active_texture: ActiveTexture,
    zoom: f32,
    x0: i16,
    y0: i16,
}

fn get_filter_mode(quality: bool) -> FilterMode {
    if quality {
        FilterMode::Nearest
    } else {
        FilterMode::Linear
    }
}
impl Graphics {
    pub fn new(quality: bool) -> Self {
        Self {
            quality,
            colors: [
                Color::from_hex(0xff000000),
                Color::from_hex(0xff010101),
                Color::from_hex(0xff3eb849),
                Color::from_hex(0xff74d07d),
                Color::from_hex(0xff5955e0),
                Color::from_hex(0xff8076f1),
                Color::from_hex(0xffb95e51),
                Color::from_hex(0xff65dbef),
                Color::from_hex(0xffdb6559),
                Color::from_hex(0xffff897d),
                Color::from_hex(0xffccc35e),
                Color::from_hex(0xffded087),
                Color::from_hex(0xff3aa241),
                Color::from_hex(0xffb766b5),
                Color::from_hex(0xffcccccc),
                Color::from_hex(0xffffffff),
            ],
            graphics_tex256: Texture2D::empty(),
            graphics_tex320: Texture2D::empty(),
            graphics_image256: Image::gen_image_color(MSX_W2, MSX_H, BLUE),
            graphics_image320: Image::gen_image_color(MSX_W1, MSX_H, BLUE),
            active_texture: ActiveTexture::Tex256,
            zoom: SCALE,
            x0: 0,
            y0: 0,
        }
    }
}
impl GraphicsDriver for Graphics {
    fn init(&mut self) {
        request_new_screen_size(WIN_W, WIN_H);

        self.graphics_tex256 = Texture2D::from_image(&self.graphics_image256);
        self.graphics_tex256
            .set_filter(get_filter_mode(self.quality));
        self.graphics_tex320 = Texture2D::from_image(&self.graphics_image320);
        self.graphics_tex320
            .set_filter(get_filter_mode(self.quality));
    }

    fn render(&mut self) {
        let current_texture = match self.active_texture {
            ActiveTexture::Tex256 => {
                self.graphics_tex256 = Texture2D::from_image(&self.graphics_image256);
                &self.graphics_tex256
            }
            ActiveTexture::Tex320 => {
                self.graphics_tex320 = Texture2D::from_image(&self.graphics_image320);
                &self.graphics_tex320
            }
        };
        current_texture.set_filter(get_filter_mode(self.quality));
        draw_texture_ex(
            current_texture,
            ((self.x0) * self.zoom as i16).into(),
            ((self.y0) * self.zoom as i16).into(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(current_texture.size() * self.zoom),
                ..Default::default()
            },
        );
    }

    fn set_logical_resolution(&mut self, scr_mode: u8) {
        match scr_mode {
            SCREEN0 => self.active_texture = ActiveTexture::Tex320,
            SCREEN2 => self.active_texture = ActiveTexture::Tex256,
            SCREEN1 => self.active_texture = ActiveTexture::Tex256,
            _ => panic!("setLogicalResolution: mode not supported"),
        }
    }
    fn draw_pixel(&mut self, x: u32, y: u32, color: usize) {
        // println!("dp: {}/{} {} ci:{:?}", x, y, color, self.active_texture);
        let current_image = match self.active_texture {
            ActiveTexture::Tex256 => &mut self.graphics_image256,
            ActiveTexture::Tex320 => &mut self.graphics_image320,
        };
        current_image.set_pixel(x, y, self.colors[color]);
    }
}
