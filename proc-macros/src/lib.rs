extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro]
pub fn stringify_to_bytes(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::Ident);
    let str = syn::LitByteStr::new(&input.to_string().as_bytes(), input.span());
    let out = quote::quote! { #str };
    return out.into();
}

#[proc_macro]
pub fn str_u8_array(item: TokenStream) -> TokenStream {
    // println!("{}", item.to_string());
    let input = syn::parse_macro_input!(item as syn::LitStr);
    let byte_str = syn::LitByteStr::new(input.value().as_bytes(), input.span());

    let out = quote::quote! {
        #byte_str
    };
    return out.into();
}
