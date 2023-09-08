//! `protoss` implements a protocol for [schema evolution] of
//! binary serialized data, designed to be used with [`rkyv`][::rkyv].
//! 
//! **DISCLAIMER**: Things are still heavily WIP and may move/change/break at any time (sorry!) though I think there's a solid
//! useful base here.
//! 
//! It offers **full** compatibilty\* (forward and backward) and **zero-copy deserialization** among "[`Evolution`]s"
//! of a base "[`Evolving`]" type, under a restrictive set of allowed changes, similar to Flatbuffers and Cap'n Proto.
//!
//! \* *A note on compatibility types:*
//! * **Backward** compatibility means that consumers (readers of serialized data) can read data
//! *produced by* an **older** version of the schema.
//! * **Forward** compatiblity means that consumers (readers of serialized data) can read data
//! *produced by* a **newer** version of the schema.
//! 
//! [`Evolution`]s of an [`Evolving`] type are allowed to:
//! * Add new fields
//!     - These new fields are always treated as optional
//!     - Fields may only be added to the end of an existing type
//!         - *but when using the provided derive macros, if you use field `id`s, you can define them in any order in code as long as the `id`s dont change*
//! * Rename existing fields (*but not change the type*)
//!
//! They are not allowed to:
//! * Remove existing fields entirely
//! * Change the type of existing fields
//! * Re-order existing fields
//! * Add new fields to the middle of an existing type
//! 
//! You can think of this as a similar type of schema evolution as what Protocol Buffers, Flatbuffers, and Cap'n Proto
//! offer. Existing consumers expecting a previous version may still read data produced with the new version as
//! the old version, and consumers expecting the new version will still be able to read all the fields that were defined
//! by the older producer.
//! 
//! For more detail on how this works internally, see the documentation of the [`Evolving`] trait, which is the centerpiece of the `protoss`
//! model.
//! 
//! # Example
//! 
//! Here we'll explore a little example of how `protoss` may be used. We'll use the proc macro as implementing things by hand
//! is error prone and very verbose. Also, see the crate-level documentation of [`protoss_derive`] for info on how the macro works.
//! 
//! 
//! ## Defining evolving types
//! 
//! Here the `v1` and `v2` modules represent two different 'time slices' of the same codebase, which may have been
//! compiled into binaries that are meant to coexist and interoperate with each other.
//! 
//! ```
//! mod v1 {
//!     #[protoss::evolving]
//!     #[archived_evolution_attr(derive(Debug, PartialEq))]
//!     pub struct Test {
//!         #[field(id = 0, since_ev = 0)]
//!         pub a: u32,
//!         #[field(id = 1, since_ev = 0)]
//!         pub b: u8,
//!         #[field(id = 2, since_ev = 1)]
//!         pub c: u32,
//!     }
//! }
//!
//! mod v2 {
//!     #[protoss::evolving]
//!     #[archived_evolution_attr(derive(Debug, PartialEq))]
//!     pub struct Test {
//!         #[field(id = 1, since_ev = 0)]
//!         pub b: u8,
//!         #[field(id = 2, since_ev = 1)]
//!         pub c: u32,
//!         #[field(id = 3, since_ev = 2)]
//!         pub d: u8,
//!         #[field(id = 0, since_ev = 0)]
//!         pub a_renamed: u32,
//!     }
//! }
//! ```
//! 
//! In `v1`, we have an [`Evolving`] type `Test` which currently has two evolutions, 0 and 1.
//! Evolution 0 has two fields called `a` (id 0) and `b` (id 1). Evolution 1 has one more field, `c` (id 2).
//! 
//! In `v2`, we have that same type `Test`, but this time with three evolutions. Evolution 2 has another field, `d` (id 3).
//! You may notice that in the definition, we've moved the field `a` to be defined last and renamed it to `a_renamed`,
//! but it still has the same type (`u32`) and id (0).
//! 
//! ## Using evolving types
//! 
//! Although `Test` implements [`Evolving`], it still by default serializes as the *static* version of its latest
//! evolution (i.e., within the context of `v1`, directly as `ArchivedTestEv1`). In order to get the benefits of evolution, we need
//! to use the `ArchiveWith` modifier, [`Evolve`]. For example, say we have some containing type, `Container`, into which
//! we want to serialize a `Test` that may evolve over time. We'd define that like so:
//! 
//! ```
//! # mod v1 {
//! #    #[protoss::evolving]
//! #    #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #    pub struct Test {
//! #        #[field(id = 0, since_ev = 0)]
//! #        pub a: u32,
//! #        #[field(id = 1, since_ev = 0)]
//! #        pub b: u8,
//! #        #[field(id = 2, since_ev = 1)]
//! #        pub c: u32,
//! #    }
//! # }
//! use protoss::Evolve;
//! 
//! #[derive(rkyv::Archive, rkyv::Serialize)]
//! struct Container {
//!     #[with(Evolve)]
//!     test: v1::Test,
//! }
//! ```
//! 
//! Now, the `test` field of `Container` will have an archived type (i.e. serialize as) an [`ArchivedEvolution<Test>`]
//! 
//! This means that in order to access it, we'll need to probe it to find out which evolution it contains and thus which versions it has.
//! 
//! Say we create and serialize a `Container` like so:
//! 
//! ```
//! # mod v1 {
//! #    #[protoss::evolving]
//! #    #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #    pub struct Test {
//! #        #[field(id = 0, since_ev = 0)]
//! #        pub a: u32,
//! #        #[field(id = 1, since_ev = 0)]
//! #        pub b: u8,
//! #        #[field(id = 2, since_ev = 1)]
//! #        pub c: u32,
//! #    }
//! # }
//! # use protoss::Evolve;
//! # #[derive(rkyv::Archive, rkyv::Serialize)]
//! # struct Container {
//! #     #[with(Evolve)]
//! #     test: v1::Test,
//! # }
//! use rkyv::ser::serializers::AllocSerializer;
//! use rkyv::ser::Serializer;
//! use rkyv::AlignedVec;
//! 
//! let container = Container {
//!     test: v1::Test {
//!         a: 1,
//!         b: 2,
//!         c: 3,
//!     }
//! };
//! 
//! let mut serializer = AllocSerializer::<256>::default();
//! serializer.serialize_value(&container).unwrap();
//! let buf: AlignedVec = serializer.into_serializer().into_inner();
//! ```
//! 
//! We may then access it as its archived form like so:
//! 
//! ```
//! # mod v1 {
//! #    #[protoss::evolving]
//! #    #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #    pub struct Test {
//! #        #[field(id = 0, since_ev = 0)]
//! #        pub a: u32,
//! #        #[field(id = 1, since_ev = 0)]
//! #        pub b: u8,
//! #        #[field(id = 2, since_ev = 1)]
//! #        pub c: u32,
//! #    }
//! # }
//! # use protoss::Evolve;
//! # #[derive(rkyv::Archive, rkyv::Serialize)]
//! # struct Container {
//! #     #[with(Evolve)]
//! #     test: v1::Test,
//! # }
//! # use rkyv::ser::serializers::AllocSerializer;
//! # use rkyv::ser::Serializer;
//! # use rkyv::AlignedVec;
//! # let container = Container {
//! #     test: v1::Test {
//! #         a: 1,
//! #         b: 2,
//! #         c: 3,
//! #     }
//! # };
//! # let mut serializer = AllocSerializer::<256>::default();
//! # serializer.serialize_value(&container).unwrap();
//! # let buf: AlignedVec = serializer.into_serializer().into_inner();
//! use protoss::Probe;
//! 
//! let archived_container: &ArchivedContainer = unsafe { rkyv::archived_root::<Container>(&buf) };
//! let archived_test: &protoss::ArchivedEvolution<v1::Test> = &archived_container.test;
//! 
//! let probe = archived_test.as_probe();
//! 
//! assert_eq!(probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(1, 2)));
//! assert_eq!(probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(1, 2, 3)));
//! assert_eq!(probe.a(), Some(&1));
//! assert_eq!(probe.b(), Some(&2));
//! assert_eq!(probe.c(), Some(&3));
//! ```
//! 
//! ## Using evolving types across different versions
//! 
//! Let's start with the example of 'backwards compatibility,' meaning that we have a binary which was compiled
//! when `v1` was the current version of the code that is *producing* (i.e. serializing) its latest known version of `Test`,
//! and another binary which was compiled now that `v2` is the current code version, and is *consuming* (i.e. accessing the archived data)
//! the serialized `Test` that the `v1` binary created.
//! 
//! ```
//! # mod v1 {
//! #     #[protoss::evolving]
//! #     #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #     pub struct Test {
//! #         #[field(id = 0, since_ev = 0)]
//! #         pub a: u32,
//! #         #[field(id = 1, since_ev = 0)]
//! #         pub b: u8,
//! #         #[field(id = 2, since_ev = 1)]
//! #         pub c: u32,
//! #     }
//! # }
//! # mod v2 {
//! #     #[protoss::evolving]
//! #     #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #     pub struct Test {
//! #         #[field(id = 1, since_ev = 0)]
//! #         pub b: u8,
//! #         #[field(id = 2, since_ev = 1)]
//! #         pub c: u32,
//! #         #[field(id = 3, since_ev = 2)]
//! #         pub d: u8,
//! #         #[field(id = 0, since_ev = 0)]
//! #         pub a_renamed: u32,
//! #     }
//! # }
//! # use rkyv::ser::serializers::AllocSerializer;
//! # use rkyv::ser::Serializer;
//! # use rkyv::AlignedVec;
//! # use rkyv::{Archive, Serialize};
//! # use protoss::Probe;
//! # use protoss::Evolve;
//! #[derive(Archive, Serialize)]
//! struct ContainerV1 {
//!     #[with(Evolve)]
//!     test: v1::Test,
//! }
//! 
//! #[derive(Archive, Serialize)]
//! struct ContainerV2 {
//!     #[with(Evolve)]
//!     test: v2::Test,
//! }
//! 
//! let container_v1 = ContainerV1 {
//!     test: v1::Test {
//!         a: 1,
//!         b: 2,
//!         c: 3,
//!     }
//! };
//! 
//! // producer is on v1, serializes a v1
//! let mut serializer = AllocSerializer::<256>::default();
//! serializer.serialize_value(&container_v1).unwrap();
//! let buf: AlignedVec = serializer.into_serializer().into_inner();
//! 
//! // consumer is on v2, accesses v1 archive as v2
//! let archived_container: &ArchivedContainerV2 = unsafe { rkyv::archived_root::<ContainerV2>(&buf) };
//! let archived_test: &protoss::ArchivedEvolution<v2::Test> = &archived_container.test;
//! 
//! // v2 probe from v1 archived data
//! let probe: &v2::TestProbe = archived_test.as_probe();
//! 
//! assert_eq!(probe.probe_as::<v2::TestEv0>(), Some(&v2::ArchivedTestEv0::new(1, 2)));
//! assert_eq!(probe.probe_as::<v2::TestEv1>(), Some(&v2::ArchivedTestEv1::new(1, 2, 3)));
//! assert_eq!(probe.probe_as::<v2::TestEv2>(), None);
//! assert_eq!(probe.a_renamed(), Some(&1));
//! assert_eq!(probe.b(), Some(&2));
//! assert_eq!(probe.c(), Some(&3));
//! assert_eq!(probe.d(), None);
//! ```
//! 
//! Next let's see the same thing in reverse ('forwards compatibility') -- simulating a newer binary compiled with `v2` serializing a `Test` which is then accessed
//! by an older binary compiled with `v1`.
//! 
//! ```
//! # mod v1 {
//! #     #[protoss::evolving]
//! #     #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #     pub struct Test {
//! #         #[field(id = 0, since_ev = 0)]
//! #         pub a: u32,
//! #         #[field(id = 1, since_ev = 0)]
//! #         pub b: u8,
//! #         #[field(id = 2, since_ev = 1)]
//! #         pub c: u32,
//! #     }
//! # }
//! # mod v2 {
//! #     #[protoss::evolving]
//! #     #[archived_evolution_attr(derive(Debug, PartialEq))]
//! #     pub struct Test {
//! #         #[field(id = 1, since_ev = 0)]
//! #         pub b: u8,
//! #         #[field(id = 2, since_ev = 1)]
//! #         pub c: u32,
//! #         #[field(id = 3, since_ev = 2)]
//! #         pub d: u8,
//! #         #[field(id = 0, since_ev = 0)]
//! #         pub a_renamed: u32,
//! #     }
//! # }
//! # use rkyv::ser::serializers::AllocSerializer;
//! # use rkyv::ser::Serializer;
//! # use rkyv::AlignedVec;
//! # use rkyv::{Archive, Serialize};
//! # use protoss::Probe;
//! # use protoss::Evolve;
//! #[derive(Archive, Serialize)]
//! struct ContainerV1 {
//!     #[with(Evolve)]
//!     test: v1::Test,
//! }
//! 
//! #[derive(Archive, Serialize)]
//! struct ContainerV2 {
//!     #[with(Evolve)]
//!     test: v2::Test,
//! }
//! 
//! let container_v2 = ContainerV2 {
//!     test: v2::Test {
//!         a_renamed: 5,
//!         b: 6,
//!         c: 7,
//!         d: 8,
//!     }
//! };
//! 
//! // producer is on v2, serializes v2
//! let mut serializer = AllocSerializer::<256>::default();
//! serializer.serialize_value(&container_v2).unwrap();
//! let buf: AlignedVec = serializer.into_serializer().into_inner();
//! 
//! // consumer is on v1, accesses v2-serialized archive as v1
//! let archived_container: &ArchivedContainerV1 = unsafe { rkyv::archived_root::<ContainerV1>(&buf) };
//! let archived_test: &protoss::ArchivedEvolution<v1::Test> = &archived_container.test;
//! 
//! // v1 probe from v2 archived data
//! let probe: &v1::TestProbe = archived_test.as_probe();
//! 
//! assert_eq!(probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(5, 6)));
//! assert_eq!(probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(5, 6, 7)));
//! // compile fails because v1 doesn't know about Ev2!
//! // assert_eq!(probe.probe_as::<v1::TestEv2>(), Some(&v2::ArchivedTestEv2::new(5, 6, 7, 8)));
//! assert_eq!(probe.a(), Some(&5));
//! assert_eq!(probe.b(), Some(&6));
//! assert_eq!(probe.c(), Some(&7));
//! // compile fails because v1 doesn't know about field d on Ev2!
//! // assert_eq!(probe.d(), Some(&8));
//! ```
//! 
//! [schema evolution]: https://martin.kleppmann.com/2012/12/05/schema-evolution-in-avro-protocol-buffers-thrift.html
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod rkyv_impl;
mod test_util;

use core::fmt;

pub use ::ptr_meta::Pointee;
pub use crate::rkyv_impl::ArchivedEvolution;
pub use crate::rkyv_impl::AnyProbe;
pub use crate::rkyv_impl::Evolve;
#[cfg(feature = "derive")]
pub use protoss_derive::evolving;

use ::rkyv::Archive;

/// A common error type for all errors that could occur in `protoss`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// Tried to get [Probe] metadata for a non-existent version of an [`Evolving`] type.
    TriedToGetProbeMetadataForNonExistentVersion,
    /// Tried to build a major version builder with an invalid combination of underlying fields,
    /// which does not match any existing minor version.
    InvalidBuilderFields,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TriedToGetProbeMetadataForNonExistentVersion => {
                write!(f, "tried to get probe metadata for a non-existent version of an Evolving type")
            }
            Self::InvalidBuilderFields => {
                write!(f, "tried to build a major version builder with an invalid combination of underlying fields that did not match any minor version")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// A version identifier containing which "minor" version.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Version(pub u16);

impl Version {
    /// Create a new [`Version`] from a given `minor` version
    pub const fn new(minor: u16) -> Self {
        Self(minor)
    }
}

/// A type that has multiple versions that may be changed over time. 
/// 
/// An [`Evolving`] type may have one or more [`Evolution`]s, which can also be thought of as *minor versions* or
/// *collections of binary-compatible changes* to their schema.
///
/// Each unique [`Evolution`] has a concrete backing type which defines its exact set of fields, and which implements
/// [`Archive`] such that its [`Archived`][Archive::Archived] type follows a specific set of rules that guarantee binary
/// compatibility between such archived [`Evolution`]s.
/// 
/// For example, say we have a type which we want to evolve over time, called `MyType`. Let's say that right now, it has
/// two "evolutions", i.e. two compatible versions of its schema (0 and 1). The core `MyType` should have all the latest fields
/// (from evolution 1) and implement [`Evolving`], and there should be two version structs, `MyTypeV0` and `MyTypeV1`, which each
/// implement [`Evolution`] with [`Base = MyType`][Evolution::Base], as well as
/// [`Archive`] with [`Archived`][Archive::Archived] types `ArchivedMyTypeV0` and `ArchivedMyTypeV1`, respectively. When
/// using the derive macros, these version and archived version structs will be generated for you.
/// 
/// Each [`Evolving`] type also has a concrete "[`Probe`]" type, which is
/// able to "poke at" or "probe" serialized binary data which
/// contains an **unknown** [`Evolution`] of that base [`Evolving`] type.
/// Through "probing", we are able to determine which actual [evolution][`Evolution`] it contains,
/// and therefore access it as the specific [evolution of `Self`][`Evolution`] we have determined it to be,
/// or alternatively attempt to access any known individual field directly.
/// 
/// [`Evolving`] types may be [serialized][::rkyv::Serialize] into an [`ArchivedEvolution`], which may hold *any* [`Evolution`] of that type.
/// From the [`ArchivedEvolution<E>`] you can obtain a reference to the base [`Evolving`] type's [`Probe`], and from that [`Probe`] attempt to
/// access any of its known fields individually, or attempt to access it as a specific archived [`Evolution`] direclty (and therefore get access
/// to all the fields included in that [`Evolution`] at zero cost if it succeeds).
/// 
/// # Safety
///
/// - `probe_metadata` must return valid metadata to construct a [`Probe`][Evolving::Probe] when combined with
/// a pointer to the archived type of an [`Evolution`] where that [`Evolution`]'s [VERSION][Evolution::VERSION] is equal to the
/// passed-in `version` parameter. See the documentation of [`Evolution`] for more.
/// - `LatestEvolution` must be the newest [`Evolution`] of `Self`
/// - `Probe` must be a [`Probe`] type capable of handling all [`Evolution`]s of `Self`
pub unsafe trait Evolving {
    /// The latest [`Evolution`] of `Self`
    type LatestEvolution: Evolution<Base = Self>;

    /// The latest [`Probe`] of `Self`
    type Probe: Probe<Base = Self> + ?Sized;

    /// Returns the [`Pointee::Metadata`] that can be used to construct a [`Probe`]
    /// which contains an [`Evolution`] of `Self` with the given `version`. In practical terms, this means
    /// the function returns the size in bytes of the [`Evolution`]'s [`Archived`][Archive::Archived] type for the specified [`Version`].
    /// This should be the same as the associated const [`Evolution::METADATA`] for that [`Evolution`].
    /// 
    /// For more information on what this means in-depth, see the Safety section in the trait-level documentation
    /// of [`Evolution`].
    fn probe_metadata(version: Version) -> Result<ProbeMetadata, crate::Error>;
}

/// Implemented by a specific concrete "evolution" (minor version) of an [`Evolving`] type.
/// 
/// # Safety
/// 
/// Implementing this trait means that it must be valid to construct (via [`ptr_meta::from_raw_parts`])
/// a [`Probe`] of `Self`'s assotiated [`Evolving`] type ([`Self::Base::Probe`][Evolution::Base])
/// with a data pointer to a [`<Self as Archive>::Archived`][Archive::Archived]
/// and the metadata returned by [`<Self::Base as Evolving>::probe_metadata(Self::VERSION)`][Evolving::probe_metadata]. This implies
/// the following requirements, as well as the ones discussed in the
/// documentation for both [`ptr_meta::Pointee`] and [`ptr_meta::from_raw_parts`]:
/// 
/// - For some [`Evolving`] type `E`, the [`Archived`] type of all [`Evolution`]s of `E` must have exactly the same
/// memory layout as the previous version until the end of the previous version's size. In plain speak
/// this means each version must only add new fields after the end of the previous version, and never change
/// the layout of fields that have already been used in a previous version. This also implies the following:
///     - The [`Archived`] type of all [`Evolution`] within the same **major version** must have size > (not >=) the version that came
/// before it (each version's size must be [*monotonically increasing*])
///     - The [`Archived`] type of all [`Evolution`] within the same **major version** must have an alignment >= the version that came before it
///     - The [`Archived`] type of `Self` must have no padding after its final field, i.e. the end of the memory that the final field
/// occupies must also be the end of the whole struct.
/// 
/// [`Archived`]: Archive::Archived
/// [`ptr_meta` documentation]: ptr_meta::
/// [*monotonically increasing*]: https://mathworld.wolfram.com/MonotoneIncreasing.html
pub unsafe trait Evolution: Archive {
    /// The [`Evolving`] type that this type is an evolution of.
    type Base: Evolving + ?Sized;

    /// The necessary metadata to uniquely identify this evolution using only the information available to a probe for the base type.
    /// For structs, this the size of the archived type of this evolution. For enums, it's the number of variants in that evolution.
    type IdentifierMeta;

    /// The version identifier of this evolution of `Self::Base`
    const VERSION: Version;

    /// TODO: docs
    const ID: Self::IdentifierMeta;

    /// The [`Pointee::Metadata`] that can be used to construct a [`Probe<Base = Self::Base>`]
    /// which contains this verion's archived data ([`<Self as Archive>::Archived`][Archive::Archived]).
    /// In practical terms, this is the size in bytes of [`Self::Archived`][Archive::Archived] for structs,
    /// or the size in bytes minus 8 for enums (since all enums have a u64 discriminant).
    const METADATA: ProbeMetadata;
}

/// All probe types must have this as their [`<Self as Pointee>::Metadata`][Pointee::Metadata]
pub type ProbeMetadata = <[u8] as Pointee>::Metadata;

/// Implemented by a concrete [Probe] type for a specific [`Evolving`] type.
/// 
/// "[`Probe`]" types are able to "poke at" or "[probe][Probe::probe_as]" binary data
/// which contains an **unknown** [`Evolution`] of an [`Evolving`]
/// type in order to determine which actual version it contains and access the contained fields.
/// Probes will often use this ability to also implement helper accessor methods that attempt to access
/// each individual field contained in any (known) minor version of that type.
/// 
/// The key method of [`Probe`] is [`probe_as`][Probe::probe_as], through which richer functionality can
/// then be built.
/// 
/// # Safety
/// 
/// - See [`Evolution`]
/// - TODO: describe more actual requirements here
pub unsafe trait Probe
where
    Self: Pointee<Metadata = ProbeMetadata>,
    Self: 'static,
{
    /// The [`Evolving`] type that this type is a probe of.
    type Base: Evolving + ?Sized;

    /// Returns the [`Version`] specifier of the actual contained version, if known.
    /// 
    /// The actual version may not be known if the contained version was created from a later versioned "producer"
    /// and consumed by an earlier-versioned "consumer" binary which does not have knowledge of the latest version(s).
    /// 
    /// You can think of this as conceptually similar to [`Any::type_id`][std::any::Any::type_id].
    fn version(&self) -> Option<Version>;

    /// "Probes" `self` as the given [`Evolution`].
    /// 
    /// Returns `Some(&EV::Archived)` if `self` is a >= minor version and `None` if `self` is an earlier minor version.
    /// 
    /// You can think of this as conceptually similar to `Any::downcast_ref`.
    fn probe_as<EV: Evolution<Base = Self::Base>>(&self) -> Option<&EV::Archived>;

    /// Assumes `self` is the given [`Evolution`] and casts self as that version.
    /// 
    /// # Safety
    /// 
    /// This probe must have been created with data that is binary-compatible with the given
    /// version: it must be an equal or later minor version of the same [`Evolving`] type (i.e. same 'major' version).
    /// 
    /// You can think of this as conceptually similar to `Any::downcast_ref_unchecked`.
    unsafe fn as_version_unchecked<EV: Evolution<Base = Self::Base>>(&self) -> &EV::Archived;

    /// Cast `&self` into a `&AnyProbe<E>`.
    /// 
    /// This is safe because you can't actually do anything (safely) with a `&AnyProbe<E>` and
    /// the [`Pointee::Metadata`] types are the same and valid between each other.
    fn as_any_probe(&self) -> &AnyProbe<Self::Base> {
        // SAFETY: This is safe because
        // - you can't actually do anything (safely) with a `&AnyProbe<E>` an
        // - the [`Pointee::Metadata`] types are the same and valid between each other
        // - the alignment requirements of Self are always more strict than `AnyProbe`
        unsafe {
            &*::ptr_meta::from_raw_parts(
                (self as *const Self).cast(),
                ptr_meta::metadata(self),
            )
        }
    }

    /// Cast a boxed version of `Self` into a `Box<AnyProbe<E>>`.
    /// 
    /// This is safe because the [`Pointee::Metadata`] for both is the same and
    /// you can't actually do anything (safely) with a `Box<AnyProbe<E>>` besides `Drop` it,
    /// and since it's still a `Box`, it will then deallocate the memory properly so long
    /// as it was allocated properly in the first place.
    fn into_boxed_any_probe(self: Box<Self>) -> Box<AnyProbe<Self::Base>> {
        let ptr = Box::into_raw(self);
        // SAFETY: 
        // This is safe because the [`Pointee::Metadata`] for both is the same and
        // you can't actually do anything (safely) with a `Box<AnyProbe<E>>` besides `Drop` it,
        // and since it's still a `Box`, it will then deallocate the memory properly so long
        // as it was allocated properly in the first place.
        unsafe {
            Box::from_raw(ptr_meta::from_raw_parts_mut(
                ptr.cast(),
                ptr_meta::metadata(ptr)
            ))
        }
    }
}

/// This is a trait that all [`Probe`] types as well as [`AnyProbe`] can implement
/// which provides raw, unsafe helper interfaces.
/// 
/// Implementing this trait is not unsafe in and of itself, but using the
/// [`as_probe_unchecked`][RawProbe::as_probe_unchecked] method is extremely unsafe.
/// 
/// It's unlikely you want to work with this trait directly, but is used internally in the implementation
/// of [`ArchivedEvolution`], which you can then work with safely.
pub trait RawProbe<E>
where
    E: Evolving + ?Sized,
    Self: Pointee<Metadata = ProbeMetadata>,
{
    /// Unsafely "casts" `Self` as a concrete [Probe][Probe] type.
    /// 
    /// # Safety
    /// 
    /// This method is extremely unsafe because it allows you to construct a `P`,
    /// which then has safe interfaces with very particular requirements.
    /// 
    /// In order for this to be valid, `Self` must have originally been a valid `P`,
    /// meaning a `P` backed by a [`Evolution`] of `<P as Probe>::Base`:
    /// 
    /// Specifically, `self` must have been created from properly aligned memory of the correct size, and
    /// [`ptr_meta::metadata(self)`] must give valid [`Pointee::Metadata`] for a `P` created from a data
    /// pointer to `self` and that metadata using [`ptr_meta::from_raw_parts`].
    unsafe fn as_probe_unchecked<P: Probe<Base = E> + ?Sized>(&self) -> &P {
        unsafe {
            &*::ptr_meta::from_raw_parts(
                (self as *const Self).cast(),
                ptr_meta::metadata(self),
            )
        }
    }
}
