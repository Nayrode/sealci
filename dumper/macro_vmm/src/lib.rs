use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TryIntoVmm)]
pub fn derive_try_into_vmm(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident;

    let expanded = quote! {
        impl TryIntoVmm for #struct_name {
            fn try_into(self) -> Result<VMM, crate::common::error::Error> {
                let config: VmmConfig = TryIntoVmmConfig::try_into(self)?;
                VMM::new(config)
            }
        }
    };

    TokenStream::from(expanded)
}
