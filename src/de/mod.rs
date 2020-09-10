use std::io::Read;
use std::marker::PhantomData;
use std::mem::size_of;

use bitvec::field::BitField;
use bitvec::order::{BitOrder, Lsb0};
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use paste::paste;
use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess, Visitor};
use serde::Deserializer;

use crate::*;
use crate::encoding::EndianEncoding;
use crate::error::Error::Unsupported;

pub struct BitDeserializer<'de, O = Lsb0, T = usize, E = EndianEncoding>
where
    O: BitOrder,
    T: BitStore,
    BitSlice<O, T>: BitField,
    E: BinaryEncoding,
{
    pub bits: &'de BitSlice<O, T>,
    endian: PhantomData<E>,
}

impl<'de, O: BitOrder, S: BitStore, E: BinaryEncoding> BitDeserializer<'de, O, S, E>
where
    BitSlice<O, S>: BitField,
{
    fn parse_bit(slice: &'de BitSlice<O, S>) -> Result<(bool, &'de BitSlice<O, S>)> {
        let (bit, rest) = slice.split_at(1);
        Ok((bit[0], rest))
    }
    fn parse_byte(slice: &'de BitSlice<O, S>) -> Result<(u8, &'de BitSlice<O, S>)>
    where
        BitSlice<O, S>: BitField,
    {
        let (mut byte_bits, rest) = slice.split_at(8);
        let mut byte = vec![0u8];
        byte_bits.read(&mut byte[..])?;
        Ok((byte[0], rest))
    }
    pub fn parse_datatype_bytes<T: Sized>(
        slice: &'de BitSlice<O, S>,
    ) -> Result<(Vec<u8>, &'de BitSlice<O, S>)>
    where
        BitSlice<O, S>: BitField,
    {
        let mut bytes = Vec::<u8>::with_capacity(size_of::<T>());
        let mut rest = slice;
        for i in 0..size_of::<T>() {
            let (byte, rem) = Self::parse_byte(rest)?;
            rest = rem;
            bytes.push(byte);
        }
        Ok((bytes, rest))
    }
    pub fn new(slice: &'de BitSlice<O, S>) -> Self {
        BitDeserializer {
            bits: slice,
            endian: PhantomData,
        }
    }
}

macro_rules! impl_encoding_deserialization {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<deserialize_ $type>]<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where V: Visitor<'de> {
                    let (bytes, rest) = BitDeserializer::<O, S, E>::parse_datatype_bytes::<$type>(self.bits)?;
                    self.bits = rest;
                    visitor.[<visit_ $type>](E::[<deserialize_ $type>](bytes)?)
                }
            )*
        }
    };
}

impl<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> Deserializer<'de>
    for &'a mut BitDeserializer<'de, O, S, E>
where
    BitSlice<O, S>: BitField,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        let (bit, rest) = BitDeserializer::<O, S, E>::parse_bit(self.bits)?;
        self.bits = rest;
        visitor.visit_bool(bit)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        let (byte, rest) = BitDeserializer::<O, S, E>::parse_byte(self.bits)?;
        self.bits = rest;
        visitor.visit_u8(byte)
    }

    impl_encoding_deserialization![i8, i16, i32, i64, u16, u32, u64, f32, f64];

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        let len = E::deserialize_len(self)?;
        let mut bytes = vec![0u8; len];
        self.bits.read(&mut bytes[..])?;
        visitor.visit_bytes(&bytes[..])
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        let len = E::deserialize_len(self)?;
        let mut bytes = vec![0u8; len];
        self.bits.read(&mut bytes[..])?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        let len = E::deserialize_len(self)?;
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        struct Access<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding>
        where
            BitSlice<O, S>: BitField,
        {
            deserializer: &'a mut BitDeserializer<'de, O, S, E>,
            len: Option<usize>,
        }
        impl<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> SeqAccess<'de>
            for Access<'de, 'a, O, S, E>
        where
            BitSlice<O, S>: BitField,
        {
            type Error = Error;

            fn next_element_seed<T>(
                &mut self,
                seed: T,
            ) -> Result<Option<<T as DeserializeSeed<'de>>::Value>>
            where
                T: DeserializeSeed<'de>,
            {
                if let Some(length) = self.len {
                    if length > 0 {
                        self.len = Some(length - 1);
                        Ok(Some(serde::de::DeserializeSeed::deserialize(
                            seed,
                            &mut *self.deserializer,
                        )?))
                    } else {
                        Ok(None)
                    }
                } else {
                    let value =
                        serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                }
            }
        }
        visitor.visit_seq(Access {
            deserializer: self,
            len: if len > 0 { Some(len) } else { None },
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        impl<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> serde::de::VariantAccess<'de>
            for &'a mut BitDeserializer<'de, O, S, E>
        where
            BitSlice<O, S>: BitField,
        {
            type Error = Error;

            fn unit_variant(self) -> Result<()> {
                Ok(())
            }

            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
            where
                T: DeserializeSeed<'de>,
            {
                serde::de::DeserializeSeed::deserialize(seed, self)
            }

            fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
            where
                V: Visitor<'de>,
            {
                serde::de::Deserializer::deserialize_tuple(self, len, visitor)
            }

            fn struct_variant<V>(
                self,
                fields: &'static [&'static str],
                visitor: V,
            ) -> Result<V::Value>
            where
                V: Visitor<'de>,
            {
                serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
            }
        }
        impl<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> serde::de::EnumAccess<'de>
            for &'a mut BitDeserializer<'de, O, S, E>
        where
            BitSlice<O, S>: BitField,
        {
            type Error = Error;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
            where
                V: serde::de::DeserializeSeed<'de>,
            {
                let index = E::deserialize_len(self)?;
                let val: Result<_> = seed.deserialize(index.into_deserializer());
                Ok((val?, self))
            }
        }
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value>
    where
        V: Visitor<'de>,
    {
        Err(Unsupported)
    }
}
