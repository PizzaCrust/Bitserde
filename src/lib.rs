mod error;
mod de;

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
    use crate::deserialize;
    use bitvec::order::Lsb0;
    use bitvec::view::BitView;
    use serde::{Serialize, Deserialize};

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
}
