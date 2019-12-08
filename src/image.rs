use crate::ensure;
use crate::core::Result;
#[allow(non_camel_case_types)] #[derive(Clone, Copy, Debug, parse_display::Display)] #[display("{x}, {y}")] pub struct uint2 { pub x: u32, pub y : u32 }
impl From<(u32,u32)> for uint2 { fn from(v : (u32, u32)) -> Self { Self{x:v.0,y:v.1} } }
#[allow(non_camel_case_types)] pub type size2 = uint2;
#[allow(non_camel_case_types)] pub type offset2 = uint2;

pub struct Image<T> {
    pub stride : u32,
    pub size : size2,
    pub buffer : T,
}
pub trait IntoImage {
    type Image;
    fn image(self, size : size2) -> Option<Self::Image>;
}
impl<'t, T> IntoImage for &'t [T] {
    type Image = Image<&'t [T]>;
    fn image(self, size : size2) -> Option<Self::Image> {
        if self.len() == (size.x*size.y) as usize { Some(Self::Image{stride: size.x, size, buffer: self}) } else { None }
    }
}
impl<'t, T> IntoImage for &'t mut [T] {
    type Image = Image<&'t mut [T]>;
    fn image(self, size : size2) -> Option<Self::Image> {
        if self.len() == (size.x*size.y) as usize { Some(Self::Image{stride: size.x, size, buffer: self}) } else { None }
    }
}

impl<T> Image<&mut [T]> {
    pub fn slice_mut(&mut self, offset : offset2, size : size2) -> Result<Image<&mut[T]>> {
        ensure!(offset.x+size.x <= self.size.x && offset.y+size.y <= self.size.y, (self.size, offset, size));
        Ok(Image{size, stride: self.stride, buffer: &mut self.buffer[(offset.y*self.stride+offset.x) as usize..] })
    }
}

pub trait Offset { fn offset(&self, offset : isize) -> Self; }
impl<T> Offset for *const T { fn offset(&self, offset : isize) -> Self { unsafe{(*self).offset(offset)} } }
impl<T> Offset for *mut T { fn offset(&self, offset : isize) -> Self { unsafe{(*self).offset(offset)} } }

pub struct Rows<T> { pub ptr: T, stride: isize }
impl<T:Offset> Rows<T> { pub fn next(&mut self) { self.ptr = self.ptr.offset(self.stride); } }
impl<T> std::ops::Index<u32> for Rows<*const T> { type Output = T; fn index(&self, x: u32) -> &T { unsafe{&*self.ptr.offset(x as isize)} } }
impl<T> std::ops::Index<u32> for Rows<*mut T> { type Output = T; fn index(&self, x: u32) -> &T { unsafe{&*self.ptr.offset(x as isize)} } }
impl<T> std::ops::IndexMut<u32> for Rows<*mut T> { fn index_mut(&mut self, x: u32) -> &mut T { unsafe{&mut *self.ptr.offset(x as isize)} } }

pub trait IntoRows {
    type ConstPtr : Eq+Copy;
    fn end(&self) -> Self::ConstPtr;

    type Ptr : Offset+Copy;
    fn rows(&mut self) -> Rows<Self::Ptr>;
    fn eq(ptr : Self::Ptr, end : Self::ConstPtr) -> bool;

    type Element : Sized;
    fn index(ptr : Self::Ptr, x : usize) -> Self::Element;
}

impl<'t, T> IntoRows for Image<&'t [T]> {
    type ConstPtr = *const T;
    fn end(&self) -> Self::ConstPtr { unsafe{(self.buffer as *const [T] as Self::ConstPtr).offset((self.size.y*self.stride) as isize)} }

    type Ptr = *const T;
    fn rows(&mut self) -> Rows<Self::Ptr> { Rows::<Self::Ptr>{ptr: self.buffer as *const [T] as Self::Ptr, stride: self.stride as isize} }
    fn eq(ptr : Self::Ptr, end : Self::ConstPtr) -> bool { ptr == end }

    type Element = &'t T;
    fn index(ptr : Self::Ptr, x : usize) -> Self::Element { unsafe{&*ptr.offset(x as isize)} }
}

impl<'t, T> IntoRows for Image<&'t mut [T]> {
    type ConstPtr = *const T;
    fn end(&self) -> Self::ConstPtr { unsafe{(self.buffer as *const [T] as Self::ConstPtr).offset((self.size.y*self.stride) as isize)} }

    type Ptr = *mut T;
    fn rows(&mut self) -> Rows<Self::Ptr> { Rows::<Self::Ptr>{ptr: self.buffer as *mut [T] as Self::Ptr, stride: self.stride as isize} }
    fn eq(ptr : Self::Ptr, end : Self::ConstPtr) -> bool { ptr as *const T == end }

    type Element = &'t mut T;
    fn index(ptr : Self::Ptr, x : usize) -> Self::Element { unsafe{&mut *ptr.offset(x as isize)} }
}

pub struct PixelIterator1<T> where Image<T> : IntoRows {
    width : usize,
    end: <Image<T> as IntoRows>::ConstPtr,
    rows: Rows<<Image<T> as IntoRows>::Ptr>,
    x : usize,
}
impl<T> Iterator for PixelIterator1<T> where Image<T> : IntoRows, <Image<T> as IntoRows>::Ptr : Eq {
    type Item = <Image<T> as IntoRows>::Element;
    #[inline] // test mov add add test jcc (SIB) inc ~ 7
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.width {
            self.x = 0;
            self.rows.next();
            if Image::<T>::eq(self.rows.ptr, self.end) { None? }
        }
        let item = Some(Image::<T>::index(self.rows.ptr, self.x));
        self.x += 1;
        item
    }
}

pub struct PixelIterator2<T0, T1> where Image<T0> : IntoRows, Image<T1> : IntoRows {
    width : usize,
    end: <Image<T0> as IntoRows>::ConstPtr,
    rows: (Rows<<Image<T0> as IntoRows>::Ptr>, Rows<<Image<T1> as IntoRows>::Ptr>), // (Rows<T::Ptr>...)
    x : usize,
}
impl<T0, T1> Iterator for PixelIterator2<T0, T1> where Image<T0> : IntoRows, Image<T1> : IntoRows, <Image<T0> as IntoRows>::Ptr : Eq {
    type Item = (<Image<T0> as IntoRows>::Element, <Image<T1> as IntoRows>::Element);
    #[inline] // test mov add add test jcc (SIB) inc ~ 7
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.width {
            self.x = 0;
            self.rows.0.next(); self.rows.1.next(); // next(self.rows)...
            if Image::<T0>::eq(self.rows.0.ptr, self.end) { None? }
        }
        let item = Some((Image::<T0>::index(self.rows.0.ptr, self.x), Image::<T1>::index(self.rows.1.ptr, self.x))); // self.x.index(self.rows)...
        self.x += 1;
        item
    }
}

pub trait IntoPixelIterator { type PixelIterator; fn pixels(&mut self) -> Self::PixelIterator; }
impl<T> IntoPixelIterator for Image<T> where Image<T> : IntoRows {
    type PixelIterator = PixelIterator1<T>;
    fn pixels(&mut self) -> Self::PixelIterator {
        Self::PixelIterator{
            width : self.size.x as usize,
            end: self.end(),
            rows: self.rows(),
            x: 0
        }
    }
}
impl<T0, T1> IntoPixelIterator for (Image<T0>, Image<T1>) where Image<T0> : IntoRows, Image<T1> : IntoRows {
    type PixelIterator = PixelIterator2<T0, T1>;
    fn pixels(&mut self) -> Self::PixelIterator {
        Self::PixelIterator{
            width : self.0.size.x as usize,
            end: self.0.end(),
            rows: (self.0.rows(), self.1.rows()), // (self.rows()...)
            x: 0
        }
    }
}

#[allow(non_camel_case_types, dead_code)] #[derive(Clone, Copy)] pub struct bgra8 { pub b : u8, pub g : u8, pub r : u8, pub a: u8  }

impl<T : Default+Clone> Image<Vec<T>> {
    pub fn new(size: size2) -> Self { Self{stride:size.x, size, buffer: vec![T::default(); (size.x*size.y) as usize]} }
    pub fn uninitialized(size: size2) -> Self {
        let len = (size.x * size.y) as usize;
        let mut buffer = Vec::with_capacity(len);
        unsafe{ buffer.set_len(len) };
        Self{stride:size.x, size, buffer}
    }
    pub fn as_ref(&self) -> Image<&[T]> { Image{stride:self.stride, size:self.size, buffer: self.buffer.as_ref()} }
    pub fn as_mut(&mut self) -> Image<&mut [T]> { Image{stride:self.stride, size:self.size, buffer: self.buffer.as_mut()} }
}
