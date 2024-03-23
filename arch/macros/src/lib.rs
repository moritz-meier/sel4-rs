use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let f_ident = &f.sig.ident;

    let arch = match () {
        #[cfg(feature = "armv7a")]
        () => quote!(armv7a),
    };

    quote!(
        #[naked]
        #[no_mangle]
        pub unsafe extern "C" fn _start() -> ! {
            ::core::arch::asm!("b {}", sym #arch::start::<crate::EntryImpl>, options(noreturn))
        }

        #f

        struct EntryImpl;
        impl #arch::PrimaryEntry for EntryImpl {
            unsafe extern "C" fn primary_entry() -> ! {
                #f_ident()
            }
        }
    )
    .into()
}

#[proc_macro_attribute]
pub fn secondary_entry(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let f_ident = &f.sig.ident;

    let arch = match () {
        #[cfg(feature = "armv7a")]
        () => quote!(armv7a),
    };

    quote!(
        #f

        impl #arch::SecondaryEntry for EntryImpl {
            unsafe extern "C" fn secondary_entry(cpu_id: usize) -> ! {
                #f_ident(cpu_id)
            }
        }
    )
    .into()
}
