#![feature(decl_macro)]

use macroquad::prelude::*;
use mmsolv::{solve, Clue};

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

#[derive(Debug)]
struct ClueRow {
    slots: Vec<Option<mmsolv::Peg>>,
}

impl ClueRow {
    pub fn new(slots: u8) -> Self {
        Self {
            slots: vec![None; slots as usize],
        }
    }
}

const CLUE_ROW_X_OFFSET: f32 = 300.;
const CLUE_ROW_Y_OFFSET: f32 = 16.;
const BOX_PADDING_INNER: f32 = 4.;
const BOX_SIZE: f32 = PEG_SIZE + BOX_PADDING_INNER;
const BOX_VERT_DISTANCE: f32 = 8.;
const BOX_HORIZ_DISTANCE: f32 = 8.;

fn clue_rect(row: usize, col: usize) -> Rect {
    Rect {
        x: CLUE_ROW_X_OFFSET + col as f32 * (BOX_SIZE + BOX_HORIZ_DISTANCE),
        y: CLUE_ROW_Y_OFFSET + row as f32 * (BOX_SIZE + BOX_VERT_DISTANCE),
        w: BOX_SIZE,
        h: BOX_SIZE,
    }
}

fn clue_rects(rows: &[ClueRow]) -> impl Iterator<Item = (Rect, usize, usize)> + '_ {
    rows.iter().enumerate().flat_map(|(row_num, row)| {
        row.slots
            .iter()
            .enumerate()
            .map(move |(col, _)| (clue_rect(row_num, col), row_num, col))
    })
}

fn draw_clue_row(
    row_num: usize,
    row: &ClueRow,
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    peg_tex: Texture2D,
) {
    for (i, slot) in row.slots.iter().enumerate() {
        let rect = clue_rect(row_num, i);
        if let Some(picked_color) = picked_color {
            if rect.contains(Vec2::new(mx, my)) {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, picked_color);
            }
        }
        if let Some(pegid) = *slot {
            draw_peg(
                peg_tex,
                Pegbug {
                    id: pegid,
                    x: rect.x,
                    y: rect.y,
                },
            );
        }
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, RED);
    }
}

fn draw_clue_rows(
    rows: &[ClueRow],
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    peg_tex: Texture2D,
) {
    for (i, row) in rows.iter().enumerate() {
        draw_clue_row(i, row, mx, my, picked_color, peg_tex);
    }
}

#[macroquad::main("mmsolv")]
async fn main() {
    let mut picked_peg: Option<Pegbug> = None;
    let mut n_pegs_in_clues = IncWrap::new(MIN_PEGS, MAX_PEGS);
    macro ptype_but_text() {
        format!("Puzzle type: {} peg", n_pegs_in_clues.value)
    }
    let mut ptype_but = SimpleButton::new(ptype_but_text!(), 8.0, 8.0, 32);
    let clue_add_but = SimpleButton::new("Add clue row".into(), 8.0, 48.0, 32);
    let mut clue_rows = Vec::new();
    loop {
        clear_background(WHITE);
        let peg_tex =
            Texture2D::from_file_with_format(include_bytes!("../../assets/pegbug.png"), None);
        let (mx, my) = mouse_position();
        // Handle mouse pressed
        if is_mouse_button_pressed(MouseButton::Left) {
            if ptype_but.mouse_over(mx, my) {
                n_pegs_in_clues.inc();
                ptype_but.set_text(ptype_but_text!());
            }
            if clue_add_but.mouse_over(mx, my) {
                clue_rows.push(ClueRow::new(n_pegs_in_clues.value));
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
                let mut ins_loc = None;
                for (clue_rect, row, col) in clue_rects(&clue_rows) {
                    if clue_rect.contains(Vec2::new(mx, my)) {
                        ins_loc = Some((row, col));
                        break;
                    }
                }
                if let Some((row, col)) = ins_loc {
                    clue_rows[row].slots[col] = Some(peg.id);
                }
                picked_peg = None;
            }
        }
        draw_clue_rows(
            &clue_rows,
            mx,
            my,
            picked_peg.map(|p| PEG_COLORS[p.id as usize]),
            peg_tex,
        );
        draw_bottom_pegs(peg_tex);
        if let Some(ref mut peg) = picked_peg {
            peg.x = mx - 32.;
            peg.y = my - 32.;
            draw_peg(peg_tex, *peg);
        }
        ptype_but.draw(mx, my);
        clue_add_but.draw(mx, my);

        next_frame().await
    }
}
