#![feature(trace_macros)]
#![recursion_limit="512"]

extern crate macroforth;
use macroforth::*;

macro_rules! stringfy {
    ($label:tt, $x:tt) => {
        println!("{}: {}", $label, stringify!($x))
    }
}

macro_rules! strerror {
    ($label:tt, $x:tt) => {
        println!("ERROR: {}: {}", $label, stringify!($x))
    }
}

macro_rules! mathfy {
    (@count ($mag:expr) [] [$label:tt $copy:tt $sign:expr]) => {
        println!("{} = {} = {}", $label, stringify!($copy), $sign * $mag)
    };

    (@count ($($mag:tt)*) [. $($x:tt)*] $pass:tt) => {
        mathfy!(@count ($($mag)* + 1) [$($x)*] $pass)
    };

    ($label:tt, [[- $($x:tt)*]]) => {
        mathfy!(@count (0) [$($x)*] [$label [- $($x)*] -1])
    };

    ($label:tt, [$x:tt]) => {
        mathfy!(@count (0) $x [$label $x 1])
    }
}

fn main() {
    lookup!(stringfy!("a",) strerror!("KeyError",) {a: 1, b: 2} a);
    lookup!(stringfy!("b",) strerror!("KeyError",) {a: 1, b: 2} b);
    lookup!(stringfy!("b",) strerror!("KeyError",) {a: 1, b: 2, b: 3} b);
    lookup!(println!("{:?}", ) strerror!("KeyError",) {a: 1, b: 2} c); //~ERROR KeyError: c
    lookup!(lookup!(stringfy!("c->b",) strerror!("KeyError",) {a: 1, b: 3, c: 2} ) strerror!("KeyError",) {a: asdf, b: +, c: b} c);

    builtin!(stringfy!("drop", ) strerror!("unknown op",) [  [   .               ] [   . .             ] [ . . . ]  ] drop ) ;
    builtin!(stringfy!("swap", ) strerror!("unknown op",) [  [   .               ] [   . .             ] [ . . . ]  ] swap ) ;
    builtin!(stringfy!("dup",  ) strerror!("unknown op",) [  [   .               ] [   . .             ] [ . . . ]  ] dup  ) ;
    builtin!(stringfy!("over", ) strerror!("unknown op",) [  [   .               ] [   . .             ] [ . . . ]  ] over ) ;
    builtin!(stringfy!("rot",  ) strerror!("unknown op",) [  [   .               ] [   . .             ] [ . . . ]  ] rot  ) ;
    builtin!(mathfy!("3+8",  )   strerror!("unknown op",) [  [   . . .           ] [   . . . . . . . . ]            ] +    ) ;
    builtin!(mathfy!("-3+8", )   strerror!("unknown op",) [  [ - . . .           ] [   . . . . . . . . ]            ] +    ) ;
    builtin!(mathfy!("-8+3", )   strerror!("unknown op",) [  [ - . . . . . . . . ] [   . . .           ]            ] +    ) ;
    builtin!(mathfy!("8+-3", )   strerror!("unknown op",) [  [   . . . . . . . . ] [ - . . .           ]            ] +    ) ;
    builtin!(mathfy!("3+-8", )   strerror!("unknown op",) [  [   . . .           ] [ - . . . . . . . . ]            ] +    ) ;
    builtin!(mathfy!("3+-3", )   strerror!("unknown op",) [  [   . . .           ] [ - . . .           ]            ] +    ) ;
    builtin!(mathfy!("-3+3", )   strerror!("unknown op",) [  [ - . . .           ] [   . . .           ]            ] +    ) ;
    builtin!(mathfy!("-3",   )   strerror!("unknown op",) [  [   . . .           ]                                  ] !    ) ;
    builtin!(mathfy!("--3",  )   strerror!("unknown op",) [  [ - . . .           ]                                  ] !    ) ;

    control!(stringfy!("1 2 +",) [[.] [. .] +] {} []);
    control!(stringfy!("1 2 + !",) [[.] [. .] + !] {} []);
    control!(stringfy!("1 -1 +",) [[.] dup ! +] {} []);
    control!(stringfy!("3 {1 +} 1 1 - if",) [[. . .] {[.] +} [.] dup ! + if] {} []);
    control!(stringfy!("3 {1 +} 2 1 - if",) [[. . .] {[.] +} [. .] [- .] + if] {} []);
    control!(stringfy!("3 8 -  8 3 -",)
        [
            : - swap ! + ;
            [. . .] [. . . . . . . .] -
            [. . . . . . . .] [. . .] -
        ] {} []);
    control!(stringfy!("3 8 - abs  8 3 - abs",)
        [
            : - swap ! + ;
            : abs dup ! ~ {!} swap if ;
            : 3 [. . .] ;
            : 8 3 3 + [. .] + ;
            3 8 - abs
            8 3 - abs
        ] {} []);
}

