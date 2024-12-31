use std::fmt::Display;

use anyhow::anyhow;
use config_macro::config_default;
use hiarc::Hiarc;
use math::math::vector::ubvec4;
use serde::{Deserialize, Serialize};

/// Rgb color specifically for config
#[config_default]
#[derive(Debug, Hiarc, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConfRgb {
    #[default = 255]
    pub r: u8,
    #[default = 255]
    pub g: u8,
    #[default = 255]
    pub b: u8,
}

impl Display for ConfRgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {}, g: {}, b: {}", self.r, self.g, self.b)
    }
}

impl From<ConfRgb> for ubvec4 {
    fn from(val: ConfRgb) -> Self {
        ubvec4::new(val.r, val.g, val.b, 255)
    }
}

impl From<ubvec4> for ConfRgb {
    fn from(val: ubvec4) -> Self {
        ConfRgb {
            r: val.r(),
            g: val.g(),
            b: val.b(),
        }
    }
}

impl ConfRgb {
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
    pub fn grey() -> Self {
        Self {
            r: 128,
            g: 128,
            b: 128,
        }
    }

    pub fn from_html_color_code(code: &str) -> anyhow::Result<Self> {
        anyhow::ensure!(code.starts_with('#'));
        let code = &code[1..];
        anyhow::ensure!(code.len() == 3 || code.len() == 6);

        if code.len() == 3 {
            Ok(Self {
                r: u8::from_str_radix(&code[0..1], 16)?,
                g: u8::from_str_radix(&code[1..2], 16)?,
                b: u8::from_str_radix(&code[2..3], 16)?,
            })
        } else {
            Ok(Self {
                r: u8::from_str_radix(&code[0..2], 16)?,
                g: u8::from_str_radix(&code[2..4], 16)?,
                b: u8::from_str_radix(&code[4..6], 16)?,
            })
        }
    }
    pub fn from_css_rgb_fn(code: &str) -> anyhow::Result<Self> {
        let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        anyhow::ensure!(code.starts_with("rgb(") && code.ends_with(')'));

        let code = &code["rgb(".len()..code.len() - ')'.len_utf8()];

        let mut nums = code.split(',');
        Ok(Self {
            r: nums
                .next()
                .ok_or_else(|| anyhow!("red component not found."))?
                .parse()?,
            g: nums
                .next()
                .ok_or_else(|| anyhow!("green component not found."))?
                .parse()?,
            b: nums
                .next()
                .ok_or_else(|| anyhow!("blue component not found."))?
                .parse()?,
        })
    }
    pub fn from_display(code: &str) -> anyhow::Result<Self> {
        let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        let mut nums = code.split(',');
        Ok(Self {
            r: nums
                .next()
                .and_then(|v| v.split_once(":").and_then(|(s, v)| (s == "r").then_some(v)))
                .ok_or_else(|| anyhow!("red component not found."))?
                .parse()?,
            g: nums
                .next()
                .and_then(|v| v.split_once(":").and_then(|(s, v)| (s == "g").then_some(v)))
                .ok_or_else(|| anyhow!("green component not found."))?
                .parse()?,
            b: nums
                .next()
                .and_then(|v| v.split_once(":").and_then(|(s, v)| (s == "b").then_some(v)))
                .ok_or_else(|| anyhow!("blue component not found."))?
                .parse()?,
        })
    }
}
