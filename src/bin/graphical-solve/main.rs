#![expect(clippy::cast_precision_loss)]
#![allow(clippy::collapsible_if)]

mod button;
mod color;
mod util;

use util::ValLooper;

use button::{ImgButton, SimpleButton};
use frogquad::{FrogCx, prelude::*};
use miniquad::{BlendFactor, BlendState, BlendValue, Equation};
use mmsolv::{Clue, Indicator, solve_bruteforce_raw};

const PEG_SIZE: f32 = 64.0;

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

const BUGS_PER_ROW: u8 = 3;

fn pickable_peg(idx: u8, y_offset: f32) -> Pegbug {
    Pegbug {
        x: FREE_PEGS_RECT.x + f32::from(idx % BUGS_PER_ROW) * 64.,
        y: FREE_PEGS_RECT.y
            + y_offset
            + FREE_PEGS_RECT.h
            + 8.0
            + f32::from(idx / BUGS_PER_ROW) * PEG_SIZE,
        id: idx,
    }
}

fn pickable_pegs(y_offset: f32, free_pegs: &[u8]) -> impl Iterator<Item = Pegbug> + '_ {
    // `SCHEMES` isn't longer than `u8::MAX`
    #[expect(clippy::cast_possible_truncation)]
    (0..color::SCHEMES.len() as u8).filter_map(move |i| {
        if free_pegs.contains(&i) {
            None
        } else {
            Some(pickable_peg(i, y_offset))
        }
    })
}

mod src_rects {
    use frogquad::prelude::Rect;
    const fn r(x: u8, y: u8, w: u8, h: u8) -> Rect {
        Rect {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
        }
    }
    pub const PEG: Rect = r(64, 0, 64, 64);
    pub const HEART: Rect = r(0, 0, 21, 21);
    pub const DOT: Rect = r(32, 0, 23, 21);
    pub const PLUS: Rect = r(144, 0, 24, 24);
    pub const MINUS: Rect = r(168, 0, 24, 24);
}

fn draw_peg(fc: &mut FrogCx, peg_tex: &Texture2D, peg: Pegbug, mat: &Material) {
    let params = DrawTextureParams {
        source: Some(src_rects::PEG),
        ..Default::default()
    };
    let color::Scheme {
        skin,
        eyes,
        eyes_shine,
    } = color::SCHEMES[peg.id as usize];
    mat.set_uniform_array(&mut fc.gl, "r_body", &skin);
    mat.set_uniform_array(&mut fc.gl, "r_eye", &eyes);
    mat.set_uniform_array(&mut fc.gl, "r_eyeshine", &eyes_shine);
    gl_use_material(&mut fc.gl, mat);
    draw_texture_ex(fc, peg_tex, peg.x, peg.y, WHITE, params);
    gl_use_default_material(&mut fc.gl);
}

fn draw_pickable_pegs(
    fc: &mut FrogCx,
    peg_tex: &Texture2D,
    y_offset: f32,
    mat: &Material,
    free_pegs: &[u8],
) {
    pickable_pegs(y_offset, free_pegs).for_each(|peg| {
        draw_peg(fc, peg_tex, peg, mat);
    });
}
struct ClueRow {
    slots: Vec<Option<mmsolv::Peg>>,
    hearts: u8,
    dots: u8,
    dot_add_but: ImgButton,
    dot_rem_but: ImgButton,
    heart_add_but: ImgButton,
    heart_rem_but: ImgButton,
}

const HEART_BUT_DOWN_COL: Color = Color {
    r: 1.0,
    g: 0.4,
    b: 0.4,
    a: 1.0,
};

impl ClueRow {
    pub fn new(slots: u8) -> Self {
        Self {
            slots: vec![None; slots as usize],
            hearts: 0,
            dots: 0,
            dot_add_but: ImgButton::new(src_rects::PLUS, 0., 0., GRAY, LIGHTGRAY),
            dot_rem_but: ImgButton::new(src_rects::MINUS, 0., 0., GRAY, LIGHTGRAY),
            heart_add_but: ImgButton::new(src_rects::PLUS, 0., 0., RED, HEART_BUT_DOWN_COL),
            heart_rem_but: ImgButton::new(src_rects::MINUS, 0., 0., RED, HEART_BUT_DOWN_COL),
        }
    }

    fn full(&self) -> bool {
        self.slots.iter().all(Option::is_some)
    }
}

const CLUE_ROW_X_OFFSET: f32 = 232.;
const CLUE_ROW_Y_OFFSET: f32 = 16.;
const BOX_PADDING_INNER: f32 = 4.;
const BOX_SIZE: f32 = PEG_SIZE + BOX_PADDING_INNER;
const BOX_VERT_DISTANCE: f32 = 8.;
const BOX_HORIZ_DISTANCE: f32 = 8.;

const SEVEN_OFFSETS: [(f32, f32); 7] = [
    (0.5, 0.0),
    (1.5, 0.0),
    (0.0, 1.0),
    (1.0, 1.0),
    (2.0, 1.0),
    (0.5, 2.0),
    (1.5, 2.0),
];

fn clue_rect(row: usize, col: usize, seven_peg: bool, y_scroll_offset: f32) -> Rect {
    if seven_peg {
        const SEVEN_PEG_PADDING: f32 = 8.0;
        Rect {
            x: CLUE_ROW_X_OFFSET + SEVEN_OFFSETS[col].0 * (BOX_SIZE + BOX_HORIZ_DISTANCE),
            y: CLUE_ROW_Y_OFFSET
                + row as f32 * ((BOX_SIZE + BOX_VERT_DISTANCE + SEVEN_PEG_PADDING) * 3.)
                + (SEVEN_OFFSETS[col].1 * (BOX_SIZE + BOX_VERT_DISTANCE))
                + y_scroll_offset,
            w: BOX_SIZE,
            h: BOX_SIZE,
        }
    } else {
        Rect {
            x: CLUE_ROW_X_OFFSET + col as f32 * (BOX_SIZE + BOX_HORIZ_DISTANCE),
            y: CLUE_ROW_Y_OFFSET + row as f32 * (BOX_SIZE + BOX_VERT_DISTANCE) + y_scroll_offset,
            w: BOX_SIZE,
            h: BOX_SIZE,
        }
    }
}

fn clue_rects(
    rows: &[ClueRow],
    seven_peg: bool,
    y_scroll_offset: f32,
) -> impl Iterator<Item = (Rect, usize, usize)> + '_ {
    rows.iter().enumerate().flat_map(move |(row_num, row)| {
        row.slots.iter().enumerate().map(move |(col, _)| {
            (
                clue_rect(row_num, col, seven_peg, y_scroll_offset),
                row_num,
                col,
            )
        })
    })
}

// Also readjusts buttons
#[expect(clippy::too_many_arguments)]
fn draw_clue_row(
    fc: &mut FrogCx,
    row_num: usize,
    row: &mut ClueRow,
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    tex: &Texture2D,
    seven_peg: bool,
    y_scroll_offset: f32,
    mat: &Material,
) {
    for (i, slot) in row.slots.iter().enumerate() {
        let rect = clue_rect(row_num, i, seven_peg, y_scroll_offset);
        if let Some(picked_color) = picked_color {
            if rect.contains(Vec2::new(mx, my)) {
                draw_rectangle(fc, rect.x, rect.y, rect.w, rect.h, picked_color);
            }
        }
        if let Some(pegid) = *slot {
            draw_peg(
                fc,
                tex,
                Pegbug {
                    id: pegid,
                    x: rect.x + 5.0,
                    y: rect.y + 5.0,
                },
                mat,
            );
        }
        draw_rectangle_lines(fc, rect.x, rect.y, rect.w, rect.h, 2.0, RED);
    }
    let last_rect_idx = if seven_peg { 1 } else { row.slots.len() - 1 };
    let last_rect = clue_rect(row_num, last_rect_idx, seven_peg, y_scroll_offset);
    row.heart_add_but.rect.x = last_rect.x + 4. + BOX_SIZE;
    row.heart_add_but.rect.y = last_rect.y + 4.;
    row.heart_rem_but.rect.x = last_rect.x + 4. + BOX_SIZE + 32.;
    row.heart_rem_but.rect.y = last_rect.y + 4.;
    row.heart_add_but.draw(fc, tex, mx, my);
    row.heart_rem_but.draw(fc, tex, mx, my);
    row.dot_add_but.rect.x = last_rect.x + 4. + BOX_SIZE;
    row.dot_add_but.rect.y = last_rect.y + 4. + 32.;
    row.dot_rem_but.rect.x = last_rect.x + 4. + BOX_SIZE + 32.;
    row.dot_rem_but.rect.y = last_rect.y + 4. + 32.;
    row.dot_add_but.draw(fc, tex, mx, my);
    row.dot_rem_but.draw(fc, tex, mx, my);
    for i in 0..row.hearts {
        draw_texture_ex(
            fc,
            tex,
            last_rect.x + 16. + BOX_SIZE + 50. + f32::from(i) * 24.,
            last_rect.y + 8.0,
            WHITE,
            DrawTextureParams {
                source: Some(src_rects::HEART),
                ..Default::default()
            },
        );
    }
    for i in 0..row.dots {
        draw_texture_ex(
            fc,
            tex,
            last_rect.x + 16. + BOX_SIZE + 50. + f32::from(i) * 24.,
            last_rect.y + 40.0,
            WHITE,
            DrawTextureParams {
                source: Some(src_rects::DOT),
                ..Default::default()
            },
        );
    }
}

#[expect(clippy::too_many_arguments)]
fn draw_clue_rows(
    fc: &mut FrogCx,
    rows: &mut [ClueRow],
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    tex: &Texture2D,
    seven_peg: bool,
    y_scroll_offset: f32,
    mat: &Material,
) {
    for (i, row) in rows.iter_mut().enumerate() {
        draw_clue_row(
            fc,
            i,
            row,
            mx,
            my,
            picked_color,
            tex,
            seven_peg,
            y_scroll_offset,
            mat,
        );
    }
}

fn conv_mmsolv(rows: &[ClueRow]) -> Result<Vec<Clue>, String> {
    let mut clues = Vec::new();
    for row in rows {
        let clue = Clue {
            indicator: Indicator {
                dots: row.dots,
                hearts: row.hearts,
            },
            pegs: {
                let mut pegs = Vec::new();
                for slot in &row.slots {
                    let Some(val) = slot else {
                        return Err("Empty slot somewhere".into());
                    };
                    pegs.push(*val);
                }
                pegs.into_boxed_slice()
            },
        };
        clues.push(clue);
    }
    Ok(clues)
}

fn repos_solve_but(but: &mut SimpleButton, bottom_rect: Rect) {
    but.rect.x = bottom_rect.x;
    but.rect.y = bottom_rect.y + 82.0;
}

const MAIN_AREA_START_X: f32 = CLUE_ROW_X_OFFSET - 8.0;
const MAX_SOLUTIONS: usize = 99;

type ClueRowsSlice = [ClueRow];

trait ClueRowsSliceExt {
    fn full(&self) -> bool;
}

impl ClueRowsSliceExt for ClueRowsSlice {
    fn full(&self) -> bool {
        self.iter().all(ClueRow::full)
    }
}

fn first_empty_clue(rows: &mut ClueRowsSlice) -> Option<&mut Option<mmsolv::Peg>> {
    rows.iter_mut()
        .find_map(|row| row.slots.iter_mut().find(|slot| slot.is_none()))
}

struct SolveApp {
    main_y_scroll_offset: f32,
    left_y_scroll_offset: f32,
    ptype_but: SimpleButton,
    n_pegs_in_clues: ValLooper<'static>,
    solve_msg: String,
    solutions: Vec<Vec<u8>>,
    clue_add_but: ImgButton,
    clue_rows: Vec<ClueRow>,
    clue_rem_but: ImgButton,
    top_but: SimpleButton,
    clear_but: SimpleButton,
    solve_but: SimpleButton,
    free_pegs: Vec<u8>,
    picked_peg: Option<Pegbug>,
    view_drag_center_y: Option<f32>,
    stored_main_y_scroll_offset: f32,
    left_drag_center_y: Option<f32>,
    stored_left_y_scroll_offset: f32,
    tex: Texture2D,
    mat: Material,
}
impl SolveApp {
    fn new(fc: &mut frogquad::FrogCx) -> Self {
        let picked_peg = None;
        let n_pegs_in_clues = ValLooper::new(&[3, 4, 5, 7]);
        let solve_msg = String::new();
        let ptype_but = SimpleButton::new(
            fc,
            format!("Type: {} peg", n_pegs_in_clues.value()),
            8.0,
            8.0,
            32,
        );
        let clue_add_but = ImgButton::new(src_rects::PLUS, 110.0, 44.0, GRAY, LIGHTGRAY);
        let clue_rem_but = ImgButton::new(src_rects::MINUS, 140.0, 44.0, GRAY, LIGHTGRAY);
        let solve_but = SimpleButton::new(fc, "Solve".into(), 8.0, 96.0, 32);
        let clear_but = SimpleButton::new(fc, "X".into(), 184.0, 44.0, 40);
        let top_but = SimpleButton::new(fc, "^".into(), 184.0, 8.0, 40);
        let clue_rows = vec![ClueRow::new(n_pegs_in_clues.value())];
        let solutions = Vec::new();
        let free_pegs = Vec::new();
        let main_y_scroll_offset = 0.0;
        let stored_main_y_scroll_offset = 0.0;
        let left_y_scroll_offset = 0.0;
        let stored_left_y_scroll_offset = 0.0;
        let view_drag_center_y = None;
        let left_drag_center_y = None;
        let tex = Texture2D::from_file_with_format(
            fc,
            include_bytes!("../../../assets/spritesheet.png"),
            None,
        );
        let mat = load_material(
            fc,
            ShaderSource::Glsl {
                vertex: include_str!("../../../assets/vertex_shader.glsl"),
                fragment: include_str!("../../../assets/color_shader.glsl"),
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc {
                        name: "r_body".into(),
                        uniform_type: UniformType::Float3,
                        array_count: 1,
                    },
                    UniformDesc {
                        name: "r_eye".into(),
                        uniform_type: UniformType::Float3,
                        array_count: 1,
                    },
                    UniformDesc {
                        name: "r_eyeshine".into(),
                        uniform_type: UniformType::Float3,
                        array_count: 1,
                    },
                ],
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();
        Self {
            main_y_scroll_offset,
            left_y_scroll_offset,
            ptype_but,
            n_pegs_in_clues,
            solve_msg,
            solutions,
            clue_add_but,
            clue_rows,
            clue_rem_but,
            top_but,
            clear_but,
            solve_but,
            free_pegs,
            picked_peg,
            view_drag_center_y,
            stored_main_y_scroll_offset,
            left_drag_center_y,
            stored_left_y_scroll_offset,
            tex,
            mat,
        }
    }
}

impl frogquad::App for SolveApp {
    #![expect(clippy::too_many_lines)]
    fn update(&mut self, fc: &mut frogquad::FrogCx) {
        clear_background(fc, WHITE);
        let (mx, my) = mouse_position(fc);

        let (_, mw_y) = mouse_wheel(fc);
        let offs = mw_y * 32.0;
        if mx > MAIN_AREA_START_X {
            self.main_y_scroll_offset += offs;
        } else {
            self.left_y_scroll_offset += offs;
        }
        // Handle mouse pressed
        if is_mouse_button_pressed(fc, MouseButton::Left) {
            let mut clicked_something = false;
            if self.ptype_but.mouse_over(mx, my) {
                self.n_pegs_in_clues.go_next();
                for row in &mut self.clue_rows {
                    row.slots
                        .resize(self.n_pegs_in_clues.value() as usize, None);
                }
                self.ptype_but
                    .set_text(fc, format!("Type: {} peg", self.n_pegs_in_clues.value()));
                self.solutions.clear();
                self.solve_msg.clear();
                clicked_something = true;
            } else if self.clue_add_but.mouse_over(mx, my) {
                self.clue_rows
                    .push(ClueRow::new(self.n_pegs_in_clues.value()));
                clicked_something = true;
            } else if self.clue_rem_but.mouse_over(mx, my) && self.clue_rows.len() > 1 {
                self.clue_rows.pop();
                clicked_something = true;
            } else if self.top_but.mouse_over(mx, my) {
                self.main_y_scroll_offset = 0.0;
            } else if self.clear_but.mouse_over(mx, my) {
                self.clue_rows.truncate(1);
                let row = &mut self.clue_rows[0];
                row.dots = 0;
                row.hearts = 0;
                row.slots.iter_mut().for_each(|slot| *slot = None);
                self.solutions.clear();
                self.free_pegs.clear();
                self.solve_msg.clear();
                clicked_something = true;
                self.main_y_scroll_offset = 0.0;
            } else if self.solve_but.mouse_over(mx, my) {
                match conv_mmsolv(&self.clue_rows) {
                    Ok(clues) => {
                        self.solutions = solve_bruteforce_raw(&self.free_pegs, &clues)
                            .take(MAX_SOLUTIONS)
                            .collect();
                        let len_s;
                        self.solve_msg = format!(
                            "{} solution{}",
                            if self.solutions.is_empty() {
                                "No"
                            } else {
                                len_s = format!(
                                    "{}{}",
                                    self.solutions.len(),
                                    if self.solutions.len() == 99 {
                                        "(+)"
                                    } else {
                                        ""
                                    }
                                );
                                &len_s
                            },
                            if self.solutions.len() > 1 { "s" } else { "" }
                        );
                    }
                    Err(e) => {
                        self.solve_msg = e;
                        self.solutions.clear();
                    }
                }
                clicked_something = true;
            }
            for row in &mut self.clue_rows {
                if row.dot_add_but.mouse_over(mx, my) && row.dots < self.n_pegs_in_clues.value() {
                    row.dots += 1;
                    clicked_something = true;
                }
                if row.heart_add_but.mouse_over(mx, my) && row.hearts < self.n_pegs_in_clues.value()
                {
                    row.hearts += 1;
                    clicked_something = true;
                }
                if row.dot_rem_but.mouse_over(mx, my) && row.dots > 0 {
                    row.dots -= 1;
                    clicked_something = true;
                }
                if row.heart_rem_but.mouse_over(mx, my) && row.hearts > 0 {
                    row.hearts -= 1;
                    clicked_something = true;
                }
            }
            if self.picked_peg.is_none() {
                if my > FREE_PEGS_RECT.y + FREE_PEGS_RECT.h {
                    for peg in pickable_pegs(self.left_y_scroll_offset, &self.free_pegs) {
                        if peg.rect().contains(Vec2::new(mx, my)) {
                            self.picked_peg = Some(peg);
                            clicked_something = true;
                        }
                    }
                }
                let mut rem = None;
                for (clue_rect, row, col) in clue_rects(
                    &self.clue_rows,
                    self.n_pegs_in_clues.value() == 7,
                    self.main_y_scroll_offset,
                ) {
                    if clue_rect.contains(Vec2::new(mx, my)) {
                        self.picked_peg = match self.clue_rows.get(row) {
                            Some(clue_row) => match clue_row.slots.get(col) {
                                Some(Some(id)) => {
                                    if !is_key_down(fc, KeyCode::LeftControl) {
                                        rem = Some((row, col));
                                    }
                                    clicked_something = true;
                                    Some(Pegbug {
                                        x: 0.,
                                        y: 0.,
                                        id: *id,
                                    })
                                }
                                _ => None,
                            },
                            None => None,
                        };
                        break;
                    }
                }
                if let Some((row, col)) = rem {
                    self.clue_rows[row].slots[col] = None;
                }
                let mut rem = None;
                for (i, peg) in crate::free_pegs(&self.free_pegs) {
                    if peg.rect().contains(Vec2::new(mx, my)) {
                        clicked_something = true;
                        self.picked_peg = Some(peg);
                        rem = Some(i);
                    }
                }
                if let Some(idx) = rem {
                    self.free_pegs.remove(idx);
                }
                if !clicked_something {
                    if mx > MAIN_AREA_START_X {
                        self.view_drag_center_y = Some(my);
                        self.stored_main_y_scroll_offset = self.main_y_scroll_offset;
                    } else {
                        self.left_drag_center_y = Some(my);
                        self.stored_left_y_scroll_offset = self.left_y_scroll_offset;
                    }
                }
            }
        }
        if is_mouse_button_pressed(fc, MouseButton::Right)
            || is_mouse_button_pressed(fc, MouseButton::Middle)
        {
            // Convenience add empty row if rows are full
            if self.clue_rows.full() {
                self.clue_rows
                    .push(ClueRow::new(self.n_pegs_in_clues.value()));
            }
            for peg in pickable_pegs(self.left_y_scroll_offset, &self.free_pegs) {
                if peg.rect().contains(Vec2::new(mx, my)) {
                    if let Some(empty) = first_empty_clue(&mut self.clue_rows) {
                        *empty = Some(peg.id);
                    }
                }
            }
        }
        if let Some(view_drag_center_y_val) = self.view_drag_center_y {
            self.main_y_scroll_offset =
                self.stored_main_y_scroll_offset - (view_drag_center_y_val - my);
        }
        if let Some(left_drag_center_y_val) = self.left_drag_center_y {
            self.left_y_scroll_offset =
                self.stored_left_y_scroll_offset - (left_drag_center_y_val - my);
        }
        if self.main_y_scroll_offset > 0.0 {
            self.main_y_scroll_offset = 0.0;
        }
        if self.left_y_scroll_offset > 0.0 {
            self.left_y_scroll_offset = 0.0;
        }
        let min_y_scroll_left = -((PEG_SIZE + 1.0) * 4.0);
        if self.left_y_scroll_offset < min_y_scroll_left {
            self.left_y_scroll_offset = min_y_scroll_left;
        }
        draw_vert_scroll_bar(
            fc,
            MAIN_AREA_START_X - 12.,
            FREE_PEGS_RECT.y + FREE_PEGS_RECT.h + 14.0,
            screen_height(fc),
            self.left_y_scroll_offset,
            min_y_scroll_left,
        );
        draw_vert_scroll_bar(
            fc,
            screen_width(fc) - 12.0,
            12.0,
            screen_height(fc),
            -self.main_y_scroll_offset,
            9000.0,
        );
        let can_drop_free = if let Some(peg) = self.picked_peg {
            FREE_PEGS_RECT.contains(Vec2::new(mx, my))
                && self.free_pegs.len() < 6
                && !self.free_pegs.contains(&peg.id)
                && !self
                    .clue_rows
                    .iter()
                    .any(|row| row.slots.contains(&Some(peg.id)))
        } else {
            false
        };
        macro_rules! rect_for_solve_button {
            () => {{
                let idx = if self.n_pegs_in_clues.value() == 7 {
                    5
                } else {
                    0
                };
                clue_rect(
                    self.clue_rows.len() - 1,
                    idx,
                    self.n_pegs_in_clues.value() == 7,
                    self.main_y_scroll_offset,
                )
            }};
        }
        if is_mouse_button_released(fc, MouseButton::Left) {
            self.view_drag_center_y = None;
            self.left_drag_center_y = None;
            if let Some(peg) = self.picked_peg {
                let mut ins_loc = None;
                for (clue_rect, row, col) in clue_rects(
                    &self.clue_rows,
                    self.n_pegs_in_clues.value() == 7,
                    self.main_y_scroll_offset,
                ) {
                    if clue_rect.contains(Vec2::new(mx, my)) {
                        ins_loc = Some((row, col));
                        break;
                    }
                }
                if let Some((row, col)) = ins_loc {
                    self.clue_rows[row].slots[col] = Some(peg.id);
                }
                if can_drop_free {
                    self.free_pegs.push(peg.id);
                }
                self.picked_peg = None;
            }
        }
        draw_clue_rows(
            fc,
            &mut self.clue_rows,
            mx,
            my,
            self.picked_peg
                .map(|p| color::SCHEMES[p.id as usize].skin_color()),
            &self.tex,
            self.n_pegs_in_clues.value() == 7,
            self.main_y_scroll_offset,
            &self.mat,
        );
        draw_pickable_pegs(
            fc,
            &self.tex,
            self.left_y_scroll_offset,
            &self.mat,
            &self.free_pegs,
        );
        draw_rectangle(
            fc,
            0.0,
            0.0,
            FREE_PEGS_RECT.x + FREE_PEGS_RECT.w,
            FREE_PEGS_RECT.y + FREE_PEGS_RECT.h,
            WHITE,
        );
        draw_solutions(
            fc,
            &self.solutions,
            &self.tex,
            rect_for_solve_button!(),
            self.n_pegs_in_clues.value() == 7,
            &self.mat,
        );
        draw_free_pegs(
            fc,
            &self.tex,
            &self.free_pegs,
            mx,
            my,
            if can_drop_free {
                self.picked_peg
                    .map(|p| color::SCHEMES[p.id as usize].skin_color())
            } else {
                None
            },
            &self.mat,
        );
        if let Some(ref mut peg) = self.picked_peg {
            peg.x = mx - 32.;
            peg.y = my - 32.;
            draw_peg(fc, &self.tex, *peg, &self.mat);
        }
        self.ptype_but.draw(fc, mx, my);
        draw_text(
            fc,
            format!("{} rows", self.clue_rows.len()),
            8.0,
            64.0,
            32.0,
            BLACK,
        );
        draw_text(
            fc,
            "Free pegs",
            FREE_PEGS_RECT.x + 4.0,
            FREE_PEGS_RECT.y + 24.0,
            32.0,
            BLACK,
        );
        self.clue_add_but.draw(fc, &self.tex, mx, my);
        self.clue_rem_but.draw(fc, &self.tex, mx, my);
        repos_solve_but(&mut self.solve_but, rect_for_solve_button!());
        self.solve_but.draw(fc, mx, my);
        self.top_but.draw(fc, mx, my);
        self.clear_but.draw(fc, mx, my);
        draw_text(
            fc,
            &self.solve_msg,
            self.solve_but.rect.x + self.solve_but.rect.w + 8.0,
            self.solve_but.rect.y + 20.0,
            32.,
            BLACK,
        );
        draw_line(
            fc,
            MAIN_AREA_START_X,
            0.0,
            MAIN_AREA_START_X,
            screen_height(fc),
            2.0,
            BLACK,
        );
    }
}

fn main() {
    frogquad::start(Conf::from_title("mmsolv"), |fc| Box::new(SolveApp::new(fc)));
}

fn draw_vert_scroll_bar(
    fc: &mut FrogCx,
    x: f32,
    start_y: f32,
    end_y: f32,
    scroll: f32,
    max_scroll: f32,
) {
    let radius = 32.0;
    let ratio = scroll / max_scroll;
    let y = ratio * ((end_y - radius / 2.0) - start_y);
    draw_circle(fc, x, start_y + y, 8.0, BLUE);
}

fn free_pegs(pegs: &[u8]) -> impl Iterator<Item = (usize, Pegbug)> + '_ {
    const TEXT_OFFSET: f32 = 24.0;
    pegs.iter().enumerate().map(|(i, &peg_id)| {
        (
            i,
            Pegbug {
                x: 12.0 + (i % FREE_PEGS_MAX_PER_ROW as usize) as f32 * PEG_SIZE,
                y: TEXT_OFFSET
                    + FREE_PEGS_RECT.y
                    + 8.0
                    + (i / FREE_PEGS_MAX_PER_ROW as usize) as f32 * PEG_SIZE,
                id: peg_id,
            },
        )
    })
}

const FREE_PEGS_RECT: Rect = Rect {
    x: 8.0,
    y: 80.0,
    w: 194.0,
    h: 168.0,
};

const FREE_PEGS_MAX_PER_ROW: u8 = 3;

fn draw_free_pegs(
    fc: &mut FrogCx,
    peg_tex: &Texture2D,
    free_pegs: &[u8],
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    mat: &Material,
) {
    if let Some(c) = picked_color {
        if FREE_PEGS_RECT.contains(Vec2::new(mx, my)) {
            draw_rectangle(
                fc,
                FREE_PEGS_RECT.x,
                FREE_PEGS_RECT.y,
                FREE_PEGS_RECT.w,
                FREE_PEGS_RECT.h,
                c,
            );
        }
    }
    draw_rectangle_lines(
        fc,
        FREE_PEGS_RECT.x,
        FREE_PEGS_RECT.y,
        FREE_PEGS_RECT.w,
        FREE_PEGS_RECT.h,
        2.0,
        GREEN,
    );

    for (_, peg) in crate::free_pegs(free_pegs) {
        draw_peg(fc, peg_tex, peg, mat);
    }
}

fn draw_solutions(
    fc: &mut FrogCx,
    solutions: &[Vec<u8>],
    peg_tex: &Texture2D,
    bottom_rect: Rect,
    seven_peg: bool,
    mat: &Material,
) {
    for (row, sol) in solutions.iter().enumerate() {
        for (col, peg_id) in sol.iter().enumerate() {
            let x = bottom_rect.x
                + if seven_peg {
                    SEVEN_OFFSETS[col].0 * 68.
                } else {
                    col as f32 * 68.
                };
            let y = bottom_rect.y
                + 120.
                + if seven_peg {
                    let padding_between_solutions = 24.0;
                    row as f32 * (68. * 3. + padding_between_solutions) + SEVEN_OFFSETS[col].1 * 68.
                } else {
                    row as f32 * 68.
                };
            draw_peg(fc, peg_tex, Pegbug { x, y, id: *peg_id }, mat);
        }
    }
}
