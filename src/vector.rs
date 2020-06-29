#[macro_export] macro_rules! vector { ($n:literal $v:ident $($tuple:ident)+, $($c:ident)+) => {
use {$crate::num::Zero, std::ops::{Add,Sub,Mul,Div}};
#[allow(non_camel_case_types)] #[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct $v<T> { $( pub $c: T ),+ }
impl<T:Eq> PartialEq<T> for $v<T> { fn eq(&self, b: &T) -> bool { $( self.$c==*b )&&+ } }
impl<T:Copy> From<T> for $v<T> { fn from(v: T) -> Self { $v{$($c:v),+} } }
impl<T:Copy> From<[T; $n]> for $v<T> { fn from([$($c),+]: [T; $n]) -> Self { $v{$($c),+} } } // FIXME: $n from $c
impl<T:Copy> From<$v<T>> for [T; $n] { fn from(v : $v<T>) -> Self { [$(v.$c),+] } }
impl<T:Copy> From<($($tuple),+)> for $v<T> { fn from(($($c),+): ($($tuple),+)) -> Self { $v{$($c),+} } } // $tuple from $n
impl<T:Copy> From<$v<T>> for ($($tuple),+) { fn from(v : $v<T>) -> Self { ($(v.$c),+) } }

impl<T:Copy+Zero> Zero for $v<T> { fn zero() -> Self { T::zero().into() } }
//pub fn min<T:Ord>(a: $v<T>, b: $v<T>) -> $v<T> { $v{$($c: a.$c .min( b.$c ) ),+} }
//pub fn max<T:Ord>(a: $v<T>, b: $v<T>) -> $v<T> { $v{$($c: a.$c .max( b.$c ) ),+} }
// Panics on unordered values (i.e NaN)
//pub fn min<T:PartialOrd>(a: $v<T>, b: $v<T>) -> $v<T> { $v{$($c: std::cmp::min_by(a.$c, b.$c, |a,b| a.partial_cmp(b).unwrap() ) ),+} }
//pub fn max<T:PartialOrd>(a: $v<T>, b: $v<T>) -> $v<T> { $v{$($c: std::cmp:: max_by(a.$c, b.$c, |a,b| a.partial_cmp(b).unwrap() ) ),+} }
impl<T:Add> Add<$v<T>> for $v<T> { type Output=$v<T::Output>; fn add(self, b: $v<T>) -> Self::Output { Self::Output{$($c: self.$c+b.$c),+} } }
impl<T:Sub> Sub<$v<T>> for $v<T> { type Output=$v<T::Output>; fn sub(self, b: $v<T>) -> Self::Output { Self::Output{$($c: self.$c-b.$c),+} } }
impl<T:Mul> Mul<$v<T>> for $v<T> { type Output=$v<T::Output>; fn mul(self, b: $v<T>) -> Self::Output { Self::Output{$($c: self.$c*b.$c),+} } }
impl<T:Div> Div<$v<T>> for $v<T> { type Output=$v<T::Output>; fn div(self, b: $v<T>) -> Self::Output { Self::Output{$($c: self.$c/b.$c),+} } }
impl<T:Div+Copy> Div<T> for $v<T> { type Output=$v<T::Output>; fn div(self, b: T) -> Self::Output { Self::Output{$($c: self.$c/b),+} } }

fn mul<T:Copy+Mul>(a: T, b: $v<T>) -> $v<T::Output> { $v{$($c: a*b.$c),+} }
fn div<T:Copy+Div>(a: T, b: $v<T>) -> $v<T::Output> { $v{$($c: a/b.$c),+} }

impl Mul<$v<u32>> for u32 { type Output=$v<u32>; fn mul(self, b: $v<u32>) -> Self::Output { mul(self, b) } }
impl Div<$v<u32>> for u32 { type Output=$v<u32>; fn div(self, b: $v<u32>) -> Self::Output { div(self, b) } }
impl Mul<$v<f32>> for f32 { type Output=$v<f32>; fn mul(self, b: $v<f32>) -> Self::Output { mul(self, b) } }
impl Div<$v<f32>> for f32 { type Output=$v<f32>; fn div(self, b: $v<f32>) -> Self::Output { div(self, b) } }
}}

vector!(2 xy T T, x y);
impl<T:Ord> PartialOrd for xy<T> { fn partial_cmp(&self, b: &xy<T>) -> Option<std::cmp::Ordering> { Some(self.cmp(b)) } }
impl<T:Ord> Ord for xy<T> { fn cmp(&self, b: &xy<T>) -> std::cmp::Ordering { // reverse lexicographic (i.e lexicographic yx)
    let ordering = self.y.cmp(&b.y);
    if ordering != std::cmp::Ordering::Equal { ordering } else { self.x.cmp(&b.x) }
} }

impl From<xy<u32>> for xy<f32> { fn from(f: xy<u32>) -> Self { xy{x: f.x as f32, y: f.y as f32} } }
impl From<xy<f32>> for xy<u32> { fn from(f: xy<f32>) -> Self { xy{x: f.x as u32, y: f.y as u32} } }

impl xy<u32> { pub const fn as_f32(self) -> xy<f32> { xy{x: self.x as f32, y: self.y as f32} } }
#[cfg(feature="const_fn")] pub const fn div_f32(a: f32, b: xy<f32>) -> xy<f32> { xy{x: a/b.x, y: a/b.y} }

#[allow(non_camel_case_types)] pub type uint2 = xy<u32>;
#[allow(non_camel_case_types)] pub type int2 = xy<i32>;
#[allow(non_camel_case_types)] pub type size2 = xy<u32>;
#[allow(non_camel_case_types)] pub type vec2 = xy<f32>;

pub fn lerp(t: f32, a: vec2, b: vec2) -> xy<f32> { (1.-t)*a + t*b }
pub fn dot(a: vec2, b: vec2) -> f32 { a.x*b.x + a.y*b.y }
pub fn cross(a: vec2, b: vec2) -> f32 { a.x*b.y - a.y*b.x }
pub fn sq(x:vec2) -> f32 { dot(x, x) }
pub fn norm(v:vec2) -> f32 { crate::num::sqrt(sq(v)) }
pub fn atan(v:vec2) -> f32 { crate::num::atan(v.y,v.x) }
