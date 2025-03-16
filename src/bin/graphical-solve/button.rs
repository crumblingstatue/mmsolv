use macroquad::prelude::*;

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
    pub fn draw(&self, tex: &Texture2D, mx: f32, my: f32) {
        let bg_color = if self.mouse_over(mx, my) {
            self.down_color
        } else {
            self.up_color
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg_color);
        draw_texture_ex(
            tex,
            self.rect.x + IMG_BUTTON_PADDING / 2.0,
            self.rect.y + IMG_BUTTON_PADDING / 2.0,
            WHITE,
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
    pub fn new(text: String, x: f32, y: f32, font_size: u16) -> Self {
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
        neu.set_text(text);
        neu
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        let tdim = measure_text(&self.text, None, self.font_size, 1.0);
        self.rect.w = tdim.width + BUTTON_PADDING;
        self.rect.h = tdim.height + BUTTON_PADDING;
        self.text_offset_y = tdim.offset_y;
    }
    pub fn mouse_over(&self, mx: f32, my: f32) -> bool {
        self.rect.contains(Vec2::new(mx, my))
    }
    pub fn draw(&self, mx: f32, my: f32) {
        let bg_color = if self.mouse_over(mx, my) {
            LIGHTGRAY
        } else {
            GRAY
        };
        let Rect { x, y, w, h } = self.rect;
        draw_rectangle(x, y, w, h, bg_color);
        draw_rectangle_lines(x, y, w, h, 1.0, BLACK);
        draw_text(
            &self.text,
            x + BUTTON_PADDING / 2.0,
            y + self.text_offset_y + BUTTON_PADDING / 2.0,
            f32::from(self.font_size),
            BLACK,
        );
    }
}
