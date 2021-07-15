#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use protoss::protoss;
struct TestVersion0 {
    a: i32,
    b: i32,
    _phantom: ::core::marker::PhantomData<Test>,
}
impl TestVersion0 {
    pub fn new(a: i32, b: i32) -> Self {
        Self {
            a,
            b,
            _phantom: ::core::marker::PhantomData,
        }
    }
}
struct TestVersion1 {
    c: u32,
    d: u8,
    _phantom: ::core::marker::PhantomData<Test>,
}
impl TestVersion1 {
    pub fn new(c: u32, d: u8) -> Self {
        Self {
            c,
            d,
            _phantom: ::core::marker::PhantomData,
        }
    }
}
#[repr(C)]
struct Test {
    version_0: TestVersion0,
    version_1: TestVersion1,
}
impl Test {
    #[inline]
    pub fn partial_v0(a: i32, b: i32) -> ::protoss::Partial<Self> {
        unsafe {
            let mut result = ::core::mem::MaybeUninit::<Self>::uninit();
            let result_ptr = result.as_mut_ptr();
            let version_ptr = &raw mut (*result_ptr).version_0;
            version_ptr.write(TestVersion0::new(a, b));
            let size = version_ptr
                .cast::<u8>()
                .offset_from(result_ptr.cast::<u8>()) as usize
                + ::core::mem::size_of::<TestVersion0>();
            ::protoss::Partial::new_unchecked(result, size)
        }
    }
    #[inline]
    pub fn partial_v1(a: i32, b: i32, c: u32, d: u8) -> ::protoss::Partial<Self> {
        unsafe {
            let mut result = ::core::mem::MaybeUninit::<Self>::uninit();
            let result_ptr = result.as_mut_ptr();
            let version_ptr = &raw mut (*result_ptr).version_0;
            version_ptr.write(TestVersion0::new(a, b));
            let version_ptr = &raw mut (*result_ptr).version_1;
            version_ptr.write(TestVersion1::new(c, d));
            let size = version_ptr
                .cast::<u8>()
                .offset_from(result_ptr.cast::<u8>()) as usize
                + ::core::mem::size_of::<TestVersion1>();
            ::protoss::Partial::new_unchecked(result, size)
        }
    }
}
unsafe impl ::protoss::Composite for Test {
    type Parts = TestParts;
}
#[repr(transparent)]
struct TestParts {
    _phantom: ::core::marker::PhantomData<Test>,
    bytes: [u8],
}
const _: () = {
    use ptr_meta::Pointee;
    impl Pointee for TestParts
    where
        [u8]: Pointee,
    {
        type Metadata = <[u8] as Pointee>::Metadata;
    }
};
impl Drop for TestParts {
    fn drop(&mut self) {
        unsafe {
            if let Some(version) = self.__version_0_mut() {
                ::core::ptr::drop_in_place(version as *mut TestVersion0);
            } else {
                return;
            }
            if let Some(version) = self.__version_1_mut() {
                ::core::ptr::drop_in_place(version as *mut TestVersion1);
            } else {
                return;
            }
        }
    }
}
impl TestParts {
    fn __version_0(&self) -> Option<&TestVersion0> {
        unsafe {
            let struct_ptr = (self as *const Self).cast::<Test>();
            let field_ptr = &raw const (*struct_ptr).version_0;
            let offset = field_ptr.cast::<u8>().offset_from(struct_ptr.cast::<u8>()) as usize;
            let size = ::core::mem::size_of::<TestVersion0>();
            if offset + size > self.bytes.len() {
                None
            } else {
                Some(&*field_ptr)
            }
        }
    }
    fn __version_0_mut(&mut self) -> Option<&mut TestVersion0> {
        unsafe {
            let struct_ptr = (self as *mut Self).cast::<Test>();
            let field_ptr = &raw mut (*struct_ptr).version_0;
            let offset = field_ptr.cast::<u8>().offset_from(struct_ptr.cast::<u8>()) as usize;
            let size = ::core::mem::size_of::<TestVersion0>();
            if offset + size > self.bytes.len() {
                None
            } else {
                Some(&mut *field_ptr)
            }
        }
    }
    fn __version_1(&self) -> Option<&TestVersion1> {
        unsafe {
            let struct_ptr = (self as *const Self).cast::<Test>();
            let field_ptr = &raw const (*struct_ptr).version_1;
            let offset = field_ptr.cast::<u8>().offset_from(struct_ptr.cast::<u8>()) as usize;
            let size = ::core::mem::size_of::<TestVersion1>();
            if offset + size > self.bytes.len() {
                None
            } else {
                Some(&*field_ptr)
            }
        }
    }
    fn __version_1_mut(&mut self) -> Option<&mut TestVersion1> {
        unsafe {
            let struct_ptr = (self as *mut Self).cast::<Test>();
            let field_ptr = &raw mut (*struct_ptr).version_1;
            let offset = field_ptr.cast::<u8>().offset_from(struct_ptr.cast::<u8>()) as usize;
            let size = ::core::mem::size_of::<TestVersion1>();
            if offset + size > self.bytes.len() {
                None
            } else {
                Some(&mut *field_ptr)
            }
        }
    }
    fn a(&self) -> Option<&i32> {
        self.__version_0().map(|version| &version.a)
    }
    fn a_mut(&mut self) -> Option<&mut i32> {
        self.__version_0_mut().map(|version| &mut version.a)
    }
    fn b(&self) -> Option<&i32> {
        self.__version_0().map(|version| &version.b)
    }
    fn b_mut(&mut self) -> Option<&mut i32> {
        self.__version_0_mut().map(|version| &mut version.b)
    }
    fn c(&self) -> Option<&u32> {
        self.__version_1().map(|version| &version.c)
    }
    fn c_mut(&mut self) -> Option<&mut u32> {
        self.__version_1_mut().map(|version| &mut version.c)
    }
    fn d(&self) -> Option<&u8> {
        self.__version_1().map(|version| &version.d)
    }
    fn d_mut(&mut self) -> Option<&mut u8> {
        self.__version_1_mut().map(|version| &mut version.d)
    }
}
