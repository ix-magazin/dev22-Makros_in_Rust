////////////////////////////////////////////////////////////
// Listing 1: Erste, einfache C-Makros (in der Sprache C) //
////////////////////////////////////////////////////////////


#define PI 3.14159265358979323846
#define CIRCLE_AREA(r) PI * r * r

int main(void) {
    int radius = 5;
    int area = CIRCLE_AREA(radius);
    printf("Radius: %d\nArea: %d\n", radius, area);
    return 0;
}



////////////////////////////////////////////////////////////////////////////
// Listing 2: C-Makro führt zu fehlerhafter Berechnung (in der Sprache C) //
////////////////////////////////////////////////////////////////////////////


#define SUM(a, b) (a) + (b)

int main(void) {
    printf("%d\n", SUM(1, 2) * 2); // Result is 5 instead of 6
    return 0;
}



/////////////////////////////////////////////////////////////////////
// Listing 3: Erstes, einfaches Rust-Makro (ab hier alles in Rust) //
/////////////////////////////////////////////////////////////////////


macro_rules! say_hi_to {
    ($name:expr) => {
        println!("Hi, {}!", $name);
    }
}

say_hi_to!("Rust");




//////////////////////////////////
// Listing 4: sum-Makro in Rust //
//////////////////////////////////


macro_rules! sum {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

println!("{}", sum!(1, 2) * 2); // Result is 6
println!("{}", sum!(2 * 2, 3 * 3) * 2); // Result is 26



////////////////////////////////////////
// Listing 5: Literale in Rust-Makros //
////////////////////////////////////////


macro_rules! sum {
    // +---------------+-- Note literals "rechne" and "plus"
    // |               |
    // v               v
    (rechne $a:literal plus $b:literal) => {
        $a + $b
    };
}

//                  +--------+-- Note literals here
//                  |        |
//                  v        v
println!("{}", sum!(rechne 1 plus 2) * 2); // Result is 6

let _x = 42;
//                              +-- Does not work as eval requires literal,
//                              |   not an identifier or an expression.
//                              v
// println!("{}", sum! { rechne _x plus 2 } * 2); // Result is 6





///////////////////////////////////////////
// Listing 6: Repetitions in Rust-Makros //
///////////////////////////////////////////

macro_rules! vector {
    ($($item:expr),*) => {{
        let mut vector = Vec::new();
        $(
            vector.push($item);
        )*
        vector
    }}
}

let my_vec = vector![];
let my_vec = vector![1];
let my_vec = vector![1, 2, 3];



///////////////////////////////////////////////////
// Listing 7: Optionale Fragmente in Rust-Makros //
///////////////////////////////////////////////////


macro_rules! print_all {
    ( $( $expr:expr ),+ $(,)? ) => {
        $(
            println!("{}", $expr);
        )*
    }
}

//           +-- Empty parameter list does not
//           |   work because of `+`
//           v
// print_all!();
print_all!(1, 2, 3);
print_all!(1, 2, 3,); // Note trailing comma



////////////////////////////////////////////////
// Listing 8: Aufrufvarianten von Rust-Makros //
////////////////////////////////////////////////


macro_rules! print_named {
    (a: $x:expr) => {
        println!("a = {}", $x)
    };
    (b: $x:expr, c: $y:expr) => {
        println!("b = {}, c = {}", $x, $y)
    };
}
   
print_named!(a: 123);
print_named!(b: 456, c: 789);




////////////////////////////////////////////////////////
// Listing 9: Variable Parameteranzahl in Rust-Makros //
////////////////////////////////////////////////////////


macro_rules! print_named {
    ($($name:ident: $val:expr),* $(,)?) => {
        $(
            println!("{} = {}", stringify!($name), $val);
        )*
    };
}

print_named!(
    foo: 123,
    bar: 456,
);





////////////////////////////////////////////////////////////////////////
// Listing 10: Einschränkung auf einen Identifier als Makro-Parameter //
////////////////////////////////////////////////////////////////////////


macro_rules! variable_scoping {
    ($number:ident) => {
        println!("{}", $number);
    };
}

let number = 1;
variable_scoping!(number); // Prints 1 on the screen

//                   +-- Does not work because macro 
//                   |   expects identifier
//                   v
// variable_scoping!(1 + number);



///////////////////////////////////////////////////////////
// Listing 11: Keine Forward Declaration bei Rust-Makros //
///////////////////////////////////////////////////////////


fn main() {
    println!("{}", sum(1, 2)); // Note: Forward declaration works
    
    //                +-- Does not work because macro must be defined
    //                |   before it can be used.
    //                v
    // println!("{}", _sum!(1, 2));
}

macro_rules! _sum {
    ($a:literal, $b:literal) => {
        $a + $b
    };
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}





/////////////////////////////////////////////
// Listing 12: Exportieren von Rust-Makros //
/////////////////////////////////////////////


// my_macros.rs
#[macro_export]
macro_rules! sum {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// main.rs
mod my_macros;

fn main() {
    println!("{}", sum!(1, 2));
}



///////////////////////////////////////////////////////
// Listing 13: Geplante, neue Syntax für Rust-Makros //
///////////////////////////////////////////////////////


// my_macros.rs ========================================
// Note new macro syntax including exporting
// with regular `pub` modifier. No `#[macro_export]`
// required anymore.
pub macro sum ($a:expr, $b:expr) {
    $a + $b
}

// main.rs =============================================
#![feature(decl_macro)]

mod my_macros;
use my_macros::sum;

fn main() {
    // Use imported macro `sum`
    println!("{}", sum!(1, 2));

    // Note that forward declaration works with
    // new macro syntax.
    println!("{}", sub!(1, 2));
}

macro sub($a:expr, $b:expr) {
    $a - $b
}
