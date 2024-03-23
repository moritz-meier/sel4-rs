use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn rootserver(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let f_ident = &f.sig.ident;

    quote!(
        #[naked]
        #[no_mangle]
        #[link_section = ".start"]
        pub unsafe extern "C" fn _start() -> ! {
            ::core::arch::asm!("b {}", sym rootserver::start::<crate::RootserverImpl>, options(noreturn))
        }

        #f

        struct RootserverImpl;
        impl rootserver::Rootserver for RootserverImpl {
            unsafe extern "C" fn entry() -> ! {
                #f_ident()
            }
        }
    )
    .into()
}
