//! Typed wrappers used by the generated protocol structs.
//!
//! - [`StrBytes`]: a zero-copy, UTF-8-guaranteed string backed by [`Bytes`].
//!   Every `string` field in the Kafka schemas decodes to this (validated once
//!   at decode time), so downstream code gets `&str` access without re-checking.
//! - Entity newtypes ([`TopicName`], [`GroupId`], [`TransactionalId`],
//!   [`BrokerId`], [`ProducerId`]): generated from the schemas' `entityType`
//!   annotations, so e.g. a topic name and a group id are not interchangeable.

use bytes::Bytes;
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::str::Utf8Error;

/// An immutable UTF-8 string backed by [`Bytes`].
///
/// Like `String` it guarantees valid UTF-8, like `Bytes` it is cheaply
/// cloneable and can be a zero-copy slice of a network frame. Decoding
/// validates UTF-8 exactly once; from then on [`StrBytes::as_str`] is free.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrBytes(Bytes);

/// Hashes as `str`, not as a byte slice — required by the `Borrow<str>` impl
/// (`HashMap<StrBytes, _>` must find entries via `&str` keys).
impl std::hash::Hash for StrBytes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl StrBytes {
    /// The empty string.
    pub const fn new() -> Self {
        StrBytes(Bytes::new())
    }

    /// Wrap a static string, zero-copy.
    pub const fn from_static(s: &'static str) -> Self {
        StrBytes(Bytes::from_static(s.as_bytes()))
    }

    /// Validate `bytes` as UTF-8 and wrap it, zero-copy.
    pub fn from_utf8(bytes: Bytes) -> Result<Self, Utf8Error> {
        std::str::from_utf8(&bytes)?;
        Ok(StrBytes(bytes))
    }

    /// Wrap `bytes` without validating UTF-8.
    ///
    /// # Safety
    /// `bytes` must be valid UTF-8; [`StrBytes::as_str`] relies on it.
    pub const unsafe fn from_utf8_unchecked(bytes: Bytes) -> Self {
        StrBytes(bytes)
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: every constructor validates UTF-8 (or is itself unsafe).
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// The underlying `Bytes`, zero-copy.
    pub fn into_bytes(self) -> Bytes {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for StrBytes {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for StrBytes {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for StrBytes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for StrBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for StrBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl From<&'static str> for StrBytes {
    fn from(s: &'static str) -> Self {
        StrBytes::from_static(s)
    }
}

impl From<String> for StrBytes {
    fn from(s: String) -> Self {
        StrBytes(Bytes::from(s.into_bytes()))
    }
}

impl From<StrBytes> for Bytes {
    fn from(s: StrBytes) -> Bytes {
        s.0
    }
}

impl TryFrom<Bytes> for StrBytes {
    type Error = Utf8Error;
    fn try_from(b: Bytes) -> Result<Self, Utf8Error> {
        StrBytes::from_utf8(b)
    }
}

impl PartialEq<str> for StrBytes {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for StrBytes {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

/// String-backed entity newtype (schema `entityType` annotation).
macro_rules! string_newtype {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub StrBytes);

        impl $name {
            /// Wrap a static string, zero-copy.
            pub const fn from_static(s: &'static str) -> Self {
                $name(StrBytes::from_static(s))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn into_inner(self) -> StrBytes {
                self.0
            }
        }

        impl Deref for $name {
            type Target = StrBytes;
            fn deref(&self) -> &StrBytes {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self.as_str(), f)
            }
        }

        impl From<StrBytes> for $name {
            fn from(s: StrBytes) -> Self {
                $name(s)
            }
        }

        impl From<$name> for StrBytes {
            fn from(v: $name) -> StrBytes {
                v.0
            }
        }

        impl From<&'static str> for $name {
            fn from(s: &'static str) -> Self {
                $name(StrBytes::from_static(s))
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                $name(StrBytes::from(s))
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }
    };
}

/// Integer-backed entity newtype (schema `entityType` annotation).
macro_rules! int_newtype {
    ($(#[$doc:meta])* $name:ident, $t:ty) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $t);

        impl Deref for $name {
            type Target = $t;
            fn deref(&self) -> &$t {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl From<$t> for $name {
            fn from(v: $t) -> Self {
                $name(v)
            }
        }

        impl From<$name> for $t {
            fn from(v: $name) -> $t {
                v.0
            }
        }

        impl PartialEq<$t> for $name {
            fn eq(&self, other: &$t) -> bool {
                self.0 == *other
            }
        }
    };
}

string_newtype! {
    /// A topic name (`entityType: topicName`).
    TopicName
}
string_newtype! {
    /// A consumer/share group id (`entityType: groupId`).
    GroupId
}
string_newtype! {
    /// A transactional producer id (`entityType: transactionalId`).
    TransactionalId
}
int_newtype! {
    /// A broker node id (`entityType: brokerId`).
    BrokerId, i32
}
int_newtype! {
    /// A producer id (`entityType: producerId`).
    ProducerId, i64
}

/// A records payload held as a chain of `Bytes` chunks instead of one
/// contiguous buffer.
///
/// This is the "cargo" type of the shell decode path: record batches read from
/// the wire land here as zero-copy slices of pool-sized read chunks, and the
/// shell encoder splices them back out as refcounted frame segments — the
/// payload is never made contiguous and never copied. Equality compares
/// logical bytes, ignoring chunk boundaries.
#[derive(Debug, Clone, Default)]
pub struct RecordsChunks {
    chunks: Vec<Bytes>,
    len: usize,
}

impl RecordsChunks {
    pub const fn new() -> Self {
        RecordsChunks {
            chunks: Vec::new(),
            len: 0,
        }
    }

    /// Append a chunk (empty chunks are dropped).
    pub fn push(&mut self, chunk: Bytes) {
        if !chunk.is_empty() {
            self.len += chunk.len();
            self.chunks.push(chunk);
        }
    }

    /// Total payload length in bytes, across all chunks.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The chunks, in payload order.
    pub fn chunks(&self) -> &[Bytes] {
        &self.chunks
    }

    pub fn into_chunks(self) -> Vec<Bytes> {
        self.chunks
    }

    /// Copy into one contiguous `Bytes`. A single-chunk payload returns a
    /// cheap refcounted clone; multi-chunk pays one copy — for inspection and
    /// tests, not the hot path.
    pub fn to_contiguous(&self) -> Bytes {
        match self.chunks.as_slice() {
            [] => Bytes::new(),
            [one] => one.clone(),
            many => {
                let mut out = Vec::with_capacity(self.len);
                for c in many {
                    out.extend_from_slice(c);
                }
                Bytes::from(out)
            }
        }
    }
}

impl From<Bytes> for RecordsChunks {
    fn from(b: Bytes) -> Self {
        let mut c = RecordsChunks::new();
        c.push(b);
        c
    }
}

impl PartialEq for RecordsChunks {
    /// Logical byte equality — chunk boundaries don't matter.
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        let (mut ai, mut bi, mut ao, mut bo) = (0usize, 0usize, 0usize, 0usize);
        while ai < self.chunks.len() && bi < other.chunks.len() {
            let a = &self.chunks[ai][ao..];
            let b = &other.chunks[bi][bo..];
            let n = a.len().min(b.len());
            if a[..n] != b[..n] {
                return false;
            }
            ao += n;
            bo += n;
            if ao == self.chunks[ai].len() {
                ai += 1;
                ao = 0;
            }
            if bo == other.chunks[bi].len() {
                bi += 1;
                bo = 0;
            }
        }
        true
    }
}

impl Eq for RecordsChunks {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_bytes_validates_utf8() {
        let ok = StrBytes::from_utf8(Bytes::from_static("héllo wörld — 你好".as_bytes())).unwrap();
        assert_eq!(ok.as_str(), "héllo wörld — 你好");
        // Truncated multibyte sequence and stray continuation bytes are rejected.
        assert!(StrBytes::from_utf8(Bytes::from_static(&[0xe4, 0xbd])).is_err());
        assert!(StrBytes::from_utf8(Bytes::from_static(&[0xff, 0xfe, 0x80])).is_err());
        assert!(StrBytes::try_from(Bytes::from_static(b"\x80")).is_err());
        // Empty is valid UTF-8.
        assert_eq!(StrBytes::from_utf8(Bytes::new()).unwrap(), "");
    }

    #[test]
    fn str_bytes_is_zero_copy() {
        let src = Bytes::from(vec![b'a'; 64]);
        let ptr = src.as_ptr();
        let s = StrBytes::from_utf8(src).unwrap();
        // Validation wraps the same allocation...
        assert_eq!(s.as_bytes().as_ptr(), ptr);
        // ...and unwrapping returns it untouched.
        assert_eq!(Bytes::from(s).as_ptr(), ptr);
    }

    #[test]
    fn str_bytes_string_ergonomics() {
        let s: StrBytes = "topic-a".into();
        // Deref gives the whole &str API.
        assert!(s.starts_with("topic"));
        assert_eq!(s.len(), 7);
        assert_eq!(s, "topic-a");
        assert_eq!(format!("{}", s), "topic-a");
        assert_eq!(format!("{:?}", s), "\"topic-a\"");
        assert_eq!(StrBytes::from("owned".to_string()).as_str(), "owned");
        assert_eq!(StrBytes::default(), StrBytes::new());

        // Borrow<str> means HashMap<StrBytes, _> lookups work with &str keys.
        let mut map = std::collections::HashMap::new();
        map.insert(StrBytes::from_static("k"), 1);
        assert_eq!(map.get("k"), Some(&1));
    }

    #[test]
    fn string_newtypes_convert_and_compare() {
        let t: TopicName = "events".into();
        assert_eq!(t.as_str(), "events");
        assert_eq!(t, "events");
        // Deref chain: TopicName -> StrBytes -> str.
        let s: &str = &t;
        assert_eq!(s, "events");
        assert_eq!(format!("{}", t), "events");

        let sb: StrBytes = t.clone().into();
        assert_eq!(TopicName::from(sb), t);
        assert_eq!(TopicName::from("events".to_string()), t);
        assert_eq!(GroupId::from_static("g1").into_inner(), StrBytes::from_static("g1"));
        assert_eq!(TransactionalId::default().as_str(), "");
    }

    #[test]
    fn int_newtypes_convert_and_compare() {
        let b: BrokerId = 3.into();
        assert_eq!(b, BrokerId(3));
        assert_eq!(b, 3);
        assert_eq!(*b, 3);
        assert_eq!(i32::from(b), 3);
        assert_eq!(format!("{}", b), "3");
        assert_eq!(BrokerId::default(), BrokerId(0));
        assert_eq!(i64::from(ProducerId(-1)), -1);
        // Ordering derives let ids sort naturally.
        let mut v = vec![BrokerId(2), BrokerId(0), BrokerId(1)];
        v.sort();
        assert_eq!(v, vec![BrokerId(0), BrokerId(1), BrokerId(2)]);
    }
}
