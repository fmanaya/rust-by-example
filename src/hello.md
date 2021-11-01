# Hola Mundo

Este es el código fuente del tradicional programa Hola Mundo.

```rust,editable
// Esto es un comentario, y es ignorado por el compilador
// Puedes probar este código haciendo clic en el boton "Run" ->
// o pulsando el atajo de teclado "Ctrl + Enter" si prefieres.

// Este código es editable, eres libre de modificarlo y probar.
// Siempre podras volver al codigo original haciendo click en el boton "Reset" ->

// Esta es la función main
fn main() {
    // Las instrucciones se ejecutan cuando se llama al binario compilado/ejecutable

    // Imprime un texto en la consola
    println!("Hello World!");
}
```

`println!` es una [*macro*][macros] que imprime texto en la consola.

EL binario/ejecutable se puede generar con el compilador Rust: `rustc`.

```bash
$ rustc hello.rs
```

`rustc` generará `hello` binario que se puede ejecutar.

```bash
$ ./hello
Hello World!
```

### Actividad

Si hace click en el 'Run' superior podrás ver la salida esperada. Luego añade una nueva linea con un segunda macro `println!` para que en la salida veamos lo siguiente:

```text
Hello World!
I'm a Rustacean!
```

[macros]: macros.md
