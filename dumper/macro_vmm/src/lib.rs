use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Path, Token, parse_macro_input, punctuated::Punctuated};

#[proc_macro_derive(TryIntoVmm, attributes(try_into_vmm_types))]
pub fn derive_try_into_vmm(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident;

    let mut types_to_implement = Vec::new();

    // Extract attribute: #[try_into_vmm_types(Type1, Type2)]
    for attr in input.attrs {
        if attr.path().is_ident("try_into_vmm_types") {
            // Use new-style parsing
            if let Ok(parsed) =
                attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
            {
                for path in parsed {
                    types_to_implement.push(quote! { #path });
                }
            } else {
                println!("Error parsing attribute: #[try_into_vmm_types]");
            }
        }
    }

    // Generate implementations for each specified type
    let implementations = types_to_implement.into_iter().map(|_| {
        quote! {
            impl TryIntoVmm for #struct_name {
                async fn try_into_vmm(self) -> Result<crate::vmm::VMM, Error> {
                    let config = self.try_into_vmm_config()?;
                    config.try_into_vmm().await
                }
            }
        }
    });

    let expanded = quote! {
        #(#implementations)*
    };
    print!("{}", expanded);
    TokenStream::from(expanded)
}
