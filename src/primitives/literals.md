# Literales y operadores

Los tipos entero `1`, punto flotante `1.2`, carácter `'a'`, cadena de texto `"abc"`, boleano `true`
y tipo unitario `()` se pueden expresar usando literales.

Los enteros alternativamente pueden ser expresados con una notación
hexadecimal, octal o binaria usando los respectivos prefijos : `0x`, `0o` or `0b`.

El símbolo de subrayado se puede insertar en literales numéricos para mejorar la legibilidad, p.e.
`1_000` es lo mismo que `1000`, y `0.000_001` es lo mismo que `0.000001`.

Tenemos que indicar al compilador el tipo de los literales que utilizamos. Por ahora, 
usaremos el sufijo `u32` para indicar que el literal es un entero de 32 bits sin signo 
y el sufijo `i32` para indicar que es un entero de 32 bits con signo.

Los operadores disponibles y su precedencia [en Rust][rust op-prec] son similares a otros 
[lenguajes como C][op-prec].

```rust,editable
fn main() {
    // Suma de enteros
    println!("1 + 2 = {}", 1u32 + 2);

    // Resta de enteros
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Prueba a cambiar `1i32` por `1u32` para ver por qué el tipo es importante

    // Evaluación de cortocircuito en logica booleana
    println!("verdadero AND falso es {}", true && false);
    println!("verdadero OR falso es {}", true || false);
    println!("NO verdadero es {}", !true);

    // Operaciones bit a bit
    println!("0011 AND 0101 es {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 es {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 es {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 es {}", 1u32 << 5);
    println!("0x80 >> 2 es 0x{:x}", 0x80u32 >> 2);

    // Utilice guiones bajos para mejorar la legibilidad!
    println!("Un millon se escribe como {}", 1_000_000u32);
}
```

[rust op-prec]: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
[op-prec]: https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages
