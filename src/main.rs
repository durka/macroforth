#![feature(trace_macros)]

extern crate macroforth;
use macroforth::*;

macro_rules! stringfy {
    ($label:tt, $x:tt) => {
        println!("{}: {}", $label, stringify!($x))
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
    lookup!(stringfy!("a",) {a: 1, b: 2} a);
    lookup!(stringfy!("b",) {a: 1, b: 2} b);
    lookup!(lookup!(stringfy!("c->b",) {a: 1, b: 3, c: 2} ) {a: asdf, b: +, c: b} c);

    builtin!(stringfy!("drop", ) [  [   .               ] [   . .             ] [ . . . ]  ] drop ) ;
    builtin!(stringfy!("swap", ) [  [   .               ] [   . .             ] [ . . . ]  ] swap ) ;
    builtin!(stringfy!("dup",  ) [  [   .               ] [   . .             ] [ . . . ]  ] dup  ) ;
    builtin!(stringfy!("over", ) [  [   .               ] [   . .             ] [ . . . ]  ] over ) ;
    builtin!(stringfy!("rot",  ) [  [   .               ] [   . .             ] [ . . . ]  ] rot  ) ;
    builtin!(mathfy!("3+8",  )   [  [   . . .           ] [   . . . . . . . . ]            ] +    ) ;
    builtin!(mathfy!("-3+8", )   [  [ - . . .           ] [   . . . . . . . . ]            ] +    ) ;
    builtin!(mathfy!("-8+3", )   [  [ - . . . . . . . . ] [   . . .           ]            ] +    ) ;
    builtin!(mathfy!("8+-3", )   [  [   . . . . . . . . ] [ - . . .           ]            ] +    ) ;
    builtin!(mathfy!("3+-8", )   [  [   . . .           ] [ - . . . . . . . . ]            ] +    ) ;
    builtin!(mathfy!("3+-3", )   [  [   . . .           ] [ - . . .           ]            ] +    ) ;
    builtin!(mathfy!("-3+3", )   [  [ - . . .           ] [   . . .           ]            ] +    ) ;
    builtin!(mathfy!("-3",   )   [  [   . . .           ]                                  ] !    ) ;
    builtin!(mathfy!("--3",  )   [  [ - . . .           ]                                  ] !    ) ;

    control!(stringfy!("1 2 + !",) [[.] [. .] + !] []);
    control!(stringfy!("1 -1 +",) [[.] dup ! +] []);
    control!(stringfy!("3 {1 +} 1 1 - if",) [[. . .] {[.] +} [.] dup ! + if] []);
    control!(stringfy!("3 {1 +} 2 1 - if",) [[. . .] {[.] +} [. .] [- .] + if] []);
    control!(stringfy!(": - swap ! + ;   3 2 -",) [: - swap ! + ; [. . .] [. .] -] []);
    
    //lookup!(println!("{:?}", ) {a: 1, b: 2} c); //~ERROR KeyError: c
}

