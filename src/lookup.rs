/// Associative array lookup
///
/// Given a key, a map, and continuations for success and failure, call
/// the success continuation with the value for the given key. If the
/// key is not in the map, pass the key to the failure continuation.
#[macro_export]
macro_rules! lookup {
    // trampoline to dispatch to the success continuation
    (@found $then:ident!($($targs:tt)*) $val:tt) => {
        $then!($($targs)* $val)
    };

    // entrypoint
    ($then:ident!$targs:tt $else:ident!($($eargs:tt)*) {$($key:tt : $val:tt),*} $needle:tt) => {{
        // construct a macro with one arm for each key, plus a catch-all
        macro_rules! _lookup {
            $(
                ($key) => {
                    // this indirection is necessary because we can't do $($targs)* while already
                    // repeating on $key:$val
                    lookup!(@found $then!$targs $val)
                };
            )*
            // failure case
            ($t:tt) => {
                $else!($($eargs)* $needle)
            }
        }

        // call the just-constructed macro
        _lookup!($needle)
    }}
}

