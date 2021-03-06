//! This module contains a bunch of traits necessary for processing byte strings.
//!
//! Most notable are:
//! * `Source` - implemented by default for `&str` and `&[u8]`, used by the `Lexer`.
//! * `Slice` - slices of `Source`, returned by `Lexer::slice`.

use std::fmt::Debug;
use std::ops::Range;

/// Trait for a `Slice` of a `Source` that the `Lexer` can consume.
///
/// Most commonly, those will be the same types:
/// * `&str` slice for `&str` source.
/// * `&[u8]` slice for `&[u8]` source.
pub trait Slice<'source>: Sized + PartialEq + Eq + Debug {
    /// In all implementations we should at least be able to obtain a
    /// slice of bytes as the lowest level common denominator.
    fn as_bytes(&self) -> &'source [u8];
}

impl<'source> Slice<'source> for &'source str {
    fn as_bytes(&self) -> &'source [u8] {
        (*self).as_bytes()
    }
}

impl<'source> Slice<'source> for &'source [u8] {
    fn as_bytes(&self) -> &'source [u8] {
        *self
    }
}

/// Trait for types the `Lexer` can read from.
///
/// Most notably this is implemented for `&str`. It is unlikely you will
/// ever want to use this Trait yourself, unless implementing a new `Source`
/// the `Lexer` can use.
pub trait Source<'source> {
    /// A type this `Source` can be sliced into.
    type Slice: self::Slice<'source>;

    /// Length of the source
    fn len(&self) -> usize;

    /// Read a chunk of bytes into an array. Returns `None` when reading
    /// out of bounds would occur.
    ///
    /// This is very useful for matching fixed-size byte arrays, and tends
    /// to be very fast at it too, since the compiler knows the byte lengths.
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// fn main() {
    ///     let foo = "foo";
    ///
    ///     assert_eq!(foo.read(0), Some(b"foo"));     // Option<&[u8; 3]>
    ///     assert_eq!(foo.read(0), Some(b"fo"));      // Option<&[u8; 2]>
    ///     assert_eq!(foo.read(2), Some(b'o'));       // Option<u8>
    ///     assert_eq!(foo.read::<&[u8; 4]>(0), None); // Out of bounds
    ///     assert_eq!(foo.read::<&[u8; 2]>(2), None); // Out of bounds
    /// }
    /// ```
    fn read<Chunk>(&self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'source>;

    /// Get a slice of the source at given range. This is analogous to
    /// `slice::get(range)`.
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// fn main() {
    ///     let foo = "It was the year when they finally immanentized the Eschaton.";
    ///
    ///     assert_eq!(Source::slice(&foo, 51..59), Some("Eschaton"));
    /// }
    /// ```
    fn slice(&self, range: Range<usize>) -> Option<Self::Slice>;

    /// Get a slice of the source at given range. This is analogous to
    /// `slice::get_unchecked(range)`.
    ///
    /// **Using this method with range out of bounds is undefined behavior!**
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// fn main() {
    ///     let foo = "It was the year when they finally immanentized the Eschaton.";
    ///
    ///     unsafe {
    ///         assert_eq!(Source::slice_unchecked(&foo, 51..59), "Eschaton");
    ///     }
    /// }
    /// ```
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> Self::Slice;

    /// For `&str` sources attempts to find the closest `char` boundary at which source
    /// can be sliced, starting from `index`.
    ///
    /// For binary sources (`&[u8]`) this should just return `index` back.
    fn find_boundary(&self, index: usize) -> usize {
        index
    }
}

/// Marker trait for any `Source` that can be sliced into arbitrary byte chunks,
/// with no regard for UTF-8 (or any other) character encoding.
pub trait BinarySource<'source>: Source<'source> {}

/// Marker trait for any `Logos`, which will constrain it to a specific subset of
/// `Source`s.
///
/// In particular, if your token definitions would allow reading invalid UTF-8,
/// the `Logos` derive macro will restrict you to lexing on `Source`s that also
/// implement the `BinarySource` marker (`&[u8]` is provided).
///
/// **Note:** You shouldn't implement this trait yourself, `#[derive(Logos)]` will
/// do it for you.
pub trait WithSource<Source> {}

impl<'source> Source<'source> for &'source str {
    type Slice = &'source str;

    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn read<Chunk>(&self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'source>,
    {
        if offset + (Chunk::SIZE - 1) < (*self).len() {
            Some(unsafe { Chunk::from_ptr((*self).as_ptr().add(offset)) })
        } else {
            None
        }
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Option<&'source str> {
        self.get(range)
    }

    #[inline]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &'source str {
        debug_assert!(
            range.start <= self.len() && range.end <= self.len(),
            "Reading out of bounds {:?} for {}!",
            range,
            self.len()
        );

        self.get_unchecked(range)
    }

    #[inline]
    fn find_boundary(&self, mut index: usize) -> usize {
        while !self.is_char_boundary(index) {
            index += 1;
        }

        index
    }
}

impl<'source> Source<'source> for &'source [u8] {
    type Slice = &'source [u8];

    #[inline]
    fn len(&self) -> usize {
        (*self).len()
    }

    #[inline]
    fn read<Chunk>(&self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'source>,
    {
        if offset + (Chunk::SIZE - 1) < (*self).len() {
            Some(unsafe { Chunk::from_ptr((*self).as_ptr().add(offset)) })
        } else {
            None
        }
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Option<&'source [u8]> {
        self.get(range)
    }

    #[inline]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &'source [u8] {
        debug_assert!(
            range.start <= self.len() && range.end <= self.len(),
            "Reading out of bounds {:?} for {}!",
            range,
            self.len()
        );

        self.get_unchecked(range)
    }
}

impl<'source> BinarySource<'source> for &'source [u8] {}

/// A fixed, statically sized chunk of data that can be read from the `Source`.
///
/// This is implemented for `u8`, as well as byte arrays `&[u8; 1]` to `&[u8; 16]`.
pub trait Chunk<'source>: Sized + Copy + PartialEq + Eq {
    /// Size of the chunk being accessed in bytes.
    const SIZE: usize;

    /// Create a chunk from a raw byte pointer.
    unsafe fn from_ptr(ptr: *const u8) -> Self;
}

impl<'source> Chunk<'source> for u8 {
    const SIZE: usize = 1;

    #[inline]
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        *ptr
    }
}

macro_rules! impl_array {
    ($($size:expr),*) => ($(
        impl<'source> Chunk<'source> for &'source [u8; $size] {
            const SIZE: usize = $size;

            #[inline]
            unsafe fn from_ptr(ptr: *const u8) -> Self {
                &*(ptr as *const [u8; $size])
            }
        }
    )*);
}

impl_array!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);