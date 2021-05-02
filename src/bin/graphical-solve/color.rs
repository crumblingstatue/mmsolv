//! Critters have different colors, but it's not a simple hue shift.
//! It uses color replacement of the original red critter sprite.

use macroquad::prelude::Color;

pub struct Scheme {
    pub skin: Vector3f,
    pub eyes: Vector3f,
    pub eyes_shine: Vector3f,
}

pub const SCHEMES: [Scheme; 15] = [
    Scheme {
        skin: cmap(0xb2, 0x38, 0x23),
        eyes: cmap(0x28, 0x28, 0x28),
        eyes_shine: cmap(0xde, 0xde, 0xde),
    },
    Scheme {
        skin: cmap(0xee, 0xda, 0x4d),
        eyes: cmap(0xc0, 0x99, 0x2f),
        eyes_shine: cmap(0xf1, 0xe3, 0x98),
    },
    Scheme {
        skin: cmap(0x41, 0xa7, 0x40),
        eyes: cmap(0xc0, 0x9a, 0x2f),
        eyes_shine: cmap(0xf1, 0xe3, 0x98),
    },
    Scheme {
        skin: cmap(0xb4, 0x79, 0x22),
        eyes: cmap(0x7d, 0x4c, 0x21),
        eyes_shine: cmap(0xe5, 0xcd, 0x7d),
    },
    Scheme {
        skin: cmap(0x6f, 0x83, 0xdb),
        eyes: cmap(0x37, 0x42, 0x65),
        eyes_shine: cmap(0xea, 0xf2, 0xf4),
    },
    Scheme {
        skin: cmap(0xa8, 0x54, 0xcb),
        eyes: cmap(0x4f, 0xa9, 0x4e),
        eyes_shine: cmap(0xdb, 0xe2, 0x8e),
    },
    // Fruit bugs
    Scheme {
        skin: cmap(0xf5, 0x7e, 0x7d),
        eyes: cmap(0x7a, 0xc2, 0x52),
        eyes_shine: cmap(0xe9, 0xf4, 0xdc),
    },
    Scheme {
        skin: cmap(0xf9, 0xbb, 0x4a),
        eyes: cmap(0xf9, 0xa7, 0x4c),
        eyes_shine: cmap(0xff, 0xf6, 0xdf),
    },
    Scheme {
        skin: cmap(0x8f, 0xea, 0x40),
        eyes: cmap(0xf5, 0xd5, 0x61),
        eyes_shine: cmap(0xfc, 0xf3, 0xcd),
    },
    // Marshmallow bugs
    Scheme {
        skin: cmap(0xfe, 0xce, 0xef),
        eyes: cmap(0xff, 0xdd, 0xf4),
        eyes_shine: cmap(0xff, 0xff, 0xff),
    },
    Scheme {
        skin: cmap(0xb1, 0xed, 0xee),
        eyes: cmap(0xc1, 0xf1, 0xf2),
        eyes_shine: cmap(0xff, 0xff, 0xff),
    },
    Scheme {
        skin: cmap(0xf9, 0xf7, 0xd9),
        eyes: cmap(0x91, 0xe6, 0xff),
        eyes_shine: cmap(0xff, 0xff, 0xff),
    },
    // Spooky bugs
    Scheme {
        skin: cmap(0x1a, 0x1a, 0x1e),
        eyes: cmap(0xb8, 0x26, 0x0b),
        eyes_shine: cmap(0xf4, 0x5e, 0x40),
    },
    Scheme {
        skin: cmap(0x7a, 0x82, 0x89),
        eyes: cmap(0xf5, 0xf0, 0xd1),
        eyes_shine: cmap(0xff, 0xff, 0xff),
    },
    Scheme {
        skin: cmap(0x0b, 0x45, 0xa6),
        eyes: cmap(0xfa, 0xd5, 0x41),
        eyes_shine: cmap(0xff, 0xff, 0xff),
    },
];

const fn cdiv(c255: u8) -> f32 {
    c255 as f32 / 255.
}

type Vector3f = [f32; 3];

const fn cmap(r: u8, g: u8, b: u8) -> Vector3f {
    [cdiv(r), cdiv(g), cdiv(b)]
}

impl Scheme {
    pub const fn skin_color(&self) -> Color {
        let [r, g, b] = self.skin;
        Color { r, g, b, a: 1.0 }
    }
}
