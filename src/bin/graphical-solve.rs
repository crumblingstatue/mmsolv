#![feature(decl_macro)]

use macroquad::prelude::*;

const PEG_SIZE: f32 = 64.0;

macro_rules! colors {
    ($($r:expr,$g:expr,$b:expr;)*) => {
        [
            $(
            color_u8!($r, $g, $b, 255),
            )*
        ]
    };
}

const MIN_PEGS: u8 = 3;
const MAX_PEGS: u8 = 5;

const PEG_COLORS: [Color; 15] = colors! {
     65, 167,  64;
    111, 131, 219;
    245, 126, 125;
    143, 234,  64;
    177, 237, 238;
     26,  26,  30;
     11,  69, 166;
    178,  56,  35;
    238, 218,  77;
    180, 121,  34;
    168,  84, 203;
    249, 187,  74;
    254, 206, 239;
    249, 247, 217;
    122, 130, 137;
};

#[derive(Clone, Copy)]
struct Pegbug {
    x: f32,
    y: f32,
    /// Simple unique identifier for type of peg.
    ///
    /// For drawing them, we use the id as an index into a color array.
    id: u8,
}

impl Pegbug {
    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: PEG_SIZE,
            h: PEG_SIZE,
        }
    }
}

const BUGS_PER_ROW: u8 = 8;

fn bottom_peg(idx: usize) -> Pegbug {
    Pegbug {
        x: (idx % BUGS_PER_ROW as usize) as f32 * 80.,
        y: screen_height() - (PEG_SIZE * 2.0) + (idx / BUGS_PER_ROW as usize) as f32 * PEG_SIZE,
        id: idx as u8,
    }
}

fn bottom_pegs() -> impl Iterator<Item = Pegbug> {
    (0..PEG_COLORS.len()).map(bottom_peg)
}

fn draw_peg(peg_tex: Texture2D, peg: Pegbug) {
    draw_texture(peg_tex, peg.x, peg.y, PEG_COLORS[peg.id as usize]);
}

fn draw_bottom_pegs(peg_tex: Texture2D) {
    bottom_pegs().for_each(|peg| {
        draw_peg(peg_tex, peg);
    });
}

struct SimpleButton {
    rect: Rect,
    text: String,
    font_size: u16,
    text_offset_y: f32,
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
            self.font_size as f32,
            BLACK,
        );
    }
}

struct IncWrap {
    pub value: u8,
    min: u8,
    max: u8,
}

impl IncWrap {
    pub fn new(min: u8, max: u8) -> Self {
        Self {
            value: min,
            min,
            max,
        }
    }
    pub fn inc(&mut self) {
        self.value += 1;
        if self.value > self.max {
            self.value = self.min;
        }
    }
}

#[macroquad::main("mmsolv")]
async fn main() {
    let mut picked_peg: Option<Pegbug> = None;
    let mut placed_pegs = Vec::new();
    let mut n_pegs_in_clues = IncWrap::new(MIN_PEGS, MAX_PEGS);
    macro ptype_but_text() {
        format!("Puzzle type: {} peg", n_pegs_in_clues.value)
    }
    let mut ptype_but = SimpleButton::new(ptype_but_text!(), 8.0, 8.0, 32);
    loop {
        clear_background(WHITE);
        let peg_tex =
            Texture2D::from_file_with_format(include_bytes!("../../assets/pegbug.png"), None);
        let (mx, my) = mouse_position();
        draw_bottom_pegs(peg_tex);
        if let Some(ref mut peg) = picked_peg {
            peg.x = mx - 32.;
            peg.y = my - 32.;
            draw_peg(peg_tex, *peg);
        }
        // Handle mouse pressed
        if is_mouse_button_pressed(MouseButton::Left) {
            if ptype_but.mouse_over(mx, my) {
                n_pegs_in_clues.inc();
                ptype_but.set_text(ptype_but_text!());
            }
            if picked_peg.is_none() {
                for peg in bottom_pegs() {
                    if peg.rect().contains(Vec2::new(mx, my)) {
                        picked_peg = Some(peg);
                    }
                }
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(peg) = picked_peg {
                placed_pegs.push(peg);
                picked_peg = None;
            }
        }
        ptype_but.draw(mx, my);

        for peg in &placed_pegs {
            draw_peg(peg_tex, *peg);
        }

        next_frame().await
    }
}
