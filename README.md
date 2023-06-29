# tokenstream2
`token_stream2` is a helper crate for parsing procedural macros. 
It allows you to quickly convert from `proc_macro2::TokenStream` into a `token_stream2::TokenStream`,
which allows you to have a much easier time traversing the token stream.

## Usage
You can easily convert into `tokenstream2::TokenStream` using `.into()`.
```rs
let to_parse: proc_macro2::TokenStream = r#"
        fn main() {
            println!("Hello world!");
        }
    "#
    .parse()
    .expect("infallible");

let stream: token_stream2::TokenStream = to_parse.into();
```

`token_stream2::TokenStream` implements `Iterator`, so you can use the various `Iterator` methods on it. 

It also has it's own `.peek()` method you can use to quickly look ahead, since that will likely be a common behavior.

## Examples
You can look in the `/examples` directory to see an example of it in use.