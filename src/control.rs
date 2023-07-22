#[macro_export]
macro_rules! control {
    // run subprocedure returned from lookup!()
    (@eval $then:ident!$args:tt [$($program:tt)*] $words:tt $loops:tt $stack:tt {$($code:tt)*}) => {
        control!($then!$args [$($code)* $($program)*] $words $loops $stack)
    };

    // IF [cond code --] runs code if cond is nonzero
    ($then:ident!$args:tt [$($program:tt)*] $words:tt $loops:tt [[] $code:tt $($tail:tt)*] if) => {
        control!($then!$args [$($program)*] $words $loops [$($tail)*])
    };
    ($then:ident!$args:tt [$($program:tt)*] $words:tt $loops:tt [$cond:tt {$($code:tt)*} $($tail:tt)*] if) => {
        control!($then!$args [$($code)* $($program)*] $words $loops [$($tail)*])
    };

    // LOOP [code --] enters loop
    ($then:ident!$args:tt $program:tt $words:tt {$($loops:tt)*} [{$($code:tt)*} $($tail:tt)*] loop) => {
        control!($then!$args [$($code)*] $words {([$($code)*] $program) $($loops)*} [$($tail)*])
    };

    // BREAK ends loop early
    ($then:ident!$args:tt $loop:tt $words:tt {($code:tt $program:tt) $($loops:tt)*} $stack:tt break) => {
        control!($then!$args $program $words {$($loops)*} $stack)
    };

    // end of loop
    ($then:ident!$args:tt [] $words:tt {($code:tt $program:tt) $($loops:tt)*} $stack:tt) => {
        control!($then!$args $code $words {($code $program) $($loops)*} $stack)
    };

    // push number
    ($then:ident!$args:tt $program:tt $words:tt $loops:tt [$($stack:tt)*] [$($x:tt)*]) => {
        control!($then!$args $program $words $loops [[$($x)*] $($stack)*])
    };
    // push code
    ($then:ident!$args:tt $program:tt $words:tt $loops:tt [$($stack:tt)*] {$($x:tt)*}) => {
        control!($then!$args $program $words $loops [{$($x)*} $($stack)*])
    };

    // start definition
    ($then:ident!$args:tt $program:tt $words:tt $loops:tt $stack:tt :) => {
        control!(@define $then!$args $program $words $loops $stack {})
    };
    // end definition
    (@define $then:ident!$args:tt [; $($program:tt)*] {$($k:tt : $v:tt),*} $loops:tt $stack:tt {$name:tt $($word:tt)*}) => {
        control!($then!$args [$($program)*] {$name: {$($word)*} $(, $k : $v)*} $loops $stack)
    };
    // continue definition
    (@define $then:ident!$args:tt [$op:tt $($program:tt)*] $words:tt $loops:tt $stack:tt {$($word:tt)*}) => {
        control!(@define $then!$args [$($program)*] $words $loops $stack {$($word)* $op})
    };

    // run operator
    ($then:ident!$args:tt $program:tt $words:tt $loops:tt $stack:tt $op:tt) => {
        lookup!(control!(@eval $then!$args $program $words $loops $stack)
                builtin!(control!($then!$args $program $words $loops)
                         strerror!("unknown word or bad stack format",)
                         $stack)
                $words $op)
    };

    // run next instruction
    ($then:ident!$args:tt [$start:tt $($program:tt)*] $words:tt $loops:tt $stack:tt) => {
        control!($then!$args [$($program)*] $words $loops $stack $start)
    };

    // end of program
    ($then:ident!($($args:tt)*) [] $words:tt {} $stack:tt) => {
        $then!($($args)* $stack)
    };
}

