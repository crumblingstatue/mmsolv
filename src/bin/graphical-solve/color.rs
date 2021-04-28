//! Critters have different colors, but it's not a simple hue shift.
//! It uses color replacement of the original red critter sprite.

type Rgb = (u8, u8, u8);

pub struct Scheme {
    skin: Rgb,
    eyes: Rgb,
    eyes_shine: Rgb,
    #[allow(dead_code)]
    name: &'static str,
    #[allow(dead_code)]
    male_percent: f32,
}
#[allow(dead_code)]
const RED_DK: usize = 0;
#[allow(dead_code)]
const YEL: usize = 1;
#[allow(dead_code)]
const GREN_DK: usize = 2;
#[allow(dead_code)]
const BRWN: usize = 3;
#[allow(dead_code)]
const BLUE_LT: usize = 4;
#[allow(dead_code)]
const PRPL: usize = 5;
#[allow(dead_code)]
const RED_LT: usize = 6;
#[allow(dead_code)]
const ORNG: usize = 7;
#[allow(dead_code)]
const GREN_LT: usize = 8;
#[allow(dead_code)]
const PINK: usize = 9;
#[allow(dead_code)]
const CYAN: usize = 10;
#[allow(dead_code)]
const WHIT: usize = 11;
#[allow(dead_code)]
const BLAK: usize = 12;
#[allow(dead_code)]
const GREY: usize = 13;
#[allow(dead_code)]
const BLUE_DK: usize = 14;

pub const SCHEMES: [Scheme; 15] = [
    Scheme {
        skin: (0xb2, 0x38, 0x23),
        eyes: (0x28, 0x28, 0x28),
        eyes_shine: (0xde, 0xde, 0xde),
        name: "red", /* FJ_RED_DK */
        male_percent: 0.6,
    },
    Scheme {
        skin: (0xee, 0xda, 0x4d),
        eyes: (0xc0, 0x99, 0x2f),
        eyes_shine: (0xf1, 0xe3, 0x98),
        name: "yellow", /* FJ_YEL */
        male_percent: 0.5,
    },
    Scheme {
        skin: (0x41, 0xa7, 0x40),
        eyes: (0xc0, 0x9a, 0x2f),
        eyes_shine: (0xf1, 0xe3, 0x98),
        name: "green", /* FJ_GREN_DK */
        male_percent: 0.5,
    },
    Scheme {
        skin: (0xb4, 0x79, 0x22),
        eyes: (0x7d, 0x4c, 0x21),
        eyes_shine: (0xe5, 0xcd, 0x7d),
        name: "brown", /* FJ_BRWN */
        male_percent: 0.5,
    },
    Scheme {
        skin: (0x6f, 0x83, 0xdb),
        eyes: (0x37, 0x42, 0x65),
        eyes_shine: (0xea, 0xf2, 0xf4),
        name: "blue", /* FJ_BLUE_LT */
        male_percent: 0.5,
    },
    Scheme {
        skin: (0xa8, 0x54, 0xcb),
        eyes: (0x4f, 0xa9, 0x4e),
        eyes_shine: (0xdb, 0xe2, 0x8e),
        name: "purple", /* FJ_PRPL */
        male_percent: 0.4,
    },
    // Fruit bugs
    Scheme {
        skin: (0xf5, 0x7e, 0x7d),
        eyes: (0x7a, 0xc2, 0x52),
        eyes_shine: (0xe9, 0xf4, 0xdc),
        name: "red", /* FJ_RED_LT */
        male_percent: 0.5,
    },
    Scheme {
        skin: (0xf9, 0xbb, 0x4a),
        eyes: (0xf9, 0xa7, 0x4c),
        eyes_shine: (0xff, 0xf6, 0xdf),
        name: "orange", /* FJ_ORNG */
        male_percent: 0.5,
    },
    Scheme {
        skin: (0x8f, 0xea, 0x40),
        eyes: (0xf5, 0xd5, 0x61),
        eyes_shine: (0xfc, 0xf3, 0xcd),
        name: "green", /* FJ_GREN_LT */
        male_percent: 0.5,
    },
    // Marshmallow bugs
    Scheme {
        skin: (0xfe, 0xce, 0xef),
        eyes: (0xff, 0xdd, 0xf4),
        eyes_shine: (0xff, 0xff, 0xff),
        name: "pink", /* FJ_PINK */
        male_percent: 0.1,
    },
    Scheme {
        skin: (0xb1, 0xed, 0xee),
        eyes: (0xc1, 0xf1, 0xf2),
        eyes_shine: (0xff, 0xff, 0xff),
        name: "cyan", /* FJ_CYAN */
        male_percent: 0.2,
    },
    Scheme {
        skin: (0xf9, 0xf7, 0xd9),
        eyes: (0x91, 0xe6, 0xff),
        eyes_shine: (0xff, 0xff, 0xff),
        name: "white", /* FJ_WHIT */
        male_percent: 0.3,
    },
    // Spooky bugs
    Scheme {
        skin: (0x1a, 0x1a, 0x1e),
        eyes: (0xb8, 0x26, 0x0b),
        eyes_shine: (0xf4, 0x5e, 0x40),
        name: "black", /* FJ_BLAK */
        male_percent: 0.9,
    },
    Scheme {
        skin: (0x7a, 0x82, 0x89),
        eyes: (0xf5, 0xf0, 0xd1),
        eyes_shine: (0xff, 0xff, 0xff),
        name: "grey", /* FJ_GREY */
        male_percent: 0.8,
    },
    Scheme {
        skin: (0x0b, 0x45, 0xa6),
        eyes: (0xfa, 0xd5, 0x41),
        eyes_shine: (0xff, 0xff, 0xff),
        name: "blue", /* FJ_BLUE_DK */
        male_percent: 0.7,
    },
];

fn cdiv(c255: u8) -> f32 {
    c255 as f32 / 255.
}

type Vector3f = [f32; 3];

fn cmap(colors: (u8, u8, u8)) -> Vector3f {
    [cdiv(colors.0), cdiv(colors.1), cdiv(colors.2)]
}

impl Scheme {
    pub fn to_rgb(&self) -> [Vector3f; 3] {
        [cmap(self.skin), cmap(self.eyes), cmap(self.eyes_shine)]
    }
}
