#![feature(test)]

use bitvec::field::BitField;
use bitvec::order::BitOrder;
use bitvec::slice::BitSlice;
use bitvec::store::BitStore;
use bitvec::vec::BitVec;
use serde::{Deserialize, Serialize};

pub use container::{BitContainer, ContainerSize};
pub use error::{Error, Result};

use crate::encoding::BinaryEncoding;

mod container;
mod de;
mod encoding;
mod error;
mod ser;

fn deserialize<'a, T: Deserialize<'a>, O: BitOrder, S: BitStore, E: BinaryEncoding>(
    bits: &'a BitSlice<O, S>,
) -> Result<T>
where
    BitSlice<O, S>: BitField,
{
    let mut deserializer = de::BitDeserializer::<'_, O, S, E>::new(bits);
    T::deserialize(&mut deserializer)
}

fn serialize<T: Serialize, O: BitOrder + 'static, S: BitStore, E: BinaryEncoding>(
    value: &T,
) -> Result<BitVec<O, S>>
where
    BitSlice<O, S::Alias>: BitField,
{
    let mut serializer = ser::BitSerializer::<O, S, E> {
        vec: BitVec::new(),
        endian: Default::default(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.vec.clone())
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use bitvec::order::Lsb0;
    use bitvec::view::BitView;
    use serde::{Deserialize, Serialize};

    use crate::encoding::EndianEncoding;
    use crate::{deserialize, serialize, BitContainer, ContainerSize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct BitTest(bool, bool, bool, bool, bool, bool, bool, bool);

    #[test]
    fn bit() {
        let data = vec![0x23u8];
        let obj = deserialize::<BitTest, _, _, EndianEncoding>(data.view_bits::<Lsb0>()).unwrap();
        assert_eq!(
            obj,
            BitTest(true, true, false, false, false, true, false, false)
        );
        assert_eq!(
            serialize::<_, Lsb0, u8, EndianEncoding>(&obj).unwrap(),
            data.view_bits::<Lsb0>().to_bitvec()
        );
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct ByteTest(u8, u8, u8);

    #[test]
    fn bytes() {
        let data = vec![0x01u8, 0x02, 0x03];
        let obj = deserialize::<ByteTest, _, _, EndianEncoding>(data.view_bits::<Lsb0>()).unwrap();
        assert_eq!(obj, ByteTest(0x01, 0x02, 0x03));
        assert_eq!(
            serialize::<_, Lsb0, u8, EndianEncoding>(&obj).unwrap(),
            data.view_bits::<Lsb0>().to_bitvec()
        );
    }

    #[derive(Debug, PartialEq)]
    struct BitsSize;
    impl ContainerSize for BitsSize {
        fn size() -> usize {
            7
        }
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct BitsTest(BitContainer<BitsSize, Lsb0, u8>, bool, u8);

    #[test]
    fn bits() {
        let data = vec![0x23u8, 0x01];
        let obj = deserialize::<BitsTest, _, _, EndianEncoding>(data.view_bits::<Lsb0>()).unwrap();
        assert_eq!(obj.0.as_byte().unwrap(), 0x23);
        assert_eq!(obj.1, false);
        assert_eq!(obj.2, 0x01);
        assert_eq!(
            serialize::<_, Lsb0, u8, EndianEncoding>(&obj).unwrap(),
            data.view_bits::<Lsb0>().to_bitvec()
        )
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct VectorTest(f64, Vec<bool>);

    #[test]
    fn vector_test() {
        let obj = VectorTest(1.104321, vec![true, true, false, false, true, true]);
        let bits = serialize::<_, Lsb0, u8, EndianEncoding>(&obj).unwrap();
        assert_eq!(bits.len(), 102);
        let obj2 = deserialize::<VectorTest, _, _, EndianEncoding>(bits.as_bitslice()).unwrap();
        assert_eq!(obj, obj2);
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum TestEnum {
        False(bool),
        True(u32),
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestEnumStruct(Vec<TestEnum>);

    #[test]
    fn enum_test() {
        let test = TestEnumStruct(vec![
            TestEnum::False(true),
            TestEnum::True(102040),
            TestEnum::False(false),
        ]);
        let bits = serialize::<_, Lsb0, u8, EndianEncoding>(&test).unwrap();
        let test2 =
            deserialize::<TestEnumStruct, _, _, EndianEncoding>(bits.as_bitslice()).unwrap();
        assert_eq!(test, test2);
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct BooleanTest(Vec<bool>);

    #[bench]
    fn boolean(b: &mut Bencher) {
        // 4 ns/iter
        let obj = BooleanTest(vec![true, false]);
        let bits = serialize::<_, Lsb0, u8, EndianEncoding>(&obj).unwrap();
        b.iter(|| {
            deserialize::<BooleanTest, _, _, EndianEncoding>(bits.as_bitslice()).unwrap();
        })
    }
}
