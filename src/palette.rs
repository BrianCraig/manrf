use embedded_graphics::pixelcolor::Rgb888;

pub struct Palette {
    pub darkest: Rgb888,
    pub dark: Rgb888,
    pub light: Rgb888,
    pub lightest: Rgb888,
}

// palettes from https://i.redd.it/0svby0ssa7s41.png by Brandon James Greer

pub static PALETTE_ICE: Palette = Palette {
    darkest: Rgb888::new(80, 26, 104),
    dark: Rgb888::new(102, 83, 203),
    light: Rgb888::new(109, 161, 223),
    lightest: Rgb888::new(155, 235, 235),
};

pub static PALETTE_DREAM: Palette = Palette {
    darkest: Rgb888::new(63, 13, 104),
    dark: Rgb888::new(244, 28, 125),
    light: Rgb888::new(50, 217, 206),
    lightest: Rgb888::new(255, 236, 141),
};

pub static PALETTE_DARK_CHERRY: Palette = Palette {
    darkest: Rgb888::new(21, 1, 122),
    dark: Rgb888::new(246, 9, 131),
    light: Rgb888::new(253, 151, 133),
    lightest: Rgb888::new(255, 245, 222),
};

pub static PALETTE_CARDBOARD: Palette = Palette {
    darkest: Rgb888::new(60, 30, 21),
    dark: Rgb888::new(175, 83, 20),
    light: Rgb888::new(198, 170, 53),
    lightest: Rgb888::new(197, 251, 225),
};