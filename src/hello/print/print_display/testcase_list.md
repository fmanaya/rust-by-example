# Caso de prueba: List

Implementar `fmt::Display` para una estructura en la que los elementos deben 
ser manejados secuencialmente es complicado. El problema es que cada `write!` 
genera un `fmt::Result`. La gestión adecuada de esto requiere tratar con *todos* 
los resultados. Para este propósito exactamente Rust proporciona el operador `?`.

El uso de `?` en `write!` tiene el siguiente aspecto:

```rust,ignore
// Probamos `write!` para comprobar si hay error. Si lo hay, retorna el error. 
// Si no, continua.
write!(f, "{}", value)?;
```

Con `?` disponible, implementar `fmt::Display` para un `Vec` es
sencillo:

```rust,editable
use std::fmt; // Importa el modulo `fmt`.

// Define una estructura llamada `List` que contiene un `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extrae el valor utilizando la indexación de tuplas, 
        // y crea una referencia a `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterar sobre `v` en `vec` mientras se enumera el número de 
        // iteraciones en `count`.
        for (count, v) in vec.iter().enumerate() {
            // Para cada elemento excepto el primero, añade una coma.
            // Utiliza el operador ? para salir en caso de error.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Cierra el corchete abierto y devuelve un valor fmt::Result.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

### Actividad

Intenta cambiar el programa para que también se imprima el índice de cada elemento del vector. La nueva salida debería ser así:

```rust,ignore
[0: 1, 1: 2, 2: 3]
```

### Ver también:

[`for`][for], [`ref`][ref], [`Result`][result], [`struct`][struct],
[`?`][q_mark], and [`vec!`][vec]

[for]: ../../../flow_control/for.md
[result]: ../../../std/result.md
[ref]: ../../../scope/borrow/ref.md
[struct]: ../../../custom_types/structs.md
[q_mark]: ../../../std/result/question_mark.md
[vec]: ../../../std/vec.md
