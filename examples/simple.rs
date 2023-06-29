fn main() {
    let to_parse: proc_macro2::TokenStream = r#"
    <html>
        <h1>{ "Hello world!" }</h1>
    </html>
    "#
        .parse()
        .expect("infallible");
    
    println!("{to_parse:#?}");
    let mut stream: token_stream2::TokenStream = to_parse.into();
    println!("{stream:#?}");
}
