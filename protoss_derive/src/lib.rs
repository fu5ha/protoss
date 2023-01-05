//! Procedural macros for `protoss`.
//! 
//! The `#[protoss::evolving]` macro is the main driver. You should apply it to the main 'type definition', like so:
//! 
//! ```rust,no_run
//! #[protoss::evolving]
//! pub struct Test {
//!     #[field(id = 0, since_ev = 0)]
//!     pub a: u32,
//!     #[field(id = 1, since_ev = 0)]
//!     pub b: u8,
//!     #[field(id = 2, since_ev = 1)]
//!     pub c: u32,
//!     #[field(id = 3, since_ev = 2)]
//!     pub d: u8,
//! }
//! ```
//! 
//! Each field must have a defined `id`, which is a unique, monotonically increasing identifier of each field that must remain
//! stable across all future evolutions of the same type. This means that the field with a given `id` must always continue to have
//! the same type in all evolutions of a type, and new fields should always get the next `id` number.
//! 
//! `id`s exist to allow fields to be renamed and shuffled in 'visual` order on the type definition while still retaining the same
//! order and therefore layout in the underlying archived evolutions.
//! 
//! One may add multiple fields at the same time within the same evolution, but once an existing evolution is in use, to add a new
//! field you must create a new evolution (think of it as a minor-version number).
//! 
//! The above definition would generate:
//! - An "evolution" struct for each defined evolution, in this case 0, 1, and 2: `TestEv0`, `TestEv1`, and `TestEv2`
//! - The concrete, padded, explicit archived representation of each evolution, `ArchivedTestEv0`, `ArchivedTestEv1`, `ArchivedTestEv2`
//! - A concrete probe type, `TestProbe`
//! - implementations for relevant traits `protoss` traits: `impl Evolving for Test`, `impl Evolution for TestEvX`, and `impl Probe for TestProbe`
//! - implementations for relevant `rkyv` traits:
//!     - `impl rkyv::{Archive, Serialize} for TestEvX` as `ArchivedTestEvX`
//!     - `impl rkyv::{Archive, Serialize} for Test` as `<Test as Evolving>::LatestEvolution`
//!
//! See `protoss_test/src/manual.rs` for an example of what all this implementation would actually look like when expanded.
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]

mod evolving;

extern crate proc_macro;

use syn::{ItemStruct, Meta, parse_macro_input};

/// See crate level documentation for now
#[proc_macro_attribute]
pub fn evolving(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr as Meta))
    };

    let mut input = parse_macro_input!(item as ItemStruct);
    input.generics.make_where_clause();

    match evolving::expand(&attr, &input) {
        Ok(result) => result.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
