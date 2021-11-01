# Impresión con formato

La impresion por consola es manejada por una serie de [`macros`][macros] definidas en [`std::fmt`][fmt]
algunas de las cuales incluyen:

* `format!`: escribe texto con formato en [`String`][string]
* `print!`: igual que `format!` pero el texto se imprime en la salida estándar (io::stdout).
* `println!`: igual que `print!` pero se añade un caracter de salto de linea al final.
* `eprint!`: igual que `format!` pero el texto se imprime en error estándar (io::stderr).
* `eprintln!`: igual que `eprint!` pero se añade un caracter de salto de linea al final.

Todas analizan el texto del mismo modo. Además, Rust comprueba la validez del formato en tiempo de compilación.

```rust,editable,ignore,mdbook-runnable
fn main() {
    // En general, el símbolo `{}` sera remplazado automaticamente por cualquier 
    // argumento. Y este sera convertido a una cadena de texto.
    println!("{} days", 31);

    // Sin un sufijo, 31 se convierte en un i32. Puedes cambiar que tipo de dato 
    // es 31 proporcionando un sufijo. El numero 31i64 por ejemplo es de tipo i64.

    // Hay varios patrones opcionales con los que funciona. Se pueden usar 
    // argumentos posicionales.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Y tambien argumentos con nombre.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Un formato especial se puede especificar despues del símbolo `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // Puedes alinear texto a la derecha con un ancho concreto. Esto imprime:
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // Puedes rellenar números con ceros extra. Esto imprime "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Rust incluso comprueba que se usa el numero correcto de argumentos.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Añade el argumento que falta: "James"

    // Crea una estructura llamada `Structure` que contiene un `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // Sin embargo, otros tipos como esta estructura requiere algo mas complicado. 
    // Esto no funcionará.
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Commenta esta linea.
}
```

[`std::fmt`][fmt] contiene muchos [`traits`][traits] que gobiernan la salida de texto. La forma básica de los dos mas importantes se listan a continuación:

* `fmt::Debug`: Usa el marcador `{:?}`. Formatea texto con propósito de depuración.
* `fmt::Display`: Usa el marcador `{}`. Formatea texto de forma mas amigable y elegante.

Aquí, usamos  `fmt::Display` porque la libreria std proporciona implementaciones para estos tipos de dato. Para imprimir texto con tipos propios se necesitan mas pasos.

Implementando el trait `fmt::Display` automaticamente implementa el trait [`ToString`] que nos permite [convertir] the type to [`String`][string].

### Actividades

 * Corrije los dos problemas del código anterior (ver FIXME) para que se ejecute sin error.
 * Añade la macro `println!` para imprimir: `Pi is roughly 3.142` controlando el número de decimales a mostrar. Para el proósito de este ejercicio, usa `let pi = 3.141592` como una estimación de pi. (Sugerencia: es posible que tenga que consultar la documentación de [`std::fmt`][fmt] para establecer el número de decimales a mostrar)

### Ver también:

[`std::fmt`][fmt], [`macros`][macros], [`struct`][structs],
and [`traits`][traits]

[fmt]: https://doc.rust-lang.org/std/fmt/
[macros]: ../macros.md
[string]: ../std/str.md
[structs]: ../custom_types/structs.md
[traits]: https://doc.rust-lang.org/std/fmt/#formatting-traits
[`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
[convertir]: ../conversion/string.md
