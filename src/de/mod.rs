use bitvec::slice::BitSlice;
use bitvec::order::{BitOrder, Lsb0};
use bitvec::store::BitStore;
use serde::Deserializer;
use serde::de::{Visitor, SeqAccess, DeserializeSeed};
use crate::*;

pub struct BitDeserializer<'de, O = Lsb0, T = usize> where O: BitOrder, T: BitStore {
    bits: &'de BitSlice<O, T>,
}

impl<'de, O: BitOrder, S: BitStore> BitDeserializer<'de, O, S> {
    fn parse_bit(slice: &'de BitSlice<O, S>) -> Result<(bool, &'de BitSlice<O, S>)> {
        let (bit, rest) = slice.split_at(1);
        Ok((bit[0], rest))
    }
    pub fn new(slice: &'de BitSlice<O, S>) -> Self {
        BitDeserializer {
            bits: slice
        }
    }
}

impl<'de, 'a, O: BitOrder, S: BitStore> Deserializer<'de> for &'a mut BitDeserializer<'de, O, S> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        let (bit, rest) = BitDeserializer::parse_bit(self.bits)?;
        self.bits = rest;
        visitor.visit_bool(bit)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

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
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as Visitor<'de>>::Value> where
        V: Visitor<'de> {
        struct Access<'de, 'a, O: BitOrder, S: BitStore> {
            deserializer: &'a mut BitDeserializer<'de, O, S>,
            len: usize
        }
        impl<'de, 'a, O: BitOrder, S: BitStore> SeqAccess<'de> for Access<'de, 'a, O, S> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'de>>::Value>> where
                T: DeserializeSeed<'de> {
                if self.len > 0 {
                    self.len -= 1;
                    let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }
        visitor.visit_seq(Access {
            deserializer: self,
            len
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