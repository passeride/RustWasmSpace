use macroquad::prelude::*;

use crate::planet_data::PlanetData;

pub fn rotate_point(cx: f32, cy: f32, angle: f32, mut p: Vec2) -> Vec2 {
    let s = angle.sin();
    let c = angle.cos();

    // translate point back to origin:
    p.x -= cx;
    p.y -= cy;

    // rotate point
    let xnew = p.x * c - p.y * s;
    let ynew = p.x * s + p.y * c;

    // translate point back:
    p.x = xnew + cx;
    p.y = ynew + cy;
    return p;
}

// Calcuate orbit
// https://www.stjarnhimlen.se/comp/ppcomp.html

// Data from : https://nssdc.gsfc.nasa.gov/planetary/factsheet/
pub struct Planet {
    pub color: Color,
    pub name: &'static str,
    pub planet_data: PlanetData,
}

impl Planet {
    pub const SUN: Planet = Planet {
        color: YELLOW,
        name: &"Sun",
        planet_data: PlanetData::SUN,
    };

    pub const MERCURY: Planet = Planet {
        color: GRAY,
        name: &"Mercury",
        planet_data: PlanetData::MERCURY,
    };

    pub const VENUS: Planet = Planet {
        color: GREEN,
        name: &"Venus",
        planet_data: PlanetData::VENUS,
    };

    pub const EARTH: Planet = Planet {
        color: BLUE,
        name: &"Earth",
        planet_data: PlanetData::EARTH,
    };

    pub const MARS: Planet = Planet {
        color: RED,
        name: &"Mars",
        planet_data: PlanetData::MARS,
    };
}
