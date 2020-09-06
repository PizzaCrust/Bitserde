mod error;
mod de;

pub use error::{Error, Result};
use bitvec::store::BitStore;
use bitvec::order::BitOrder;
use bitvec::slice::BitSlice;
use serde::Deserialize;

fn deserialize<'a, T: Deserialize<'a>, O: BitOrder, S: BitStore>(bits: &'a BitSlice<O, S>) -> Result<T> {
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
    struct BitsTest(bool, bool, bool, bool, bool, bool, bool, bool);

    #[test]
    fn bits() {
        let data = vec![0x23u8];
        assert_eq!(deserialize::<BitsTest, _, _>(data.view_bits::<Lsb0>()).unwrap(), BitsTest(true, true, false, false, false, true, false, false))
    }
}
