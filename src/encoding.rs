use crate::de::BitDeserializer;
use crate::ser::BitSerializer;
use crate::Result;
use bitvec::field::BitField;
use bitvec::order::BitOrder;
use bitvec::prelude::BitView;
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, LE};
use paste::paste;
use serde::export::PhantomData;

macro_rules! create_primitive_encoding {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<deserialize_ $type>](bytes: Vec<u8>) -> Result<$type>;
                fn [<serialize_ $type>](value: $type) -> Result<Vec<u8>>;
            )*
        }
    };
}

macro_rules! impl_primitive_encoding {
    ($endian:ty; $($type:ty),*) => {
        paste! {
            $(
                fn [<deserialize_ $type>](bytes: Vec<u8>) -> Result<$type> {
                    Ok(bytes.as_slice().[<read_ $type>]::<$endian>()?)
                }
                fn [<serialize_ $type>](value: $type) -> Result<Vec<u8>> {
                    let mut vec = Vec::with_capacity(std::mem::size_of::<$type>());
                    vec.[<write_ $type>]::<$endian>(value)?;
                    Ok(vec)
                }
            )*
        }
    };
}

pub trait BinaryEncoding {
    fn deserialize_len<O: BitOrder, S: BitStore, EN: BinaryEncoding>(
        deserializer: &mut BitDeserializer<O, S, EN>,
    ) -> Result<usize>
    where
        BitSlice<O, S>: BitField;

    fn serialize_len<O: BitOrder + 'static, S: BitStore, E: BinaryEncoding>(
        serializer: &mut BitSerializer<O, S, E>,
        len: usize,
    ) -> Result<()>
    where
        BitSlice<O, S::Alias>: BitField;

    create_primitive_encoding![i8, i16, i32, i64, u16, u32, u64, f32, f64];
}

pub struct EndianEncoding<E = LE>(PhantomData<E>)
where
    E: ByteOrder;

impl<E: ByteOrder> BinaryEncoding for EndianEncoding<E> {
    fn deserialize_len<O: BitOrder, S: BitStore, EN: BinaryEncoding>(
        deserializer: &mut BitDeserializer<O, S, EN>,
    ) -> Result<usize>
    where
        BitSlice<O, S>: BitField,
    {
        let (bytes, rest) =
            BitDeserializer::<O, S, EN>::parse_datatype_bytes::<u32>(deserializer.bits)?;
        deserializer.bits = rest;
        Ok(Self::deserialize_u32(bytes)? as usize)
    }

    fn serialize_len<O: BitOrder + 'static, S: BitStore, EN: BinaryEncoding>(
        serializer: &mut BitSerializer<O, S, EN>,
        len: usize,
    ) -> Result<()>
    where
        BitSlice<O, S::Alias>: BitField,
    {
        serializer.vec.extend(
            Self::serialize_u32(len as u32)?
                .view_bits::<O>()
                .to_bitvec(),
        );
        Ok(())
    }

    fn deserialize_i8(bytes: Vec<u8>) -> Result<i8> {
        Ok(bytes.as_slice().read_i8()?)
    }

    fn serialize_i8(value: i8) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(1);
        vec.as_mut_slice().write_i8(value)?;
        Ok(vec)
    }

    impl_primitive_encoding![E; i16, i32, i64, u16, u32, u64, f32, f64];
}
