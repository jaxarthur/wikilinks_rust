#[macro_export]
macro_rules! include_static_pages {
    ( $($str:expr),*) => {
        {
            let mut temp_hashmap = HashMap::new();
            $(
                temp_hashmap.insert(String::from($str), String::from(include_str!(concat!("../static/", $str))));
            )*
            temp_hashmap
        }
    };
}