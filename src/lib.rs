#![warn(clippy::cargo)]

mod format;
mod ser;

pub use crate::ser::*;
use crate::ser::{SerError, Serializer};
use serde::Serialize;
use std::io;

/// Serialize the given data structure in lua representation into the IO stream.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), SerError>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

/// Serialize the given data structure as a pretty-printed lua representation into the IO
/// stream.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> Result<(), SerError>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::pretty(writer);
    value.serialize(&mut ser)
}

/// Serialize the given data structure in lua representation byte vector.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, SerError>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given data structure as a pretty-printed lua representation byte vector.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>, SerError>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given data structure as a String in lua representation.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_string<T>(value: &T) -> Result<String, SerError>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // Safety: We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given data structure as a pretty-printed String in lua representation.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_string_pretty<T>(value: &T) -> Result<String, SerError>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // Safety: We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use mlua::{Lua, Value};

    #[test]
    fn it_woks() {
        let file = std::fs::read("xd.lua").unwrap();

        let lua = Lua::new();
        lua.load(&file).exec().unwrap();

        let table: Value = lua.globals().get("ALIEN").unwrap();
        to_writer_pretty(io::stdout(), &table).unwrap();
    }
}
