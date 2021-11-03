# Primitivos

Rust proporciona un amplio abanico de tipos `primitivos`. Una parte de ellos son:

### Tipos escalares

* enteros con signo: `i8`, `i16`, `i32`, `i64`, `i128` y `isize` (tamaño del puntero)
* enteros sin signo: `u8`, `u16`, `u32`, `u64`, `u128` y `usize` (tamaño del puntero)
* punto flotante: `f32`, `f64`
* `char` valores escalares Unicode como `'a'`, `'α'` y `'∞'` (4 bytes cada uno)
* `bool`, `verdadero` o `falso`
* y el tipo unitario `()`, cuyo único valor posible es una tupla vacía: `()`

A pesar de que el valor de un tipo unitario es una tupla, no se considera un 
tipo compuesto porque no contiene múltiples valores.

### Tipos compuestos

* arrays como `[1, 2, 3]`
* tuplas como `(1, true)`

Las variables siempre pueden ser *anotadas por tipo*. Los números también pueden 
ser anotados mediante un *sufijo* o *por defecto*. Los enteros son por defecto `i32` 
y los punto flotante a `f64`. Rust también puede inferir los tipos a partir del contexto.

```rust,editable,ignore,mdbook-runnable
fn main() {
    // Las variables pueden ser anotadas
    let logical: bool = true;

    let a_float: f64 = 1.0;  // notación regular
    let an_integer   = 5i32; // notación como sufijo

    // o usar el tipo por defecto.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // Un tipo también puede inferir del contexto 
    let mut inferred_type = 12; // El tipo i64 se deduce de otra línea
    inferred_type = 4294967296i64;
    
    // El valor de una variable mutable puede ser modificado.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error. No se puede cambiar el tipo de una variable.
    mutable = true;
    
    // Las variables se pueden sobrescribir con el sombreado de variables.
    let mutable = true;
}
```

### Ver también:

[el módulo `std`][std], [`mutabilidad`][mut], [`inferencia`][inference], y [`sombreado`][shadowing]

[std]: https://doc.rust-lang.org/std/
[mut]: variable_bindings/mut.md
[inference]: types/inference.md
[shadowing]: variable_bindings/scope.md
