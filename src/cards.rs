use std::slice::Iter;

use self::Color::*;
use self::Value::*;
use anyhow::Result;
use crypto_bigint::BoxedUint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Card(pub Color, pub Value);

impl TryFrom<&BoxedUint> for Card {
    type Error = postcard::Error;
    fn try_from(value: &BoxedUint) -> Result<Self, Self::Error> {
        let value = value - BoxedUint::from(2u32);
        postcard::from_bytes(&*value.to_le_bytes())
    }
}

impl TryFrom<&Card> for BoxedUint {
    type Error = anyhow::Error;

    fn try_from(val: &Card) -> Result<Self, Self::Error> {
        let uint = BoxedUint::from_le_slice(postcard::to_allocvec(val)?.as_slice(), 16)?
            + BoxedUint::from(2u8);
        Ok(uint)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Color {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl Color {
    pub fn iter() -> Iter<'static, Color> {
        static COLORS: [Color; 4] = [Diamonds, Hearts, Spades, Clubs];
        COLORS.iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn iter() -> Iter<'static, Value> {
        static VALUES: [Value; 13] = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];
        VALUES.iter()
    }
}
