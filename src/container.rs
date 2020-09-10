use bitvec::field::BitField;
use bitvec::order::BitOrder;
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use serde::de::{Error, SeqAccess, Visitor};
use serde::export::Formatter;
use serde::ser::{SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

/// Represents a bit container size.
pub trait ContainerSize {
    /// The size of the bit container.
    fn size() -> usize;
}

/// This represents a collection of bits that have a specific capacity defined which is used to deserialize and serialize the bit data at runtime.
#[derive(PartialEq, Debug)]
pub struct BitContainer<T: ContainerSize, O: BitOrder, S: BitStore>(BitVec<O, S>, PhantomData<T>);

impl<T: ContainerSize, O: BitOrder, S: BitStore> BitContainer<T, O, S>
where
    BitSlice<O, S>: BitField,
{
    /// If the container is 8 bits or under, it can represent the bit vector as a single byte.
    pub fn as_byte(&self) -> crate::Result<u8> {
        if T::size() > 8 {
            return Err(crate::Error::Message(
                "Tried to convert a bit vector larger then a single byte into a single byte."
                    .to_owned(),
            ));
        }
        let mut result = 0u8;
        for i in 0..T::size() {
            if self.0[i] {
                result |= 1 << i;
            }
        }
        Ok(result)
    }
}

struct BitContainerVisitor<T: ContainerSize, O: BitOrder, S: BitStore>(
    PhantomData<T>,
    PhantomData<O>,
    PhantomData<S>,
);
impl<'de, T: ContainerSize, O: BitOrder, S: BitStore> Visitor<'de>
    for BitContainerVisitor<T, O, S>
{
    type Value = BitContainer<T, O, S>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(&*format!("expecting {} bits", T::size()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = BitVec::<O, S>::with_capacity(T::size());
        for _ in 0..T::size() {
            vec.push(
                seq.next_element::<bool>()?
                    .ok_or(A::Error::custom("Couldn't grab next bit"))?,
            )
        }
        Ok(BitContainer(vec, PhantomData))
    }
}

impl<'de, T: ContainerSize, O: BitOrder, S: BitStore> Deserialize<'de> for BitContainer<T, O, S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct(
            "BitContainer",
            0,
            BitContainerVisitor(PhantomData, PhantomData, PhantomData),
        )
    }
}

impl<T: ContainerSize, O: BitOrder, X: BitStore> Serialize for BitContainer<T, O, X> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for x in &self.0 {
            seq.serialize_element(x)?;
        }
        seq.end()
    }
}
