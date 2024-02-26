use crate::{types::key::Key, KEY_SIZE};

// uint::construct_uint! {
//     /// 256-bit unsigned integer.
//     pub(super) struct U256(4);
// }

/// benchmark this
pub fn bucket_index(local: &Key, key: &Key) -> usize {
    let distance = local.distance(key);

    // taken from libp2p
    // let l = U256::from(local.0.as_slice());
    // let k = U256::from(key.0.as_slice());

    // let distance = k ^ l;

    // taken from stack overflow
    for i in 0..KEY_SIZE {
        for j in (0..8).rev() {
            let bit = distance.0[i] >> (7 - j);
            if bit & 0x1 != 0 {
                return i * 8 + j;
            }
        }
    }

    // super::KEY_SIZE * 8 - 1
    // let index = (256 - distance.leading_zeros()).checked_sub(1);
    // println!("{:?}", index);

    // index.unwrap_or_default() as usize
    KEY_SIZE * 8 - 1
}

#[test]
fn bucket_index_test() {
    // use rand::Rng;

    let key = Key::new("test".to_owned());
    dbg!(key, bucket_index(&key, &key));

    let a = Key([0; super::KEY_SIZE]);
    let mut b = Key([0; super::KEY_SIZE]);
    b.0[KEY_SIZE - 1] = u8::MAX;
    dbg!(a, b, bucket_index(&a, &b));

    // let mut rng = rand::thread_rng();
    // for _ in 0..100 {
    //     let mut a = Key([0; super::KEY_SIZE]);
    //     let mut b = Key([0; super::KEY_SIZE]);

    //     for i in 0..KEY_SIZE {
    //         a.0[i] = rng.gen();
    //         b.0[i] = rng.gen();
    //     }

    //     dbg!(a, b, bucket_index(&a, &b));
    //     println!();
    // }
    // panic!();
}
