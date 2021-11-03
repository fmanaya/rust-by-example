use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitud
    lat: f32,
    // Longitud
    lon: f32,
}

impl Display for City {
    // `f` es un buffer, y este metodo debe escribir la cadena formateada en el
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` es como `format!`, pero escribirá la cadena formateada
        // en un buffer (el primer argumento)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
// impl Display for Color {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        // `write!` es como `format!`, pero escribirá la cadena formateada
//        // en un buffer (el primer argumento)
       
//        let _R = format!("{:X}", self.red);
//        let _G = format!("{:X}", self.green);
//        let _B = format!("{:X}", self.blue);
       
//        write!(f, "RGB ({}, {}, {}) 0x{:0>2}{:0>2}{:0>2}",
//               self.red, self.green, self.blue,
//               _R, _G, _B)
//    }
// }

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Cambie esto para usar {} una vez que haya añadido una 
        // implementación para fmt::Display.
        println!("{:?}", *color);
//        println!("{}", *color);
    }
}