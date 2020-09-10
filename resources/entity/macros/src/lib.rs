use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bind(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let item = item.to_string();
    assert!(attr.len() > 0);

    let mut it = item.split(|c| c == ' ' || c == '(' || c == '<');
    while let Some(x) = it.next() {
        if x == "fn" {
            break;
        }
    }

    let fn_name = it.next().unwrap();
    let fn_ret = attr.split(" ").next().unwrap();

    let new_fn_name = format!("__internal_{}__", fn_name);
    let new_fn = item.replacen(fn_name, &new_fn_name, 1);

    let res = format!(
        "{}

#[no_mangle]
unsafe extern \"C\" fn {} () -> {} {{
    return {};
}}
    ",
        new_fn, fn_name, fn_ret, new_fn_name
    );

    // println!("{}", res);

    res.parse().unwrap()
}
