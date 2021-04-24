#![feature(decl_macro)]

use std::collections::HashSet;

use macroquad::prelude::*;
use mmsolv::{solve_raw, Clue, Indicator};

const PEG_SIZE: f32 = 64.0;

macro colors($($r:expr,$g:expr,$b:expr;)*){
    [
        $(
        color_u8!($r, $g, $b, 255),
        )*
    ]
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

mod src_rects {
    use macroquad::prelude::Rect;
    macro rects($($name:ident: $x:expr, $y:expr, $w:expr, $h:expr)+) {
        $(
            pub const $name: Rect = Rect {x: $x as f32, y: $y as f32, w: $w as f32, h: $h as f32};
        )+
    }
    rects! {
        PEG: 64, 0, 58, 58
        DOT: 0, 0, 21, 21
        HEART: 32, 0, 23, 21
    }
}

fn draw_peg(peg_tex: Texture2D, peg: Pegbug) {
    let params = DrawTextureParams {
        source: Some(src_rects::PEG),
        ..Default::default()
    };
    draw_texture_ex(peg_tex, peg.x, peg.y, PEG_COLORS[peg.id as usize], params);
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
struct ClueRow {
    slots: Vec<Option<mmsolv::Peg>>,
    hearts: u8,
    dots: u8,
    dot_add_but: SimpleButton,
    dot_rem_but: SimpleButton,
    heart_add_but: SimpleButton,
    heart_rem_but: SimpleButton,
}

impl ClueRow {
    pub fn new(slots: u8) -> Self {
        Self {
            slots: vec![None; slots as usize],
            hearts: 0,
            dots: 0,
            dot_add_but: SimpleButton::new("+".into(), 0., 0., 24),
            dot_rem_but: SimpleButton::new("-".into(), 0., 0., 24),
            heart_add_but: SimpleButton::new("+".into(), 0., 0., 24),
            heart_rem_but: SimpleButton::new("-".into(), 0., 0., 24),
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

// Also readjusts buttons
fn draw_clue_row(
    row_num: usize,
    row: &mut ClueRow,
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    tex: Texture2D,
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
                tex,
                Pegbug {
                    id: pegid,
                    x: rect.x,
                    y: rect.y,
                },
            );
        }
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, RED);
    }
    let last_rect = clue_rect(row_num, row.slots.len() - 1);
    row.dot_add_but.rect.x = last_rect.x + 8. + BOX_SIZE;
    row.dot_add_but.rect.y = last_rect.y + 8.;
    row.dot_rem_but.rect.x = last_rect.x + 8. + BOX_SIZE + 24. + 2.;
    row.dot_rem_but.rect.y = last_rect.y + 8.;
    row.dot_add_but.draw(mx, my);
    row.dot_rem_but.draw(mx, my);
    row.heart_add_but.rect.x = last_rect.x + 8. + BOX_SIZE;
    row.heart_add_but.rect.y = last_rect.y + 8. + 32.;
    row.heart_rem_but.rect.x = last_rect.x + 8. + BOX_SIZE + 24. + 2.;
    row.heart_rem_but.rect.y = last_rect.y + 8. + 32.;
    row.heart_add_but.draw(mx, my);
    row.heart_rem_but.draw(mx, my);
    for i in 0..row.dots {
        draw_texture_ex(
            tex,
            last_rect.x + 8. + BOX_SIZE + 50. + i as f32 * 24.,
            last_rect.y + 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(src_rects::DOT),
                ..Default::default()
            },
        );
    }
    for i in 0..row.hearts {
        draw_texture_ex(
            tex,
            last_rect.x + 8. + BOX_SIZE + 50. + i as f32 * 24.,
            last_rect.y + 40.0,
            WHITE,
            DrawTextureParams {
                source: Some(src_rects::HEART),
                ..Default::default()
            },
        );
    }
}

fn draw_clue_rows(
    rows: &mut [ClueRow],
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    tex: Texture2D,
) {
    for (i, row) in rows.iter_mut().enumerate() {
        draw_clue_row(i, row, mx, my, picked_color, tex);
    }
}

fn conv_mmsolv(rows: &[ClueRow]) -> Result<(Vec<u8>, Vec<Clue>), String> {
    let mut clues = Vec::new();
    let mut set = HashSet::new();
    for row in rows {
        let clue = Clue {
            indicator: Indicator {
                dots: row.dots,
                hearts: row.hearts,
            },
            pegs: {
                let mut pegs = Vec::new();
                for slot in &row.slots {
                    let &val = match slot {
                        Some(id) => {
                            set.insert(*id);
                            id
                        }
                        None => return Err("Empty slot somewhere".into()),
                    };
                    pegs.push(val);
                }
                pegs.into_boxed_slice()
            },
        };
        clues.push(clue);
    }
    Ok((set.into_iter().collect::<Vec<_>>(), clues))
}

#[macroquad::main("mmsolv")]
async fn main() {
    let mut picked_peg: Option<Pegbug> = None;
    let mut n_pegs_in_clues = IncWrap::new(MIN_PEGS, MAX_PEGS);
    let mut solve_msg: String = String::new();
    macro ptype_but_text() {
        format!("Puzzle type: {} peg", n_pegs_in_clues.value)
    }
    let mut ptype_but = SimpleButton::new(ptype_but_text!(), 8.0, 8.0, 32);
    let clue_add_but = SimpleButton::new("+ row".into(), 8.0, 48.0, 32);
    let clue_rem_but = SimpleButton::new("- row".into(), 148.0, 48.0, 32);
    let solve_but = SimpleButton::new("Solve".into(), 8.0, 96.0, 32);
    let mut clue_rows: Vec<ClueRow> = Vec::new();
    let mut solutions = Vec::new();
    loop {
        clear_background(WHITE);
        let tex =
            Texture2D::from_file_with_format(include_bytes!("../../assets/spritesheet.png"), None);
        let (mx, my) = mouse_position();
        // Handle mouse pressed
        if is_mouse_button_pressed(MouseButton::Left) {
            if ptype_but.mouse_over(mx, my) {
                n_pegs_in_clues.inc();
                for row in &mut clue_rows {
                    row.slots.resize(n_pegs_in_clues.value as usize, None);
                }
                ptype_but.set_text(ptype_but_text!());
            }
            if clue_add_but.mouse_over(mx, my) {
                clue_rows.push(ClueRow::new(n_pegs_in_clues.value));
            }
            if clue_rem_but.mouse_over(mx, my) {
                clue_rows.pop();
            }
            if solve_but.mouse_over(mx, my) {
                match conv_mmsolv(&clue_rows) {
                    Ok((set, clues)) => {
                        solutions = solve_raw(&set, &clues);
                        solve_msg = format!("{} solutions found: ", solutions.len());
                    }
                    Err(e) => {
                        solve_msg = e;
                        solutions.clear();
                    }
                }
            }
            for row in &mut clue_rows {
                if row.dot_add_but.mouse_over(mx, my) {
                    row.dots += 1;
                }
                if row.heart_add_but.mouse_over(mx, my) {
                    row.hearts += 1;
                }
                if row.dot_rem_but.mouse_over(mx, my) && row.dots > 0 {
                    row.dots -= 1;
                }
                if row.heart_rem_but.mouse_over(mx, my) && row.hearts > 0 {
                    row.hearts -= 1;
                }
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
            &mut clue_rows,
            mx,
            my,
            picked_peg.map(|p| PEG_COLORS[p.id as usize]),
            tex,
        );
        draw_bottom_pegs(tex);
        draw_text(&solve_msg, 8., 150., 32., BLACK);
        draw_solutions(&solutions, tex);
        if let Some(ref mut peg) = picked_peg {
            peg.x = mx - 32.;
            peg.y = my - 32.;
            draw_peg(tex, *peg);
        }
        ptype_but.draw(mx, my);
        clue_add_but.draw(mx, my);
        clue_rem_but.draw(mx, my);
        solve_but.draw(mx, my);

        next_frame().await
    }
}

fn draw_solutions(solutions: &[Vec<u8>], peg_tex: Texture2D) {
    for (row, sol) in solutions.iter().enumerate() {
        for (col, peg_id) in sol.iter().enumerate() {
            draw_peg(
                peg_tex,
                Pegbug {
                    x: col as f32 * 68.,
                    y: 180. + row as f32 * 68.,
                    id: *peg_id,
                },
            )
        }
    }
}
