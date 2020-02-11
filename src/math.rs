#[macro_export]
macro_rules! math {
    // addition: - 0
    ($then:ident!($($args:tt)*) [- $($a:tt)*] [] +) => {
        $then!($($args)* [- $($a)*])
    };

    // addition: -1 +
    ($then:ident!($($args:tt)*) [- .] [. $($b:tt)*] +) => {
        $then!($($args)* [$($b)*])
    };

    // addition: - +
    ($then:ident!($($args:tt)*) [- . $($a:tt)*] [. $($b:tt)*] +) => {
        math!($then!($($args)*) [- $($a)*] [$($b)*] +)
    };

    // addition: + -0
    ($then:ident!($($args:tt)*) [$($a:tt)*] [-] +) => {
        $then!($($args)* [$($a)*])
    };

    // addition: 0 -
    ($then:ident!($($args:tt)*) [] [- $($b:tt)*] +) => {
        $then!($($args)* [- $($b)*])
    };

    // addition: + -
    ($then:ident!($($args:tt)*) [. $($a:tt)*] [- . $($b:tt)*] +) => {
        math!($then!($($args)*) [$($a)*] [- $($b)*] +)
    };

    // addition: + +
    ($then:ident!($($args:tt)*) [$($a:tt)*] [$($b:tt)*] +) => {
        $then!($($args)* [$($a)* $($b)*])
    };

    // negation: -
    ($then:ident!($($args:tt)*) [- $($a:tt)*] !) => {
        $then!($($args)* [$($a)*])
    };

    // negation: +
    ($then:ident!($($args:tt)*) [$($a:tt)*] !) => {
        $then!($($args)* [- $($a)*])
    };

    // negative? -
    ($then:ident!($($args:tt)*) [- $($a:tt)*] ~) => {
        $then!($($args)* [.])
    };

    // negative? +
    ($then:ident!($($args:tt)*) [$($a:tt)*] ~) => {
        $then!($($args)* [])
    };

    ($then:ident!($($args:tt)*) $($x:tt)*) => {
        compile_error!(concat!("Unknown math expression: ", stringify!($($x)*)))
    };
}

