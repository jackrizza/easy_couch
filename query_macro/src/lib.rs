extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(QueryMacro)]
pub fn query_macro(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Queries<#name> for #name {
            fn query_fmt(&self) -> Result<Value, String> {
                Ok(serde_json::to_value(self).unwrap())
            }
            fn query<T: QueryGeneric<T>>(&self, input: Value) -> Result<T, String> {
                let res: T = T::new_with_input(QGEnum::Val(input));
                Ok(res)
            }
        }
    };
    gen.into()
}
