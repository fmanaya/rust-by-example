# Debug

Todos los tipos de datos que quieran usar los `trait` de formato de la libreria `std::fmt` requieren una
implementación para ser imprimible. Las implementaciones automáticas solo se proporcionan para tipos 
como en la biblioteca `std`. Todos los demás *deben* implementarse manualmente.

El `trait` `fmt::Debug` hace que esto sea muy sencillo. *Todos* los tipos pueden `derivar` (crear automáticamente) 
la implementación `fmt::Debug`. Esto no es cierto para `fmt::Display`, que debe implementarse manualmente.

```rust
// Esta estuctura no se puede imprimir con `fmt::Display` ni con `fmt::Debug`.
struct UnPrintable(i32);

// El atributo `derive` automaticamente crea la implementacion necesaria 
// para hacer esta `struct` imprimible con `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);
```

Todos los tipos de la libreria `std` son también imprimibles con el simbolo `{:?}`:

```rust,editable

// Deriva la implementacion `fmt::Debug` para la estructura `Structure`. `Structure`
// es una estrutura que contiene un solo `i32`.
#[derive(Debug)]
struct Structure(i32);

// Pone `Structure` en la estructura `Deep`. También es imprimible.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Imprimir con el simbolo `{:?}` es similar a hacerlo con `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` es imprimible!
    println!("Now {:?} will print!", Structure(3));
    
    // El problema con `derive` es que no hay control sobre como aparece el resultado   
    // Que pasa si solo quiero mostrar `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

Así `fmt::Debug` definitivamente hace esto imprimible, pero sacrifica la posibilidad de algo 
mas elegante. Rust también proporciona una "impresion hermosa" con el simbolo `{:#?}`.

```rust,editable
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Impresion hermosa
    println!("{:#?}", peter);
}
```

Se puede implementar manualmente `fmt::Display` para controlar la visualización.

### Ver también:

[`atributos`][attributes], [`derive`][derive], [`std::fmt`][fmt],
and [`struct`][structs]

[attributes]: https://doc.rust-lang.org/reference/attributes.html
[derive]: ../../trait/derive.md
[fmt]: https://doc.rust-lang.org/std/fmt/
[structs]: ../../custom_types/structs.md

