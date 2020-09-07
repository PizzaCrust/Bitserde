mod error;
mod de;
mod container;

pub use container::{BitContainer,ContainerSize};
pub use error::{Error, Result};
use bitvec::store::BitStore;
use bitvec::order::BitOrder;
use bitvec::slice::BitSlice;
use serde::Deserialize;
use bitvec::field::BitField;

fn deserialize<'a, T: Deserialize<'a>, O: BitOrder, S: BitStore>(bits: &'a BitSlice<O, S>) -> Result<T> where BitSlice<O, S>: BitField {
    let mut deserializer = de::BitDeserializer::new(bits);
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod tests {
    use crate::{deserialize, ContainerSize, BitContainer};
    use bitvec::order::{Lsb0, BitOrder};
    use bitvec::view::BitView;
    use serde::{Serialize, Deserialize};
    use bitvec::store::BitStore;

    #[derive(Deserialize, PartialEq, Debug)]
    struct BitTest(bool, bool, bool, bool, bool, bool, bool, bool);

    #[test]
    fn bit() {
        let data = vec![0x23u8];
        assert_eq!(deserialize::<BitTest, _, _>(data.view_bits::<Lsb0>()).unwrap(), BitTest(true, true, false, false, false, true, false, false))
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct ByteTest(u8, u8, u8);

    #[test]
    fn bytes() {
        let data = vec![0x01u8, 0x02, 0x03];
        assert_eq!(deserialize::<ByteTest, _, _>(data.view_bits::<Lsb0>()).unwrap(), ByteTest(0x01, 0x02, 0x03))
    }

    #[derive(Debug, PartialEq)]
    struct BitsSize;
    impl ContainerSize for BitsSize {
        fn size() -> usize {
            7
        }
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct BitsTest(BitContainer<BitsSize, Lsb0, u8>, bool, u8);

    #[test]
    fn bits() {
        let data = vec![0x23u8, 0x01];
        let obj = deserialize::<BitsTest, _, _>(data.view_bits::<Lsb0>()).unwrap();
        assert_eq!(obj.0.as_byte().unwrap(), 0x23);
        assert_eq!(obj.1, false);
        assert_eq!(obj.2, 0x01);
    }
}
