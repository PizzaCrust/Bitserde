use bitvec::slice::BitSlice;
use bitvec::order::{BitOrder, Lsb0};
use bitvec::store::BitStore;
use serde::Deserializer;
use serde::de::{Visitor, SeqAccess, DeserializeSeed};
use crate::*;
use bitvec::vec::BitVec;
use std::io::Read;
use bitvec::field::BitField;
use std::mem::size_of;
use crate::encoding::EndianEncoding;
use std::marker::PhantomData;
use paste::paste;

pub struct BitDeserializer<'de, O = Lsb0, T = usize, E = EndianEncoding> where O: BitOrder, T: BitStore, BitSlice<O, T>: BitField, E: BinaryEncoding {
    pub bits: &'de BitSlice<O, T>,
    endian: PhantomData<E>
}

impl<'de, O: BitOrder, S: BitStore, E: BinaryEncoding> BitDeserializer<'de, O, S, E> where BitSlice<O, S>: BitField {
    fn parse_bit(slice: &'de BitSlice<O, S>) -> Result<(bool, &'de BitSlice<O, S>)> {
        let (bit, rest) = slice.split_at(1);
        Ok((bit[0], rest))
    }
    fn parse_byte(slice: &'de BitSlice<O, S>) -> Result<(u8, &'de BitSlice<O, S>)> where BitSlice<O,S>: BitField {
        let (mut byte_bits, rest) = slice.split_at(8);
        let mut byte = vec![0u8];
        byte_bits.read(&mut byte[..]);
        Ok((byte[0], rest))
    }
    pub fn parse_datatype_bytes<T: Sized>(slice: &'de BitSlice<O, S>) -> Result<(Vec<u8>, &'de BitSlice<O, S>)> where BitSlice<O,S>: BitField {
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
            endian: PhantomData
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

impl<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> Deserializer<'de> for &'a mut BitDeserializer<'de, O, S, E> where BitSlice<O, S>: BitField {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        let (bit, rest) = BitDeserializer::<O, S, E>::parse_bit(self.bits)?;
        self.bits = rest;
        visitor.visit_bool(bit)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        let (byte, rest) = BitDeserializer::<O, S, E>::parse_byte(self.bits)?;
        self.bits = rest;
        visitor.visit_u8(byte)
    }

    impl_encoding_deserialization![i8, i16, i32, i64, u16, u32, u64, f32, f64];

    fn deserialize_char<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        let len = E::deserialize_len(self)?;
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        struct Access<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> where BitSlice<O, S>: BitField {
            deserializer: &'a mut BitDeserializer<'de, O, S, E>,
            len: Option<usize>
        }
        impl<'de, 'a, O: BitOrder, S: BitStore, E: BinaryEncoding> SeqAccess<'de> for Access<'de, 'a, O, S, E> where BitSlice<O, S>: BitField {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'de>>::Value>> where
                T: DeserializeSeed<'de> {
                if let Some(length) = self.len {
                    if length > 0 {
                        self.len = Some(length - 1);
                        Ok(Some(serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?))
                    } else {
                        Ok(None)
                    }
                } else {
                    let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                }
            }
        }
        visitor.visit_seq(Access {
            deserializer: self,
            len: if len > 0 { Some(len) } else { None }
        })
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }
}