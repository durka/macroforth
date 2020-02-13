#[macro_export]
macro_rules! builtin {
    // helper for pushing return values from subprocedures
    (@push $then:ident!($($targs:tt)*) $else:ident!$eargs:tt [$($tail:tt)*] $head:tt) => {
        $then!($($targs)* [$head $($tail)*])
    };

    // DROP [x -- ]
    ($then:ident!($($targs:tt)*) $else:ident!$eargs:tt [$head:tt $($tail:tt)*] drop) => {
        $then!($($targs)* [$($tail)*])
    };

    // SWAP [x y -- y x]
    ($then:ident!($($targs:tt)*) $else:ident!$eargs:tt [$head:tt $neck:tt $($tail:tt)*] swap) => {
        $then!($($targs)* [$neck $head $($tail)*])
    };

    // DUP [x -- x x]
    ($then:ident!($($targs:tt)*) $else:ident!$eargs:tt [$head:tt $($tail:tt)*] dup) => {
        $then!($($targs)* [$head $head $($tail)*])
    };

    // OVER [x y -- x y x]
    ($then:ident!($($targs:tt)*) $else:ident!$eargs:tt [$head:tt $neck:tt $($tail:tt)*] over) => {
        $then!($($targs)* [$neck $head $neck $($tail)*])
    };

    // ROT [x y z -- y z x]
    ($then:ident!($($targs:tt)*) $else:ident!$eargs:tt [$head:tt $neck:tt $torso:tt $($tail:tt)*] rot) => {
        $then!($($targs)* [$neck $torso $head $($tail)*])
    };

    // ADD [x y -- x+y]
    ($then:ident!$targs:tt $else:ident!$eargs:tt [$head:tt $neck:tt $($tail:tt)*] +) => {
        math!(builtin!(@push $then!$targs $else!$eargs [$($tail)*]) $head $neck +)
    };

    // NEG [x -- -x]
    ($then:ident!$targs:tt $else:ident!$eargs:tt [$head:tt $($tail:tt)*] !) => {
        math!(builtin!(@push $then!$targs $else!$eargs [$($tail)*]) $head !)
    };

    // SIGN [x -- s] (s == 1 if x > 0, 0 otherwise)
    ($then:ident!$targs:tt $else:ident!$eargs:tt [$head:tt $($tail:tt)*] ~) => {
        math!(builtin!(@push $then!$targs $else!$eargs [$($tail)*]) $head ~)
    };

    ($then:ident!$targs:tt $else:ident!($($eargs:tt)*) $stack:tt $op:tt) => {
        $else!($($eargs)* $op)
    };
}

