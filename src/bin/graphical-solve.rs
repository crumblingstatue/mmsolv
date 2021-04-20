use macroquad::prelude::*;

const PEG_SIZE: f32 = 64.0;
const PEG_COLORS: [Color; 6] = [RED, YELLOW, GREEN, BLUE, VIOLET, BROWN];

#[derive(Clone, Copy)]
struct Pegbug {
    x: f32,
    y: f32,
    color_idx: usize,
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

fn bottom_peg(idx: usize) -> Pegbug {
    Pegbug {
        x: idx as f32 * 80.,
        y: screen_height() - PEG_SIZE,
        color_idx: idx,
    }
}

fn bottom_pegs() -> impl Iterator<Item = Pegbug> {
    (0..PEG_COLORS.len()).map(bottom_peg)
}

fn draw_peg(peg_tex: Texture2D, peg: Pegbug) {
    draw_texture(peg_tex, peg.x, peg.y, PEG_COLORS[peg.color_idx]);
    draw_rectangle_lines(peg.x, peg.y, PEG_SIZE, PEG_SIZE, 3.0, RED);
}

fn draw_bottom_pegs(peg_tex: Texture2D) {
    bottom_pegs().for_each(|peg| {
        draw_peg(peg_tex, peg);
    });
}

#[macroquad::main("mmsolv")]
async fn main() {
    let mut picked_peg: Option<Pegbug> = None;
    let mut placed_pegs = Vec::new();
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
        if is_mouse_button_pressed(MouseButton::Left) {
            match picked_peg {
                None => {
                    for peg in bottom_pegs() {
                        if peg.rect().contains(Vec2::new(mx, my)) {
                            picked_peg = Some(peg);
                        }
                    }
                }
                Some(peg) => {
                    placed_pegs.push(peg);
                    picked_peg = None;
                }
            }
        }

        for peg in &placed_pegs {
            draw_peg(peg_tex, *peg);
        }

        next_frame().await
    }
}
