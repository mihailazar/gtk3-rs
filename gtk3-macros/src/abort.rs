// Take a look at the license at the top of the repository in the LICENSE file.

// In-crate replacement for the unmaintained proc-macro-error (RUSTSEC-2024-0370).
use proc_macro::TokenStream;
use std::panic::{catch_unwind, panic_any, resume_unwind, AssertUnwindSafe};

pub(crate) struct Abort(pub(crate) syn::Error);

pub(crate) fn raise(err: syn::Error) -> ! {
    panic_any(Abort(err))
}

pub(crate) fn guard(f: impl FnOnce() -> TokenStream) -> TokenStream {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(ts) => ts,
        Err(payload) => match payload.downcast::<Abort>() {
            Ok(abort) => abort.0.to_compile_error().into(),
            Err(other) => resume_unwind(other),
        },
    }
}

macro_rules! abort_call_site {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {
        $crate::abort::raise(syn::Error::new(proc_macro2::Span::call_site(), format!($fmt $(, $arg)*)))
    };
    ($msg:expr) => {
        $crate::abort::raise(syn::Error::new(proc_macro2::Span::call_site(), $msg))
    };
}

pub(crate) use abort_call_site;
