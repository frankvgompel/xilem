use crate::apca::estimate_lc;
use palette::{LinSrgb, Okhsl, Srgb, FromColor, IntoColor};
use peniko::Color;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenColor {
    #[default]
    AppBackground,
    SubtleBackground,
    UiElementBackground,
    HoveredUiElementBackground,
    ActiveUiElementBackground,
    SubtleBordersAndSeparators,
    UiElementBorderAndFocusRings,
    HoveredUiElementBorder,
    SolidBackgrounds,
    HoveredSolidBackgrounds,
    LowContrastText,
    HighContrastText,
    AccentText,
    Transparent,
    Custom(Color),
}

/// The functional UI elements mapped to a scale
#[derive(Default, Debug, Clone, Copy)]
pub struct ColorTokens {
    pub app_background: Color,
    pub subtle_background: Color,
    pub ui_element_background: Color,
    pub hovered_ui_element_background: Color,
    pub active_ui_element_background: Color,
    pub subtle_borders_and_separators: Color,
    pub ui_element_border_and_focus_rings: Color,
    pub hovered_ui_element_border: Color,
    pub solid_backgrounds: Color,
    pub hovered_solid_backgrounds: Color,
    pub low_contrast_text: Color,
    pub high_contrast_text: Color,
    pub(crate) inverse_color: bool,
    pub color_on_accent: Color,
}

impl ColorTokens {
    pub(crate) fn color_on_accent(&mut self) {
        let white = Srgb::<u8>::from_components((255, 255, 255));
        let bg = self.solid_backgrounds;
        let bg_cl = Srgb::<u8>::from_components((bg.r, bg.g, bg.b));
  
        let lc = estimate_lc(white, bg_cl);
        if lc > -46. {
            self.inverse_color = true;
            let s = self.solid_backgrounds;
            let inverse = LinSrgb::from_components((s.r, s.g, s.b)).into_format();
            let mut okhsl = Okhsl::from_color(inverse);

            okhsl.lightness = 0.01;
            okhsl.saturation = 0.7;
            let (r, g, b) = Srgb::from_linear(okhsl.into_color()).into();
            self.color_on_accent = Color::rgb8(r, g, b);
        } else {
            self.color_on_accent = Color::WHITE;
        }
    }

    /// the color for the text on accented color (solid backgrounds)
    // #[must_use]
    // pub const fn text_color(&self) -> Color {
    //     self.color_on_accent
    // }

    /// notifies when lc > -46.
    #[must_use]
    pub const fn inverse_color(&self) -> bool {
        self.inverse_color
    }

    pub(crate) fn update_schema(&mut self, i: usize, fill: Color) {
        match i {
            0 => self.app_background = fill,
            1 => self.subtle_background = fill,
            2 => self.ui_element_background = fill,
            3 => self.hovered_ui_element_background = fill,
            4 => self.active_ui_element_background = fill,
            5 => self.subtle_borders_and_separators = fill,
            6 => self.ui_element_border_and_focus_rings = fill,
            7 => self.hovered_ui_element_border = fill,
            8 => self.solid_backgrounds = fill,
            9 => self.hovered_solid_backgrounds = fill,
            10 => self.low_contrast_text = fill,
            11 => self.high_contrast_text = fill,
            _ => {}
        }
    }

    pub fn set_color(&self, token: TokenColor) -> Color {
        match token {
            TokenColor::AppBackground => self.app_background,
            TokenColor::SubtleBackground => self.subtle_background,
            TokenColor::UiElementBackground => self.ui_element_background,
            TokenColor::HoveredUiElementBackground => self.hovered_ui_element_background,
            TokenColor::ActiveUiElementBackground => self.active_ui_element_background,
            TokenColor::SubtleBordersAndSeparators => self.subtle_borders_and_separators,
            TokenColor::UiElementBorderAndFocusRings => self.ui_element_border_and_focus_rings,
            TokenColor::HoveredUiElementBorder => self.hovered_ui_element_border,
            TokenColor::SolidBackgrounds => self.solid_backgrounds,
            TokenColor::HoveredSolidBackgrounds => self.hovered_solid_backgrounds,
            TokenColor::LowContrastText => self.low_contrast_text,
            TokenColor::HighContrastText => self.high_contrast_text,
            TokenColor::AccentText => self.color_on_accent,
            TokenColor::Custom(color) => color,
            TokenColor::Transparent => Color::TRANSPARENT,         
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThemeColor {
    #[default]
    Gray,
    EguiBlue,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Brown,
    Bronze,
    Gold,
    Orange,
    Custom([u8; 3]),
}

impl ThemeColor {
    pub(crate) fn get_srgb(self) -> LinSrgb<f32> {
        let [r, g, b] = self.rgb();
        Srgb::new(r, g, b).into_linear()
    }
    /// Returns the rgb values of this preset.
    ///
    /// Useful for example if you want to serialize the color theme.
    #[must_use]
    pub const fn rgb(&self) -> [u8; 3] {
        match *self {
            Self::Gray => [117, 117, 117],
            Self::EguiBlue => [0, 109, 143],
            Self::Tomato => [229, 77, 46],
            Self::Red => [229, 72, 77],
            Self::Ruby => [229, 70, 102],
            Self::Crimson => [233, 61, 130],
            Self::Pink => [214, 64, 159],
            Self::Plum => [171, 74, 186],
            Self::Purple => [142, 78, 198],
            Self::Violet => [110, 86, 207],
            Self::Iris => [91, 91, 214],
            Self::Indigo => [62, 99, 214],
            Self::Blue => [0, 144, 255],
            Self::Cyan => [0, 162, 199],
            Self::Teal => [18, 165, 148],
            Self::Jade => [41, 163, 131],
            Self::Green => [48, 164, 108],
            Self::Grass => [70, 167, 88],
            Self::Brown => [173, 127, 88],
            Self::Bronze => [161, 128, 114],
            Self::Gold => [151, 131, 101],
            Self::Orange => [247, 107, 21],
            Self::Custom([r, g, b]) => [r, g, b],
        }
    }
    // pub(crate) const fn label(self) -> &'static str {
    //     match self {
    //         Self::Gray => "Gray",
    //         Self::EguiBlue => "EguiBlue",
    //         Self::Tomato => "Tomato",
    //         Self::Red => "Red",
    //         Self::Ruby => "Ruby",
    //         Self::Crimson => "Crimson",
    //         Self::Pink => "Pink",
    //         Self::Plum => "Plum",
    //         Self::Purple => "Purple",
    //         Self::Violet => "Violet",
    //         Self::Iris => "Iris",
    //         Self::Indigo => "Indigo",
    //         Self::Blue => "Blue",
    //         Self::Cyan => "Cyan",
    //         Self::Teal => "Teal",
    //         Self::Jade => "Jade",
    //         Self::Green => "Green",
    //         Self::Grass => "Grass",
    //         Self::Brown => "Brown",
    //         Self::Bronze => "Bronze",
    //         Self::Gold => "Gold",
    //         Self::Orange => "Orange",
    //         Self::Custom(_) => "Custom",
    //     }
    // }
}
