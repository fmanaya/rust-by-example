# Comentarios

Cualquier programa necesita comentarios, y Rust los contempla en varias formas:


* *Comentarios normales* que son ignorados por el compilador:
   * `// Line comments which go to the end of the line.`
   * `/* Block comments which go to the closing delimiter. */`
* *Comentarios de documentación* que se transforman con la libreria HTML de 
  [documentación][docs]:
   * `/// Generate library docs for the following item.`
   * `//! Generate library docs for the enclosing item.`

```rust,editable
fn main() {
    // Este es un ejemplo de comentario de linea
    // Hay dos slashes al principio de linea
    // Y nada escrito despues es leido por el compilador

    // println!("Hello, world!");

    // Ejecuta el script. Ves? Ahora intenta borrar los dos slashes y ejecutalo de nuevo.

    /* 
     * Este es otro tipo de comentario, comentario de bloque. En general 
     * los comentarios de línea son los recomendados. Pero los comentarios 
     * de bloque son muy utiles para desactivar temporalmente trozos de 
     * código. /* Los comentarios de bloque se pueden /* anidar */ */ asi 
     * solo nos llevará unos pocas pulsaciones comentar toda esta funcion 
     * main() /*/*/* Pruebalo tu mismo */*/*/
     */

    /*
    Nota: La anterior columna de `*` es solo por guardar el estilo. Realmente 
    no es necesario ponerla.
    */

    // Puedes manipular expresiones mas facilmente con comentarios de bloque
    // que con comentarios de elinea. Prueba a borrar los delimitadores para cambiar el resultado
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

```

### Ver también:

[Library documentation][docs]

[docs]: ../meta/doc.md
