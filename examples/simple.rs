fn main() {
    let to_parse: proc_macro2::TokenStream = r#"
    adwmn3qwbjhd32hgj32hd 23kdh jh23hd jhk2hdjk32 hjshd hjdh32 jkh32d d23ld; al;dasl;d ald"a" "a" "a"
    1 2 3 4 5.335435 0xff 9.32234423 032324.392394 /234483284 *++++ != != 1-122^^&&%$54%$^
    "#
        .parse()
        .expect("infallible");

    println!("{to_parse:#?}");
    let mut stream: token_stream2::TokenStream = to_parse.into();
    println!("{stream:#?}");
}
