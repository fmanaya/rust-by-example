# Formateando

Hemos visto que el formato se especifica mediante una *cadena de formato*.

* `format!("{}", foo)` -> `"3735928559"`
* `format!("0x{:X}", foo)` ->
  [`"0xDEADBEEF"`][deadbeef]
* `format!("0o{:o}", foo)` -> `"0o33653337357"`

La misma variable (`foo`) puede tener un formato diferente según el *tipo de argumento* 
que se utilice: `X` vs. `o` vs. *ninguno*.

Esta funcionalidad de formato se implementa mediante traits, y hay un trait para 
cada tipo de argumento. El trait de formato más común es `Display`, que maneja los 
casos en los que el tipo de argumento se deja sin especificar: `{}` por ejemplo.

```rust,editable
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
    }
}
```

Puede ver una [lista completa de traits de formato][fmt_traits] y sus tipos de 
argumentos en su documentación [`std::fmt`][fmt].

### Actividad
Añade una implementación del trait `fmt::Display` para la estructura `Color` 
anterior de modo que la salida se muestre como:

```text
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

Dos consejos si te quedas atascado:
 * Puedes [necesitar usar cada color más de una vez][named_parameters],
 * Puedes [rellenar con ceros hasta un ancho de 2][fmt_width] con `:0>2`.

### Ver también:

[`std::fmt`][fmt]

[named_parameters]: https://doc.rust-lang.org/std/fmt/#named-parameters
[deadbeef]: https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values
[fmt]: https://doc.rust-lang.org/std/fmt/
[fmt_traits]: https://doc.rust-lang.org/std/fmt/#formatting-traits
[fmt_width]: https://doc.rust-lang.org/std/fmt/#width
