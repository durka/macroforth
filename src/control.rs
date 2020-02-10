#[macro_export]
macro_rules! control {
    // IF [cond code --] runs code if cond is zero
    ($then:ident!($($args:tt)*) [$($program:tt)*] [[] {$($code:tt)*} $($tail:tt)*] if) => {
        control!($then!($($args)*) [$($code)* $($program)*] [$($tail)*])
    };
    ($then:ident!($($args:tt)*) [$($program:tt)*] [$cond:tt $code:tt $($tail:tt)*] if) => {
        control!($then!($($args)*) [$($program)*] [$($tail)*])
    };

    // push number
    ($then:ident!($($args:tt)*) $program:tt [$($stack:tt)*] [$($x:tt)*]) => {
        control!($then!($($args)*) $program [[$($x)*] $($stack)*])
    };
    // push code
    ($then:ident!($($args:tt)*) $program:tt [$($stack:tt)*] {$($x:tt)*}) => {
        control!($then!($($args)*) $program [{$($x)*} $($stack)*])
    };

    // builtin
    ($then:ident!($($args:tt)*) $program:tt $stack:tt $op:tt) => {
        builtin!(control!($then!($($args)*) $program) $stack $op)
    };

    // run next instruction
    ($then:ident!($($args:tt)*) [$start:tt $($program:tt)*] $stack:tt) => {
        control!($then!($($args)*) [$($program)*] $stack $start)
    };

    // end of program
    ($then:ident!($($args:tt)*) [] $stack:tt) => {
        $then!($($args)* $stack)
    };
}

