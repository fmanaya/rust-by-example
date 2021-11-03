use std::fmt::{self, Display, Formatter};

// Las tuplas se pueden utilizar como argumentos de funciones y como valores de retorno
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` puede utilizarse para vincular los miembros de una tupla a variables
    let (integer, boolean) = pair;

    (boolean, integer)
}
//
fn transpose(mat: Matrix) -> Matrix {
    Matrix(mat.0, mat.2, mat.1, mat.3)
}

// La siguiente estructura es para la actividad.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    // `f` es un buffer, y este metodo debe escribir la cadena formateada en el
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // Una tupla con un conjunto de tipos diferentes
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Los valores pueden extraerse de la tupla por su índice
    println!("long_tuple primer valor: {}", long_tuple.0);
    println!("long_tuple segundo valor: {}", long_tuple.1);

    // Las tuplas a su vez pueden ser miembros de tuplas
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuplas son imprimibles
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // Pero tuplas grandes no lo son
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Descomente las 2 líneas anteriores para ver el error del compilador

    let pair = (1, true);
    println!("pair es {:?}", pair);

    println!("pais a la inversa es {:?}", reverse(pair));

    // Para crear tuplas de un elemento, la coma es necesaria para
    // distinguirlas de un literal entre paréntesis
    println!("una tupla: {:?}", (5u32,));
    println!("simplemente un entero: {:?}", (5u32));

    //las tuplas pueden ser desestructuradas para crear enlaces
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix");
    println!("{}", matrix);
    println!("Transpose");
    println!("{}", transpose(matrix));
}
