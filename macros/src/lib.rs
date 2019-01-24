//  In the future, there will be a second kind of declarative macro with the macro keyword that will work in a similar fashion but fix some of these edge cases. After that is done, macro_rules! will be effectively deprecated.

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// vec![1, 2, 3] is generated as:
// let mut temp_vec = Vec::new();
// temp_vec.push(1);
// temp_vec.push(2);
// temp_vec.push(3);
// temp_vec
