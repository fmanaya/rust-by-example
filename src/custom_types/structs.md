# Estructuras

Existen tres tipos de estructuras ("structs") que pueden crearse con la etiqueta `struct`:

* Structs de tupla, que son, básicamente, tuplas con nombre.
* Las clásicas [estructuras de C][c_struct]
* Las estructuras unitarias, que no tienen propiedades, son útiles para los genéricos.


```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Una estructura unitaria
struct Unit;

// Una estructura tupla
struct Pair(i32, f32);

// Una estructura con dos campos
struct Point {
    x: f32,
    y: f32,
}

// Las estructuras se pueden usar como campos de otra estructura
#[allow(dead_code)]
struct Rectangle {
    // Un rectángulo se puede definir por el lugar que ocupan en el 
    // espacio las esquinas superior izquierda e inferior derecha.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Crea una estructura con atajo de inicializacion de campos
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Imprime la estructura modo debug
    println!("{:?}", peter);


    // Instancia un `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Accso a los campos de point
    println!("coordenadas del punto: ({}, {})", point.x, point.y);

    // Crear un nuevo punto utilizando la sintaxis struct update para 
    // utilizar los campos de nuestro otro punto
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` será el mismo que `point.y` porque hemos usado aquel campo
    // de `point`
    println!("segundo punto: ({}, {})", bottom_right.x, bottom_right.y);

    // Desestructurar point utilizando un enlace `let`.
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // La instanciación de la estructura también es una expresión
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instancia una estructura unitaria
    let _unit = Unit;

    // Instancia una estructura de tupla
    let pair = Pair(1, 0.1);

    // Acceso a los campos de una estructura de tupla
    println!("pair contiene {:?} y {:?}", pair.0, pair.1);

    // Destructura una estructura de tupla
    let Pair(integer, decimal) = pair;

    println!("pair contiene {:?} and {:?}", integer, decimal);
}
```

### Actividad

1. Añade una función `rect_area` que calcule el área de un `Rectangle` 
   (intenta utilizando la desestructuración anidada).
2. Añade una función `square` que toma un `Point` y un `f32` como argumentos, 
   y devuelve un `Rectangle` con su esquina inferior izquierda en el punto, 
   y una anchura y altura correspondientes a `f32`.

### Ver también

[`atributos`][attributes] y [desestructurando][destructuring]

[attributes]: ../attribute.md
[c_struct]: https://en.wikipedia.org/wiki/Struct_(C_programming_language)
[destructuring]: ../flow_control/match/destructuring.md

#https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#unit-like-structs-without-any-fields