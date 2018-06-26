use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,

    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}:{:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {
            name: "Doublin",
            lat: 53.342222,
            lon: -6.3444434,
        },
        City {
            name: "Oslo",
            lat: 59.34,
            lon: 10.32,
        },
        City {
            name: "Van",
            lat: 34.22,
            lon: -123.34,
        },
    ].iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 225,
            blue: 98,
        },
        Color {
            red: 0,
            green: 22,
            blue: 234,
        },
        Color {
            red: 0,
            green: 3,
            blue: 0,
        },
    ].iter()
    {
        println!("{:?}", *color)
    }
}
