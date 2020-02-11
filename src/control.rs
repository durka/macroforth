#[macro_export]
macro_rules! control {
    // run subprocedure returned from lookup!()
    (@eval $then:ident!($($args:tt)*) [$($program:tt)*] $words:tt $stack:tt {$($code:tt)*}) => {
        control!($then!($($args)*) [$($code)* $($program)*] $words $stack)
    };

    // IF [cond code --] runs code if cond is nonzero
    ($then:ident!($($args:tt)*) [$($program:tt)*] $words:tt [[] $code:tt $($tail:tt)*] if) => {
        control!($then!($($args)*) [$($program)*] $words [$($tail)*])
    };
    ($then:ident!($($args:tt)*) [$($program:tt)*] $words:tt [$cond:tt {$($code:tt)*} $($tail:tt)*] if) => {
        control!($then!($($args)*) [$($code)* $($program)*] $words [$($tail)*])
    };

    // push number
    ($then:ident!($($args:tt)*) $program:tt $words:tt [$($stack:tt)*] [$($x:tt)*]) => {
        control!($then!($($args)*) $program $words [[$($x)*] $($stack)*])
    };
    // push code
    ($then:ident!($($args:tt)*) $program:tt $words:tt [$($stack:tt)*] {$($x:tt)*}) => {
        control!($then!($($args)*) $program $words [{$($x)*} $($stack)*])
    };

    // start definition
    ($then:ident!($($args:tt)*) $program:tt $words:tt $stack:tt :) => {
        control!(@define $then!($($args)*) $program $words $stack {})
    };
    // end definition
    (@define $then:ident!($($args:tt)*) [; $($program:tt)*] {$($k:tt : $v:tt),*} $stack:tt {$name:tt $($word:tt)*}) => {
        control!($then!($($args)*) [$($program)*] {$name: {$($word)*} $(, $k : $v)*} $stack)
    };
    // continue definition
    (@define $then:ident!($($args:tt)*) [$op:tt $($program:tt)*] $words:tt $stack:tt {$($word:tt)*}) => {
        control!(@define $then!($($args)*) [$($program)*] $words $stack {$($word)* $op})
    };

    // run operator
    ($then:ident!($($args:tt)*) $program:tt $words:tt $stack:tt $op:tt) => {
        lookup!(control!(@eval $then!($($args)*) $program $words $stack)
                builtin!(control!($then!($($args)*) $program $words)
                         control!($then!($($args)*) $program $words $stack)
                         $stack)
                $words $op)
    };

    // run next instruction
    ($then:ident!($($args:tt)*) [$start:tt $($program:tt)*] $words:tt $stack:tt) => {
        control!($then!($($args)*) [$($program)*] $words $stack $start)
    };

    // end of program
    ($then:ident!($($args:tt)*) [] $words:tt $stack:tt) => {
        $then!($($args)* $stack)
    };
}

