# Display

`fmt::Debug` no es muy compacto y limpio, por lo que a menudo combiene 
adaptar la apariencia de la salida. Esto se hace implementando manualmente 
[`fmt::Display`][fmt], que usa el marcador de impresión `{}`. Implementarlo 
se ve así:

```rust
// Importa (via `use`) el módulo `fmt` para tenerlo disponible.
use std::fmt;

// Define una estructura para la cual se implementará `fmt::Display`. Esta 
// es una estructura de tupla llamada "Structure" que contiene un "i32".
struct Structure(i32);

// Para usar el marcador `{}`, el trait `fmt::Display` debe implementarse 
// manualmente para el tipo.

impl fmt::Display for Structure {
    // Este trait requiere `fmt` con esta misma nomenclatura.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Escribe estrictamente el primer elemento en el flujo de salida 
        // proporcionado: `f`. Devuelve `fmt::Result`, que indica si la 
        // operación tuvo éxito o no. Tenga en cuenta que `write!` Usa 
        // una sintaxis que es muy similar a `println!`.        
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display` puede ser más limpio que` fmt::Debug` pero presenta
un problema para la biblioteca `std`. ¿Cómo se deben mostrar los tipos 
ambiguos? Por ejemplo, si la biblioteca `std` implementó un solo estilo para 
todos los` Vec <T> `, ¿qué estilo debería ser? ¿Sería alguno de estos dos?

* `Vec<path>`: `/:/etc:/home/username:/bin` (separar por `:`)
* `Vec<number>`: `1,2,3` (separat por `,`)

No, porque no existe un estilo ideal para todos los tipos y la biblioteca `std` 
no pretende imponer uno. `fmt::Display` no está implementado para` Vec<T>` ni 
para ningún otro contenedor genérico. `fmt::Debug` debe usarse entonces para 
estos casos genéricos.

Sin embargo, esto no es un problema porque para cualquier nuevo tipo de *contenedor* 
que no sea genérico, se puede implementar `fmt::Display`.

```rust,editable
use std::fmt; // Importa `fmt`

// Una estructura que posee dos numeros. `Debug` se derivará para que los
// resultados se puedan contrastar con `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implementación `Display` para `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Usa `self.number` para referirse por posición a cada punto de datos.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Defina una estructura con nombres de campos para poder compararlos.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// De forma parecida, implementa `Display` para `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Adaptacion para solo indicar `x` e `y`.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Se implementaron tanto `Debug` como `Display`, pero `{:b}` 
    // requiere que se implemente `fmt::Binary`. Esto no funcionará.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

Entonces, `fmt::Display` se ha implementado pero `fmt::Binary` no, y por lo 
tanto no se puede usar. `std::fmt` tiene muchos [`traits`][traits] y cada 
uno requiere su propia implementación. Esto se detalla más en [`std::fmt`][fmt].

### Actividad

Después de verificar el resultado del ejemplo anterior, use la estructura `Point2D`
como guía para agregar una estructura `Complex` al ejemplo. Cuando se imprima de la 
misma manera, la salida debe ser:

```txt
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

### Ver también:

[`derive`][derive], [`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
[`trait`][traits], and [`use`][use]

[derive]: ../../trait/derive.md
[fmt]: https://doc.rust-lang.org/std/fmt/
[macros]: ../../macros.md
[structs]: ../../custom_types/structs.md
[traits]: https://doc.rust-lang.org/std/fmt/#formatting-traits
[use]: ../../mod/use.md
