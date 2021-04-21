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

const BUGS_PER_ROW: u8 = 8;

fn bottom_peg(idx: usize) -> Pegbug {
    Pegbug {
        x: (idx % BUGS_PER_ROW as usize) as f32 * 80.,
        y: screen_height() - (PEG_SIZE * 2.0) + (idx / BUGS_PER_ROW as usize) as f32 * PEG_SIZE,
        color_idx: idx,
    }
}

fn bottom_pegs() -> impl Iterator<Item = Pegbug> {
    (0..PEG_COLORS.len()).map(bottom_peg)
}

fn draw_peg(peg_tex: Texture2D, peg: Pegbug) {
    draw_texture(peg_tex, peg.x, peg.y, PEG_COLORS[peg.color_idx]);
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
        if is_mouse_button_pressed(MouseButton::Left) && picked_peg.is_none() {
            for peg in bottom_pegs() {
                if peg.rect().contains(Vec2::new(mx, my)) {
                    picked_peg = Some(peg);
                }
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(peg) = picked_peg {
                placed_pegs.push(peg);
                picked_peg = None;
            }
        }

        for peg in &placed_pegs {
            draw_peg(peg_tex, *peg);
        }

        next_frame().await
    }
}
