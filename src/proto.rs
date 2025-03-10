use crypto_bigint::BoxedUint;
use serde::{Deserialize, Serialize};

pub enum Package {
    Shuffle { deck: Vec<BoxedUint> },
    Hand { cards: (BoxedUint, BoxedUint) },
}
