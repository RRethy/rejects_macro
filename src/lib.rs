extern crate proc_macro;

use quote::quote;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitStr, Result};

struct RejectsPattern {
    pat: LitStr,
}

impl Parse for RejectsPattern {
    fn parse(input: ParseStream) -> Result<Self> {
        let pat = input.parse()?;
        Ok(RejectsPattern { pat })
    }
}

#[proc_macro]
pub fn rejects(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let RejectsPattern { pat } = parse_macro_input!(item as RejectsPattern);
    let mut expanded = proc_macro2::TokenStream::new();
    match rejects::rejects::Rejects::new(&pat.value()) {
        Ok(re) => {
            expanded.append_all(quote! {
                fn foobarbaz() -> rejects::rejects::Rejects {
                    #re
                }
            });
        }
        Err(v) => {
            println!("ERROR {:?}", v);
        }
    }
    return proc_macro::TokenStream::from(expanded);
}
