use frogquad::{
    Color, FrogCx,
    glam::Vec2,
    math::Rect,
    texture::{DrawTextureParams, Texture2D},
};

pub struct SimpleButton {
    pub rect: Rect,
    text: String,
    font_size: u16,
    text_offset_y: f32,
}

pub struct ImgButton {
    img_src_rect: Rect,
    pub rect: Rect,
    up_color: Color,
    down_color: Color,
}

const IMG_BUTTON_PADDING: f32 = 4.0;

impl ImgButton {
    pub fn new(img_src_rect: Rect, x: f32, y: f32, up_color: Color, down_color: Color) -> Self {
        let rect = Rect {
            x,
            y,
            w: img_src_rect.w + IMG_BUTTON_PADDING,
            h: img_src_rect.h + IMG_BUTTON_PADDING,
        };
        Self {
            img_src_rect,
            rect,
            up_color,
            down_color,
        }
    }
    pub fn draw(&self, fc: &mut FrogCx, tex: &Texture2D, mx: f32, my: f32) {
        let bg_color = if self.mouse_over(mx, my) {
            self.down_color
        } else {
            self.up_color
        };
        fc.draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg_color);
        fc.draw_texture_ex(
            tex,
            self.rect.x + IMG_BUTTON_PADDING / 2.0,
            self.rect.y + IMG_BUTTON_PADDING / 2.0,
            Color::WHITE,
            DrawTextureParams {
                source: Some(self.img_src_rect),
                ..Default::default()
            },
        );
    }
    pub fn mouse_over(&self, mx: f32, my: f32) -> bool {
        self.rect.contains(Vec2::new(mx, my))
    }
}

const BUTTON_PADDING: f32 = 8.0;

impl SimpleButton {
    pub fn new(fc: &mut FrogCx, text: String, x: f32, y: f32, font_size: u16) -> Self {
        let mut neu = Self {
            text: String::new(),
            rect: Rect {
                x,
                y,
                w: 0.0,
                h: 0.0,
            },
            font_size,
            text_offset_y: 0.0,
        };
        neu.set_text(fc, text);
        neu
    }
    pub fn set_text(&mut self, fc: &mut FrogCx, text: String) {
        self.text = text;
        let tdim = fc
            .default_font()
            .measure_text(&self.text, self.font_size, 1.0);
        self.rect.w = tdim.width + BUTTON_PADDING;
        self.rect.h = tdim.height + BUTTON_PADDING;
        self.text_offset_y = tdim.offset_y;
    }
    pub fn mouse_over(&self, mx: f32, my: f32) -> bool {
        self.rect.contains(Vec2::new(mx, my))
    }
    pub fn draw(&self, fc: &mut FrogCx, mx: f32, my: f32) {
        let bg_color = if self.mouse_over(mx, my) {
            Color::LIGHTGRAY
        } else {
            Color::GRAY
        };
        let Rect { x, y, w, h } = self.rect;
        fc.draw_rectangle(x, y, w, h, bg_color);
        fc.draw_rectangle_lines(x, y, w, h, 1.0, Color::BLACK);
        fc.draw_text(
            &self.text,
            x + BUTTON_PADDING / 2.0,
            y + self.text_offset_y + BUTTON_PADDING / 2.0,
            f32::from(self.font_size),
            Color::BLACK,
        );
    }
}
