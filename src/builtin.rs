#[macro_export]
macro_rules! builtin {
    // helper for pushing return values from subprocedures
    (@push $then:ident!($($args:tt)*) [$($tail:tt)*] $head:tt) => {
        $then!($($args)* [$head $($tail)*])
    };

    // DROP [x -- ]
    ($then:ident!($($args:tt)*) [$head:tt $($tail:tt)*] drop) => {
        $then!($($args)* [$($tail)*])
    };

    // SWAP [x y -- y x]
    ($then:ident!($($args:tt)*) [$head:tt $neck:tt $($tail:tt)*] swap) => {
        $then!($($args)* [$neck $head $($tail)*])
    };

    // DUP [x -- x x]
    ($then:ident!($($args:tt)*) [$head:tt $($tail:tt)*] dup) => {
        $then!($($args)* [$head $head $($tail)*])
    };

    // OVER [x y -- x y x]
    ($then:ident!($($args:tt)*) [$head:tt $neck:tt $($tail:tt)*] over) => {
        $then!($($args)* [$neck $head $neck $($tail)*])
    };

    // ROT [x y z -- y z x]
    ($then:ident!($($args:tt)*) [$head:tt $neck:tt $torso:tt $($tail:tt)*] rot) => {
        $then!($($args)* [$neck $torso $head $($tail)*])
    };

    // ADD [x y -- x+y]
    ($then:ident!($($args:tt)*) [$head:tt $neck:tt $($tail:tt)*] +) => {
        math!(builtin!(@push $then!($($args)*) [$($tail)*]) $head $neck +)
    };

    // NEG [x -- -x]
    ($then:ident!($($args:tt)*) [$head:tt $($tail:tt)*] !) => {
        math!(builtin!(@push $then!($($args)*) [$($tail)*]) $head !)
    };

    ($then:ident!$args:tt $stack:tt $op:tt) => {
        compile_error!(concat!("Unknown operation ", stringify!($op)))
    };
}

