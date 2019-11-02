#[macro_use] extern crate lazy_static;
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

use std::collections::HashMap;

lazy_static! {
    static ref FINES: HashMap<String, f64> =
        hashmap![
            "fine_rate_before_7_days".to_string() => 0.35,
            "fine_rate_before_14_days".to_string() => 0.50,
            "fine_rate_after_14_days".to_string() => 0.60
        ];
}