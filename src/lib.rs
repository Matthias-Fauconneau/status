#![cfg_attr(feature="array", allow(incomplete_features),feature(const_generics,maybe_uninit_extra,maybe_uninit_uninit_array))]
#![cfg_attr(feature="timeout", feature(thread_spawn_unchecked))]
#![cfg_attr(any(feature="array",feature="image",all(feature="num",feature="font")), feature(track_caller))]
//#![cfg_attr(feature="vector", feature(cmp_min_max_by))]
#![cfg_attr(feature="window", feature(async_closure))]
#![cfg_attr(feature="font", feature(type_alias_impl_trait))]
#![cfg_attr(feature="graphic", feature(iterator_fold_self))]
//#![cfg_attr(feature="image", feature(slice_index_methods))]

pub mod error; pub use error::{Error, Result/*bail, ensure, Ok*/}; #[cfg(feature="fehler")] pub use error::throws;
pub use cfg_if::cfg_if;
mod iter;
mod slice;
#[cfg(feature="array")] pub mod array; //pub use array::{Iterator, map};
cfg_if! { if #[cfg(feature="num")] { pub mod num; pub use num::{Zero, Ratio}; } }
cfg_if! { if #[cfg(feature="vector")] { #[macro_use] pub mod vector; pub use vector::{xy, int2, uint2, size2, vec2}; }}
cfg_if! { if #[cfg(feature="trace")] { mod trace; pub use trace::{rstack_self, trace}; #[cfg(feature="timeout")] pub use trace::timeout; }}
cfg_if! { if #[cfg(feature="image")] { pub mod image; pub use image::{Image, bgra8}; }}
cfg_if! { if #[cfg(feature="sRGB")] { pub use image::sRGB; }}
#[cfg(feature="color")] pub mod color;
cfg_if! { if #[cfg(feature="widget")] { pub mod widget; pub use widget::{Target, Widget, bg, fg}; }}
cfg_if! { if #[cfg(feature="window")] { pub mod window; }}
cfg_if! { if #[cfg(feature="font")] {
	mod quad;
	mod cubic;
	mod raster;
	pub mod font; pub use font::Font;
}}
cfg_if! { if #[cfg(feature="text")] { pub mod text; pub use text::{Text, default_style}; }}
cfg_if! { if #[cfg(feature="graphic")] { pub mod graphic; pub use graphic::{Rect, Glyph, Graphic, GraphicView}; }}
