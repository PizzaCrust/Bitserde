use std::mem::size_of;

use bitvec::vec::BitVec;
use bitvec::field::BitField;
use bitvec::order::BitOrder;
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, LE};
use paste::paste;
use serde::export::PhantomData;

use crate::de::BitDeserializer;
use crate::ser::BitSerializer;
use crate::Result;

macro_rules! create_primitive_encoding {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<deserialize_ $type>]<O: BitOrder, T: BitStore>(bytes: &BitSlice<O, T>) -> Result<$type> where BitSlice<O, T>: BitField;
                fn [<serialize_ $type>]<O: BitOrder, T: BitStore>(vec: &mut BitVec<O, T>, value: $type) -> Result<()> where BitSlice<O, T::Alias>: BitField;
            )*
        }
    };
}

macro_rules! impl_primitive_encoding {
    ($endian:ty; $($type:ty),*) => {
        paste! {
            $(
                #[inline]
                fn [<deserialize_ $type>]<O: BitOrder, T: BitStore>(mut bytes: &BitSlice<O, T>) -> Result<$type> where BitSlice<O, T>: BitField, {
                    Ok(bytes.[<read_ $type>]::<$endian>()?)
                }
                #[inline]
                fn [<serialize_ $type>]<O: BitOrder, T: BitStore>(vec: &mut BitVec<O, T>, value: $type) -> Result<()> where BitSlice<O, T::Alias>: BitField {
                    vec.[<write_ $type>]::<$endian>(value)?;
                    Ok(())
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
    #[inline]
    fn deserialize_len<O: BitOrder, S: BitStore, EN: BinaryEncoding>(
        deserializer: &mut BitDeserializer<O, S, EN>,
    ) -> Result<usize>
    where
        BitSlice<O, S>: BitField,
    {
        Ok(Self::deserialize_u32(deserializer.read_bits(size_of::<u32>() * 8))? as usize)
    }
    #[inline]
    fn serialize_len<O: BitOrder + 'static, S: BitStore, EN: BinaryEncoding>(
        serializer: &mut BitSerializer<O, S, EN>,
        len: usize,
    ) -> Result<()>
    where
        BitSlice<O, S::Alias>: BitField,
    {
        Self::serialize_u32(&mut serializer.vec, len as u32)?;
        Ok(())
    }

    #[inline]
    fn deserialize_i8<O: BitOrder, T: BitStore>(mut bytes: &BitSlice<O, T>) -> Result<i8>
    where
        BitSlice<O, T>: BitField,
    {
        Ok(bytes.read_i8()?)
    }

    #[inline]
    fn serialize_i8<O: BitOrder, T: BitStore>(vec: &mut BitVec<O,T>, value: i8) -> Result<()> where BitSlice<O, T::Alias>: BitField {
        vec.write_i8(value)?;
        Ok(())
    }

    impl_primitive_encoding![E; i16, i32, i64, u16, u32, u64, f32, f64];
}
