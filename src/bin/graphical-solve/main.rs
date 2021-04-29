#![feature(decl_macro)]
#![allow(clippy::too_many_arguments)]

mod color;
mod util;

use std::collections::HashSet;
use util::ValLooper;

use macroquad::prelude::*;
use miniquad::{BlendFactor, BlendState, BlendValue, Equation};
use mmsolv::{solve_raw, Clue, Indicator};

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

fn pickable_peg(idx: usize, y_offset: f32) -> Pegbug {
    Pegbug {
        x: FREE_PEGS_RECT.x + (idx % BUGS_PER_ROW as usize) as f32 * 64.,
        y: FREE_PEGS_RECT.y
            + y_offset
            + FREE_PEGS_RECT.h
            + 8.0
            + (idx / BUGS_PER_ROW as usize) as f32 * PEG_SIZE,
        id: idx as u8,
    }
}

fn pickable_pegs(y_offset: f32) -> impl Iterator<Item = Pegbug> {
    (0..color::SCHEMES.len()).map(move |i| pickable_peg(i, y_offset))
}

mod src_rects {
    use macroquad::prelude::Rect;
    macro rects($($name:ident: $x:expr, $y:expr, $w:expr, $h:expr)+) {
        $(
            pub const $name: Rect = Rect {x: $x as f32, y: $y as f32, w: $w as f32, h: $h as f32};
        )+
    }
    rects! {
        PEG: 64, 0, 64, 64
        HEART: 0, 0, 21, 21
        DOT: 32, 0, 23, 21
        PLUS: 144, 0, 24, 24
        MINUS: 168, 0, 24, 24
    }
}

fn draw_peg(peg_tex: Texture2D, peg: Pegbug, mat: &[Material]) {
    let params = DrawTextureParams {
        source: Some(src_rects::PEG),
        ..Default::default()
    };
    // FIXME: Workaround for white texture bug. Draw a texture without a material first.
    draw_texture_ex(peg_tex, peg.x, peg.y, WHITE, params.clone());
    gl_use_material(mat[peg.id as usize]);
    draw_texture_ex(peg_tex, peg.x, peg.y, WHITE, params);
    gl_use_default_material();
}

fn draw_pickable_pegs(peg_tex: Texture2D, y_offset: f32, mat: &[Material]) {
    pickable_pegs(y_offset).for_each(|peg| {
        draw_peg(peg_tex, peg, mat);
    });
}

struct SimpleButton {
    rect: Rect,
    text: String,
    font_size: u16,
    text_offset_y: f32,
}

struct ImgButton {
    img_src_rect: Rect,
    rect: Rect,
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
    pub fn draw(&self, tex: Texture2D, mx: f32, my: f32) {
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
            self.font_size as f32,
            BLACK,
        );
    }
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
fn draw_clue_row(
    row_num: usize,
    row: &mut ClueRow,
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    tex: Texture2D,
    seven_peg: bool,
    y_scroll_offset: f32,
    mat: &[Material],
) {
    for (i, slot) in row.slots.iter().enumerate() {
        let rect = clue_rect(row_num, i, seven_peg, y_scroll_offset);
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
                    x: rect.x + 5.0,
                    y: rect.y + 5.0,
                },
                mat,
            );
        }
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, RED);
    }
    let last_rect_idx = if seven_peg { 1 } else { row.slots.len() - 1 };
    let last_rect = clue_rect(row_num, last_rect_idx, seven_peg, y_scroll_offset);
    row.heart_add_but.rect.x = last_rect.x + 4. + BOX_SIZE;
    row.heart_add_but.rect.y = last_rect.y + 4.;
    row.heart_rem_but.rect.x = last_rect.x + 4. + BOX_SIZE + 32.;
    row.heart_rem_but.rect.y = last_rect.y + 4.;
    row.heart_add_but.draw(tex, mx, my);
    row.heart_rem_but.draw(tex, mx, my);
    row.dot_add_but.rect.x = last_rect.x + 4. + BOX_SIZE;
    row.dot_add_but.rect.y = last_rect.y + 4. + 32.;
    row.dot_rem_but.rect.x = last_rect.x + 4. + BOX_SIZE + 32.;
    row.dot_rem_but.rect.y = last_rect.y + 4. + 32.;
    row.dot_add_but.draw(tex, mx, my);
    row.dot_rem_but.draw(tex, mx, my);
    for i in 0..row.hearts {
        draw_texture_ex(
            tex,
            last_rect.x + 16. + BOX_SIZE + 50. + i as f32 * 24.,
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
            tex,
            last_rect.x + 16. + BOX_SIZE + 50. + i as f32 * 24.,
            last_rect.y + 40.0,
            WHITE,
            DrawTextureParams {
                source: Some(src_rects::DOT),
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
    seven_peg: bool,
    y_scroll_offset: f32,
    mat: &[Material],
) {
    for (i, row) in rows.iter_mut().enumerate() {
        draw_clue_row(
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

fn repos_solve_but(but: &mut SimpleButton, bottom_rect: Rect) {
    but.rect.x = bottom_rect.x;
    but.rect.y = bottom_rect.y + 82.0;
}

const MAIN_AREA_START_X: f32 = CLUE_ROW_X_OFFSET - 8.0;
const MAX_SOLUTIONS: usize = 99;

#[macroquad::main("mmsolv")]
async fn main() {
    let mut picked_peg: Option<Pegbug> = None;
    let mut n_pegs_in_clues = ValLooper::new(&[3, 4, 5, 7]);
    let mut solve_msg: String = String::new();
    macro ptype_but_text() {
        format!("Type: {} peg", n_pegs_in_clues.value())
    }
    let mut ptype_but = SimpleButton::new(ptype_but_text!(), 8.0, 8.0, 32);
    let clue_add_but = ImgButton::new(src_rects::PLUS, 110.0, 44.0, GRAY, LIGHTGRAY);
    let clue_rem_but = ImgButton::new(src_rects::MINUS, 140.0, 44.0, GRAY, LIGHTGRAY);
    let mut solve_but = SimpleButton::new("Solve".into(), 8.0, 96.0, 32);
    let mut clue_rows: Vec<ClueRow> = vec![ClueRow::new(n_pegs_in_clues.value())];
    let mut solutions = Vec::new();
    let mut free_pegs = Vec::new();
    let mut main_y_scroll_offset: f32 = 0.0;
    let mut stored_main_y_scroll_offset: f32 = 0.0;
    let mut left_y_scroll_offset: f32 = 0.0;
    let mut stored_left_y_scroll_offset: f32 = 0.0;
    macro rect_for_solve_button() {{
        let idx = if n_pegs_in_clues.value() == 7 { 5 } else { 0 };
        clue_rect(
            clue_rows.len() - 1,
            idx,
            n_pegs_in_clues.value() == 7,
            main_y_scroll_offset,
        )
    }}
    let mut view_drag_center_y = None;
    let mut left_drag_center_y = None;
    let tex =
        Texture2D::from_file_with_format(include_bytes!("../../../assets/spritesheet.png"), None);
    let mut peg_materials = Vec::new();
    for i in 0..15 {
        let mat = load_material(
            r#"#version 100
        attribute vec3 position;
        attribute vec2 texcoord;
        attribute vec4 color0;
        varying lowp vec2 uv;
        varying lowp vec4 color;
        uniform mat4 Model;
        uniform mat4 Projection;
        void main() {
            gl_Position = Projection * Model * vec4(position, 1);
            color = color0 / 255.0;
            uv = texcoord;
        }"#,
            include_str!("../../../assets/color_shader.glsl"),
            MaterialParams {
                uniforms: vec![
                    ("c_body".into(), UniformType::Float3),
                    ("r_body".into(), UniformType::Float3),
                    ("c_eye".into(), UniformType::Float3),
                    ("r_eye".into(), UniformType::Float3),
                    ("c_eyeshine".into(), UniformType::Float3),
                    ("r_eyeshine".into(), UniformType::Float3),
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
        mat.set_uniform("c_body", [1.0f32, 0.0, 0.0]);
        mat.set_uniform("c_eye", [0.0f32, 1.0, 0.0]);
        mat.set_uniform("c_eyeshine", [0.0f32, 0.0, 1.0]);
        let [body, eye, shine] = color::SCHEMES[i].to_rgb();
        mat.set_uniform("r_body", body);
        mat.set_uniform("r_eye", eye);
        mat.set_uniform("r_eyeshine", shine);
        peg_materials.push(mat);
    }

    loop {
        clear_background(WHITE);
        let (mx, my) = mouse_position();

        let (_, mw_y) = mouse_wheel();
        let offs = mw_y * 32.0;
        if mx > MAIN_AREA_START_X {
            main_y_scroll_offset += offs;
        } else {
            left_y_scroll_offset += offs;
        }
        // Handle mouse pressed
        if is_mouse_button_pressed(MouseButton::Left) {
            let mut clicked_something = false;
            if ptype_but.mouse_over(mx, my) {
                n_pegs_in_clues.go_next();
                for row in &mut clue_rows {
                    row.slots.resize(n_pegs_in_clues.value() as usize, None);
                }
                ptype_but.set_text(ptype_but_text!());
                clicked_something = true;
            } else if clue_add_but.mouse_over(mx, my) {
                clue_rows.push(ClueRow::new(n_pegs_in_clues.value()));
                clicked_something = true;
            } else if clue_rem_but.mouse_over(mx, my) && clue_rows.len() > 1 {
                clue_rows.pop();
                clicked_something = true;
            } else if solve_but.mouse_over(mx, my) {
                match conv_mmsolv(&clue_rows) {
                    Ok((mut set, clues)) => {
                        set.extend_from_slice(&free_pegs);
                        solutions = solve_raw(&set, &clues).take(MAX_SOLUTIONS).collect();
                        solve_msg = format!("{} solutions found: ", solutions.len());
                    }
                    Err(e) => {
                        solve_msg = e;
                        solutions.clear();
                    }
                }
                clicked_something = true;
            }
            for row in &mut clue_rows {
                if row.dot_add_but.mouse_over(mx, my) && row.dots < n_pegs_in_clues.value() {
                    row.dots += 1;
                    clicked_something = true;
                }
                if row.heart_add_but.mouse_over(mx, my) && row.hearts < n_pegs_in_clues.value() {
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
            if picked_peg.is_none() {
                if my > FREE_PEGS_RECT.y + FREE_PEGS_RECT.h {
                    for peg in pickable_pegs(left_y_scroll_offset) {
                        if peg.rect().contains(Vec2::new(mx, my)) {
                            picked_peg = Some(peg);
                            clicked_something = true;
                        }
                    }
                }
                let mut rem = None;
                for (clue_rect, row, col) in clue_rects(
                    &clue_rows,
                    n_pegs_in_clues.value() == 7,
                    main_y_scroll_offset,
                ) {
                    if clue_rect.contains(Vec2::new(mx, my)) {
                        picked_peg = match clue_rows.get(row) {
                            Some(clue_row) => match clue_row.slots.get(col) {
                                Some(Some(id)) => {
                                    if !is_key_down(KeyCode::LeftControl) {
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
                    clue_rows[row].slots[col] = None;
                }
                let mut rem = None;
                for (i, peg) in crate::free_pegs(&free_pegs) {
                    if peg.rect().contains(Vec2::new(mx, my)) {
                        clicked_something = true;
                        picked_peg = Some(peg);
                        rem = Some(i);
                    }
                }
                if let Some(idx) = rem {
                    free_pegs.remove(idx);
                }
                if !clicked_something {
                    if mx > MAIN_AREA_START_X {
                        view_drag_center_y = Some(my);
                        stored_main_y_scroll_offset = main_y_scroll_offset;
                    } else {
                        left_drag_center_y = Some(my);
                        stored_left_y_scroll_offset = left_y_scroll_offset;
                    }
                }
            }
        }
        if let Some(view_drag_center_y_val) = view_drag_center_y {
            main_y_scroll_offset = stored_main_y_scroll_offset - (view_drag_center_y_val - my);
        }
        if let Some(left_drag_center_y_val) = left_drag_center_y {
            left_y_scroll_offset = stored_left_y_scroll_offset - (left_drag_center_y_val - my);
        }
        if main_y_scroll_offset > 0.0 {
            main_y_scroll_offset = 0.0;
        }
        if left_y_scroll_offset > 0.0 {
            left_y_scroll_offset = 0.0;
        }
        let min_y_scroll_left = -((PEG_SIZE + 1.0) * 4.0);
        if left_y_scroll_offset < min_y_scroll_left {
            left_y_scroll_offset = min_y_scroll_left;
        }
        draw_vert_scroll_bar(
            MAIN_AREA_START_X - 12.,
            FREE_PEGS_RECT.y + FREE_PEGS_RECT.h + 14.0,
            screen_height(),
            left_y_scroll_offset,
            min_y_scroll_left,
        );
        draw_vert_scroll_bar(
            screen_width() - 12.0,
            12.0,
            screen_height(),
            -main_y_scroll_offset,
            9000.0,
        );
        if is_mouse_button_released(MouseButton::Left) {
            view_drag_center_y = None;
            left_drag_center_y = None;
            if let Some(peg) = picked_peg {
                let mut ins_loc = None;
                for (clue_rect, row, col) in clue_rects(
                    &clue_rows,
                    n_pegs_in_clues.value() == 7,
                    main_y_scroll_offset,
                ) {
                    if clue_rect.contains(Vec2::new(mx, my)) {
                        ins_loc = Some((row, col));
                        break;
                    }
                }
                if let Some((row, col)) = ins_loc {
                    clue_rows[row].slots[col] = Some(peg.id);
                }
                if FREE_PEGS_RECT.contains(Vec2::new(mx, my))
                    && free_pegs.len() < 6
                    && !free_pegs.contains(&peg.id)
                {
                    free_pegs.push(peg.id);
                }
                picked_peg = None;
            }
        }
        draw_clue_rows(
            &mut clue_rows,
            mx,
            my,
            picked_peg.map(|p| color::SCHEMES[p.id as usize].skin_color()),
            tex,
            n_pegs_in_clues.value() == 7,
            main_y_scroll_offset,
            &peg_materials,
        );
        draw_pickable_pegs(tex, left_y_scroll_offset, &peg_materials);
        draw_rectangle(
            0.0,
            0.0,
            FREE_PEGS_RECT.x + FREE_PEGS_RECT.w,
            FREE_PEGS_RECT.y + FREE_PEGS_RECT.h,
            WHITE,
        );
        draw_solutions(
            &solutions,
            tex,
            rect_for_solve_button!(),
            n_pegs_in_clues.value() == 7,
            &peg_materials,
        );
        draw_free_pegs(
            tex,
            &free_pegs,
            mx,
            my,
            picked_peg.map(|p| color::SCHEMES[p.id as usize].skin_color()),
            &peg_materials,
        );
        if let Some(ref mut peg) = picked_peg {
            peg.x = mx - 32.;
            peg.y = my - 32.;
            draw_peg(tex, *peg, &peg_materials);
        }
        ptype_but.draw(mx, my);
        draw_text(&format!("{} rows", clue_rows.len()), 8.0, 64.0, 32.0, BLACK);
        draw_text(
            "Free pegs",
            FREE_PEGS_RECT.x + 4.0,
            FREE_PEGS_RECT.y + 24.0,
            32.0,
            BLACK,
        );
        clue_add_but.draw(tex, mx, my);
        clue_rem_but.draw(tex, mx, my);
        repos_solve_but(&mut solve_but, rect_for_solve_button!());
        solve_but.draw(mx, my);
        draw_text(
            &solve_msg,
            solve_but.rect.x + solve_but.rect.w + 8.0,
            solve_but.rect.y + 20.0,
            32.,
            BLACK,
        );
        draw_line(
            MAIN_AREA_START_X,
            0.0,
            MAIN_AREA_START_X,
            screen_height(),
            2.0,
            BLACK,
        );

        next_frame().await
    }
}

fn draw_vert_scroll_bar(x: f32, start_y: f32, end_y: f32, scroll: f32, max_scroll: f32) {
    let radius = 32.0;
    let ratio = scroll / max_scroll;
    let y = ratio * ((end_y - radius / 2.0) - start_y);
    draw_circle(x, start_y + y, 8.0, BLUE);
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
    peg_tex: Texture2D,
    free_pegs: &[u8],
    mx: f32,
    my: f32,
    picked_color: Option<Color>,
    mat: &[Material],
) {
    if let Some(c) = picked_color {
        if FREE_PEGS_RECT.contains(Vec2::new(mx, my)) {
            draw_rectangle(
                FREE_PEGS_RECT.x,
                FREE_PEGS_RECT.y,
                FREE_PEGS_RECT.w,
                FREE_PEGS_RECT.h,
                c,
            );
        }
    }
    draw_rectangle_lines(
        FREE_PEGS_RECT.x,
        FREE_PEGS_RECT.y,
        FREE_PEGS_RECT.w,
        FREE_PEGS_RECT.h,
        1.0,
        GREEN,
    );

    for (_, peg) in crate::free_pegs(free_pegs) {
        draw_peg(peg_tex, peg, mat);
    }
}

fn draw_solutions(
    solutions: &[Vec<u8>],
    peg_tex: Texture2D,
    bottom_rect: Rect,
    seven_peg: bool,
    mat: &[Material],
) {
    for (row, sol) in solutions.iter().enumerate() {
        for (col, peg_id) in sol.iter().enumerate() {
            let x = bottom_rect.x
                + if seven_peg {
                    SEVEN_OFFSETS[col as usize].0 * 68.
                } else {
                    col as f32 * 68.
                };
            let y = bottom_rect.y
                + 120.
                + if seven_peg {
                    let padding_between_solutions = 24.0;
                    row as f32 * (68. * 3. + padding_between_solutions)
                        + SEVEN_OFFSETS[col as usize].1 * 68.
                } else {
                    row as f32 * 68.
                };
            draw_peg(peg_tex, Pegbug { x, y, id: *peg_id }, mat)
        }
    }
}
