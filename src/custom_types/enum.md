# Enumerados

La etiqueta `enum` permite la creación de un tipo que puede ser una de las 
posibles variantes. Cualquier variante que sea válida como `struct` es también válida 
como `enum`.


```rust,editable
// Crear un `enum` para clasificar un evento web. Observe cómo 
// los nombres y la información del tipo especifican conjuntamente 
// la variante:
// `PageLoad != PageUnload` y `KeyPress(char) != Paste(String)`.
// Cada uno es diferente e independiente.
enum WebEvent {
    // Un `enum` puede ser tanto de tipo unitario
    PageLoad,
    PageUnload,
    // como structs de tupla
    KeyPress(char),
    Paste(String),
    // o como estructuras de C.
    Click { x: i64, y: i64 },
}

// Una función que tiene un enumerado de `WebEvent` como argumento y 
// no devuelve nada.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructura `c` sdentro del `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructura `Click` en `x` e `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` crea una `String` propia a partir de una slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```
## Alias de tipo

Si utiliza un alias de tipo, puede referirse a cada variante del enumerado a través de su alias. Esto puede ser útil si el nombre del enumerado es demasiado largo o demasiado genérico, y usted desea renombrarlo.

```rust,editable
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Crea un alias de tipo
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // Podemos referirnos a cada variante a través de su alias, y no de su largo e inconveniente nombre.
    let x = Operations::Add;
}
```

El lugar más común donde verás esto es en los bloques `impl` usando el alias `Self`.

```rust,editable
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

Para saber más sobre los enums y los alias de tipo, puede leer el
[informe de estabilización][aliasreport] de cuando esta característica se estabilizó en Rust.

### Ver también:

[`match`][match], [`fn`][fn], and [`String`][str], ["Alias de tipo en variantes de enumerado" RFC][type_alias_rfc]

[c_struct]: https://en.wikipedia.org/wiki/Struct_(C_programming_language)
[match]: ../flow_control/match.md
[fn]: ../fn.md
[str]: ../std/str.md
[aliasreport]: https://github.com/rust-lang/rust/pull/61682/#issuecomment-502472847
[type_alias_rfc]: https://rust-lang.github.io/rfcs/2338-type-alias-enum-variants.html
