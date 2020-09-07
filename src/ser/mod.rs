use bitvec::order::{BitOrder, Lsb0};
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use serde::{Serializer, Serialize};
use crate::Error;
use std::fmt::Display;
use std::io::Write;
use serde::ser::{SerializeStructVariant, SerializeStruct, SerializeMap, SerializeTupleVariant, SerializeTupleStruct, SerializeTuple, SerializeSeq};
use bitvec::slice::BitSlice;
use bitvec::field::BitField;

pub struct BitSerializer<O = Lsb0, T = usize> where O: BitOrder, T: BitStore {
    pub vec: BitVec<O, T>
}

pub struct Compound<'a, O: BitOrder, S: BitStore> {
    ser: &'a mut BitSerializer<O, S>
}

impl<'a, O: BitOrder, S: BitStore> Serializer for &'a mut BitSerializer<O, S> where BitSlice<O, S::Alias>: BitField {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, O, S>;
    type SerializeTuple = Compound<'a, O, S>;
    type SerializeTupleStruct = Compound<'a, O, S>;
    type SerializeTupleVariant = Compound<'a, O, S>;
    type SerializeMap = Compound<'a, O, S>;
    type SerializeStruct = Compound<'a, O, S>;
    type SerializeStructVariant = Compound<'a, O, S>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.vec.push(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.vec.write(&[v]);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where
        T: Display {
        unimplemented!()
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeStructVariant for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField  {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeStruct for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeMap for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeTupleVariant for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
       value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeTupleStruct for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField  {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeTuple for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, O: BitOrder, S: BitStore> SerializeSeq for Compound<'a, O, S> where BitSlice<O, S::Alias>: BitField {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}