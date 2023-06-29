fn main() {
    let to_parse: proc_macro2::TokenStream = r#"
        "hey"
    "#
        .parse()
        .expect("infallible");
    
    let mut _stream: token_stream2::TokenStream = to_parse.into();
}
