use std::fmt::Display;
use std::io::Write;
use std::marker::PhantomData;

use bitvec::field::BitField;
use bitvec::order::{BitOrder, Lsb0};
use bitvec::prelude::BitView;
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use paste::paste;
use serde::{Serialize, Serializer};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};

use crate::encoding::{BinaryEncoding, EndianEncoding};
use crate::Error;
use crate::error::Error::Unsupported;

pub struct BitSerializer<O = Lsb0, T = usize, E = EndianEncoding>
where
    O: BitOrder,
    T: BitStore,
    E: BinaryEncoding,
{
    pub vec: BitVec<O, T>,
    pub(crate) endian: PhantomData<E>,
}

pub struct Compound<'a, O: BitOrder, S: BitStore, E: BinaryEncoding> {
    ser: &'a mut BitSerializer<O, S, E>,
}

macro_rules! impl_encoding_serialization {
    ($($type:ty),*) => {
        paste! {
            $(
                fn [<serialize_ $type>](self, v: $type) -> Result<Self::Ok, Self::Error> {
                    self.vec.extend(E::[<serialize_ $type>](v)?.view_bits::<O>().to_bitvec());
                    Ok(())
                }
            )*
        }
    };
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> Serializer
    for &'a mut BitSerializer<O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, O, S, E>;
    type SerializeTuple = Compound<'a, O, S, E>;
    type SerializeTupleStruct = Compound<'a, O, S, E>;
    type SerializeTupleVariant = Compound<'a, O, S, E>;
    type SerializeMap = Compound<'a, O, S, E>;
    type SerializeStruct = Compound<'a, O, S, E>;
    type SerializeStructVariant = Compound<'a, O, S, E>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.vec.push(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.vec.write(&[v])?;
        Ok(())
    }

    impl_encoding_serialization![i8, i16, i32, i64, u16, u32, u64, f32, f64];

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(Unsupported)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Unsupported)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        E::serialize_len(self, v.len())?;
        self.vec.write(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Unsupported)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        E::serialize_len(self, variant_index as usize)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        E::serialize_len(self, variant_index as usize)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(size) = len {
            E::serialize_len(self, size)?;
        }
        Ok(Compound { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        E::serialize_len(self, variant_index as usize)?;
        Ok(Compound { ser: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        E::serialize_len(self, variant_index as usize)?;
        Ok(Compound { ser: self })
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        unimplemented!()
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeStructVariant
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeStruct
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeMap
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeTupleVariant
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeTupleStruct
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeTuple
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding> SerializeSeq
    for Compound<'a, O, S, E>
where
    BitSlice<O, S::Alias>: BitField,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
