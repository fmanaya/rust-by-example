# Arrays y Slices

Un array es una colección de objetos del mismo tipo `T`, almacenados en una zona de memoria 
contigua. Los arrays se crean utilizando corchetes `[]`, y su longitud, que se 
conoce en tiempo de compilación, forma parte de su definición de tipo `[T; longitud]`.

Las slices son similares a los arrays, pero su longitud no se conoce en tiempo 
de compilación. En su lugar, una slice es un objeto de dos palabras, la primera 
palabra es un puntero a los datos, y la segunda es la longitud de la slice. 
El tamaño de la palabra es el mismo que usize, determinado por la 
arquitectura del procesador, por ejemplo, 64 bits en un x86-64. Las slices pueden 
utilizarse para tomar prestada una sección de una matriz, y tienen la definición 
de tipo `&[T]`.



```rust,editable,ignore,mdbook-runnable
use std::mem;

// Esta función toma prestada una slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Array de tamaño fijo (la definición de tipo) es innecesaria.
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Todos los elementos pueden ser inicializados con el mismo valor.
    let ys: [i32; 500] = [0; 500];

    // La indexación empieza en 0
    println!("primer elemento del array: {}", xs[0]);
    println!("segundo elemento del: {}", xs[1]);

    // `len` devuelve el número de elementos de la matriz
    println!("numero de elementos en el array: {}", xs.len());

    // Los arrays se almacenan en zonas contiguas de memoria
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Los arrays pueden tomarse prestadas automáticamente como slices
    println!("tomamos prestado todo el array como una slice");
    analyze_slice(&xs);

    // Las slices pueden apuntar a una sección de un array
    // Son de la forma [índice_inicial..índice_final] 
    // índice_inicial es la primera posición de la slice  
    // índice_final es una posición más que la última de la slice
    println!("toma prestada una sección del array como una slice");
    analyze_slice(&ys[1 .. 4]);

    // La indexación fuera de límites provoca un error de compilación
    println!("{}", xs[5]);
}
```
