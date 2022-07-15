#![allow(incomplete_features)]
#![feature(ptr_const_cast)]
#![feature(const_mut_refs)]
#![feature(unchecked_math)]
#![feature(const_inherent_unchecked_arith)]
#![feature(adt_const_params)]
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Add, AddAssign, Deref, DerefMut, Range};
pub trait BitIndex<T, const I: usize> {
    fn bit(&self) -> &Bit<T, I>;
}
pub trait BitIndexMut<T, const I: usize> {
    fn bit_mut(&mut self) -> &mut Bit<T, I>;
}
use testing_bin::bitfield;
bitfield!(
    GeneratedBitField,
    u32,
    [
        SSE,
        1,
        SSE1,
        3,
        RANGE1,
        4..6,
        SSE2,
        9,
        SSE3,
        10,
        RANGE2,
        12..15,
        SSE4,
        18
    ]
);

// asdasd
// asdasd
// asd
// asdsa
// asdasd
//
#[derive(Debug, Clone, Copy)]
pub struct BitRange<T, const R: Range<usize>>(std::marker::PhantomData<T>);
impl<const R: Range<usize>> fmt::Display for BitRange<u64, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u64 = self.clone().into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u32, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u32 = self.clone().into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u16, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u16 = self.clone().into();
        write!(f, "{}", a)
    }
}
impl<const R: Range<usize>> fmt::Display for BitRange<u8, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: u8 = self.clone().into();
        write!(f, "{}", a)
    }
}

impl<const R: Range<usize>> BitRange<u64, R> {
    const MASK: u64 =
        unsafe { u64::MAX.unchecked_shl(R.start as u64) & u64::MAX.unchecked_shr(R.start as u64) };
    const fn data(&self) -> *const u64 {
        let a = self as *const Self;
        a as *const u64
    }

    const fn data_mut(&mut self) -> *mut u64 {
        let a = self as *const Self;
        let b = a as *const u64;
        b.as_mut()
    }
}
impl<const R: Range<usize>> BitRange<u32, R> {
    const MASK: u32 =
        unsafe { u32::MAX.unchecked_shl(R.start as u32) & u32::MAX.unchecked_shr(R.start as u32) };

    const fn data(&self) -> *const u32 {
        let a = self as *const Self;
        a as *const u32
    }

    const fn data_mut(&mut self) -> *mut u32 {
        let a = self as *const Self;
        let b = a as *const u32;
        b.as_mut()
    }
}
impl<const R: Range<usize>> BitRange<u16, R> {
    const MASK: u16 =
        unsafe { u16::MAX.unchecked_shl(R.start as u16) & u16::MAX.unchecked_shr(R.start as u16) };
    const fn data(&self) -> *const u16 {
        let a = self as *const Self;
        a as *const u16
    }

    const fn data_mut(&mut self) -> *mut u16 {
        let a = self as *const Self;
        let b = a as *const u16;
        b.as_mut()
    }
}
impl<const R: Range<usize>> BitRange<u8, R> {
    const MASK: u8 =
        unsafe { u8::MAX.unchecked_shl(R.start as u8) & u8::MAX.unchecked_shr(R.start as u8) };
    const fn data(&self) -> *const u8 {
        let a = self as *const Self;
        a as *const u8
    }

    const fn data_mut(&mut self) -> *mut u8 {
        let a = self as *const Self;
        let b = a as *const u8;
        b.as_mut()
    }
}
impl<const R: Range<usize>> AddAssign<u32> for BitRange<u32, R> {
    fn add_assign(&mut self, x: u32) {
        let a = Self::MASK & (x << R.start);
        unsafe { *self.data_mut() += a }
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u64> for BitRange<u64, R> {
    fn into(self) -> u64 {
        let a = Self::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u32> for BitRange<u32, R> {
    fn into(self) -> u32 {
        let a = Self::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u16> for BitRange<u16, R> {
    fn into(self) -> u16 {
        let a = Self::MASK & unsafe { *self.data() };
        a >> R.start
    }
}
#[allow(clippy::from_over_into)]
impl<const R: Range<usize>> Into<u8> for BitRange<u8, R> {
    fn into(self) -> u8 {
        let a = Self::MASK & unsafe { *self.data() };
        a >> R.start
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bit<T, const P: usize>(std::marker::PhantomData<T>);
impl<const P: usize> fmt::Display for Bit<u64, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u32, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u16, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> fmt::Display for Bit<u8, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a: bool = self.into();
        write!(f, "{}", a)
    }
}
impl<const P: usize> Bit<u64, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u64 {
        let a = self as *const Self;
        a as *const u64
    }

    const fn data_mut(&mut self) -> *mut u64 {
        let a = self as *const Self;
        let b = a as *const u64;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u32, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u32 {
        let a = self as *const Self;
        a as *const u32
    }

    const fn data_mut(&mut self) -> *mut u32 {
        let a = self as *const Self;
        let b = a as *const u32;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u16, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u16 {
        let a = self as *const Self;
        a as *const u16
    }

    const fn data_mut(&mut self) -> *mut u16 {
        let a = self as *const Self;
        let b = a as *const u16;
        b.as_mut()
    }
}
impl<const P: usize> Bit<u8, P> {
    pub const fn on(&mut self) {
        unsafe { *self.data_mut() |= 1 << P };
    }

    pub const fn off(&mut self) {
        unsafe { *self.data_mut() &= !(1 << P) };
    }

    pub const fn flip(&mut self) {
        unsafe { *self.data_mut() ^= 1 << P };
    }

    const fn data(&self) -> *const u8 {
        let a = self as *const Self;
        a as *const u8
    }

    const fn data_mut(&mut self) -> *mut u8 {
        let a = self as *const Self;
        let b = a as *const u8;
        b.as_mut()
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u64, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u32, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u16, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<bool> for &Bit<u8, P> {
    fn into(self) -> bool {
        unsafe { (*self.data() >> P) & 1 == 1 }
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u64, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u32, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u16, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
#[allow(clippy::from_over_into)]
impl<const P: usize> Into<u8> for &Bit<u8, P> {
    fn into(self) -> u8 {
        let a: bool = self.into();
        a as u8
    }
}
impl<const P: usize> PartialEq for Bit<u64, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u32, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u16, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq for Bit<u8, P> {
    fn eq(&self, other: &Self) -> bool {
        let a: bool = self.into();
        let b: bool = other.into();
        a == b
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u64, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u32, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u16, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> PartialEq<bool> for Bit<u8, P> {
    fn eq(&self, other: &bool) -> bool {
        let a: bool = self.into();
        a == *other
    }
}
impl<const P: usize> Eq for Bit<u64, P> {}
impl<const P: usize> Eq for Bit<u32, P> {}
impl<const P: usize> Eq for Bit<u16, P> {}
impl<const P: usize> Eq for Bit<u8, P> {}

struct MyBitField{
    pub data: u8,
    pub bits: (MyBit<u8,0>,MyBit<u8,1>,MyBit<u8,2>,MyBit<u8,3>,MyBit<u8,4>,MyBit<u8,5>,MyBit<u8,6>,MyBit<u8,7>),
    pub ranges: (MyBitRange<u8,{1..4}>,MyBitRange<u8,{5..7}>)
}
struct MyBit<T,const P:usize>(PhantomData<T>);
struct MyBitRange<T,const R: Range<usize>>(PhantomData<T>);
impl MyBitField {
    fn ptr(&self) -> *const Self {
        self as *const Self
    }
}
impl<T, const P:usize> MyBit<T,P> {
    fn ptr(&self) -> *const T {
        let a = self as *const Self;
        a as *const T
    }
}
impl<T, const R: Range<usize>> MyBitRange<T,R> {
    fn ptr(&self) -> *const T{
        let a  = self as *const Self;
        a as *const T
    }
}

fn main() {
    println!("started");
    let mut bitfield = GeneratedBitField::from(7);
    println!("bitfield: {:08b} | {:?} | {}", bitfield, bitfield, bitfield);
    println!(
        "size_of::<GeneratedBitField>(): {}",
        size_of::<GeneratedBitField>()
    );
    println!("size_of::<Bit<u16,10>>(): {}", size_of::<Bit<u16, 10>>());
    println!("bitfield ptr: {:?}", &bitfield as *const GeneratedBitField);

    // let ptr_test = PointerTesting { a: 2u32, b:
    // (InnerPointerTesting(PhantomData),InnerPointerTesting(PhantomData),
    // InnerPointerTesting(PhantomData)),c:(InnerRangeTesting(PhantomData),
    let ptr_test = MyBitField {
        data: 15,
        bits: (
            MyBit(PhantomData),
            MyBit(PhantomData),
            MyBit(PhantomData),
            MyBit(PhantomData),
            MyBit(PhantomData),
            MyBit(PhantomData),
            MyBit(PhantomData),
            MyBit(PhantomData),
        ),
        ranges: (
            MyBitRange(PhantomData),
            MyBitRange(PhantomData),
        ),
    };
    println!(
        "ptrs: {:?} | {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} | {:?} {:?}",
        ptr_test.ptr(),
        ptr_test.bits.0.ptr(),
        ptr_test.bits.1.ptr(),
        ptr_test.bits.2.ptr(),
        ptr_test.bits.3.ptr(),
        ptr_test.bits.4.ptr(),
        ptr_test.bits.5.ptr(),
        ptr_test.bits.6.ptr(),
        ptr_test.bits.7.ptr(),
        ptr_test.ranges.0.ptr(),
        ptr_test.ranges.1.ptr()
    );

    *bitfield.RANGE1_mut() += 2;
    println!("bitfield: {:08b} | {:?} | {}", bitfield, bitfield, bitfield);
}
