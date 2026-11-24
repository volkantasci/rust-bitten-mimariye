use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// `#[derive(Tanit)]`: struct'ın adını ve alan adlarını derleme anında okuyup
/// `tip_adi()` ve `alan_adlari()` metotlarını üretir.
#[proc_macro_derive(Tanit)]
pub fn tanit(girdi: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(girdi as DeriveInput); // belirteçler → söz dizimi ağacı
    let ad = &ast.ident;
    let alanlar: Vec<String> = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => f.named.iter().map(|a| a.ident.as_ref().unwrap().to_string()).collect(),
            _ => Vec::new(),
        },
        _ => {
            return syn::Error::new_spanned(ad, "Tanit yalnızca struct'lar için türetilebilir")
                .to_compile_error()
                .into();
        }
    };
    let ad_metni = ad.to_string();
    quote! { // söz dizimi ağacı → yeni belirteçler
        impl #ad {
            pub fn tip_adi() -> &'static str { #ad_metni }
            pub fn alan_adlari() -> &'static [&'static str] { &[#(#alanlar),*] }
        }
    }
    .into()
}
