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

//  calcule el área de un Rectangle (intenta utilizando la desestructuración anidada).
fn rect_area(rect: Rectangle) -> f32 {
    //ancho * alto
    /*
    let Rectangle {
        Point { x: left_edge, y: top_edge },
        Point { x: left_edge, y: top_edge }
    } = rect;*/
    println!("================");
    println!("tl.x={}, tl.y={}", rect.top_left.x, rect.top_left.y);
    println!("br.x={}, br.y={}", rect.bottom_right.x, rect.bottom_right.y);

    let ancho = rect.bottom_right.x - rect.top_left.x;
    let alto =  rect.top_left.y - rect.bottom_right.y;
    println!("ancho={}, alto={}", ancho, alto);
    println!("================");
    (ancho * alto).abs()
}

/*Añade una función square que toma un Point y un f32 como argumentos,
y devuelve un Rectangle con su esquina inferior izquierda en el punto,
y una anchura y altura correspondientes a f32.*/
fn square(pt: Point, dim: f32) -> Rectangle {
    println!("SQUARE desde el punto: ({}, {})", pt.x, pt.y);
    let tlx = pt.x;
    let tly = pt.y + dim;
    let brx = pt.x + dim;
    let bry = pt.y;
    println!("SQUARE; tlx={}, tly={}", tlx, tly);

    Rectangle {
        top_left: Point { x: tlx, y: tly },
        bottom_right: Point { x: brx, y: bry },
    }
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
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let mut _rectangle = Rectangle {
        // La instanciación de la estructura también es una expresión
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    _rectangle.top_left.y = 3.3;
    println!("area de: {}", rect_area(_rectangle));
    println!("square, area de: {}", rect_area(square(point, 25.2)));
    // Instancia una estructura unitaria
    let _unit = Unit;

    // Instancia una estructura de tupla
    let pair = Pair(1, 0.1);

    // Acceso a los campos de una estructura de tupla
    println!("pair contiene {:?} y {:?}", pair.0, pair.1);

    // Destructura una estructura de tupla
    let Pair(integer, decimal) = pair;

    println!("pair contiene {:?} y {:?}", integer, decimal);
}
