//#![feature(trace_macros)] trace_macros!(true);

/// Associative array lookup
///
/// Given a key, a map, and a continuation, call the continuation with
/// the value for the given key. If the key is not in the map, bail with
/// a compile error.
#[macro_export]
macro_rules! lookup {
    // The macro works by destructively looping through the map, performing
    // a linear search for the needle.

    // The map-to-scan is empty. Abort compilation immediately.
    (@scan $needle:tt {} $pass:tt) => {
        compile_error!(concat!("KeyError: ", stringify!($needle)))
    };
    
    // This rule destructively scans the map. We check the first key to see
    // if it is equal to the needle. If so, jump to continuation. Otherwise,
    // throw away that key/value pair and keep scanning.
    (@scan $needle:tt {$key:tt : $val:tt $($k:tt : $v:tt)*} [$then:ident!($($args:tt)*)]) => {{
        /// We redefine this macro every time in order to compare the current
        /// key with the needle.
        macro_rules! _lookup_cmp {
            // Found it!
            ($needle $needle) => {
                $then!($($args)* $val)
            };
            
            // Did not find it
            ($x:tt $y:tt) => {
                lookup!(@scan $needle {$($k:$v)*} [$then!($($args)*)])
            }
        }
        
        // invoke the just-redefined comparator macro
        _lookup_cmp!($needle $key)
    }};

    // Entrypoint
    ($then:ident!$args:tt {$($k:tt : $v:tt),*} $needle:tt) => {
        lookup!(@scan $needle {$($k:$v)*} [$then!$args])
    }
}

