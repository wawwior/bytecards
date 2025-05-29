#![feature(iterator_try_collect)]
use anyhow::Result;
use cards::{Card, Color, Value};
use crypt::{decrypt, encrypt, gen_sra_key};
use crypto_bigint::BoxedUint;
use crypto_primes::{generate_safe_prime, is_prime};

mod cards;
mod crypt;
mod proto;

pub const P_BITS: u32 = 256; // 4 Limbs

fn main() -> Result<()> {
    Ok(()).and(test_crypt())
}

#[allow(dead_code)]
fn big_primes() -> Result<()> {
    let p: BoxedUint = generate_safe_prime(P_BITS);
    let is_prime = is_prime(&p);
    println!("{}", p.to_string_radix_vartime(10));
    println!("{is_prime}");
    Ok(())
}

fn test_crypt() -> Result<()> {
    let p: BoxedUint = generate_safe_prime(P_BITS);
    let q: BoxedUint = generate_safe_prime(P_BITS);

    // sharing any key will break encryption
    let k1 = gen_sra_key(&p, &q)?;
    let k2 = gen_sra_key(&p, &q)?;

    let deck = Color::iter()
        .flat_map(|c| Value::iter().map(move |v| (c, v)))
        .map(|(&c, &v)| Card(c, v));

    // println!("deck unencrypted:");
    // deck.clone().for_each(|m| println!("{:?}", m));

    let mut e_deck: Vec<BoxedUint> = deck.map(|c| BoxedUint::try_from(&c)).try_collect()?;

    // c1 scope start
    e_deck = e_deck.iter().map(|m| encrypt(m, &k1)).try_collect()?;
    e_deck.sort_unstable();
    // c1 scope end

    // println!("e_deck first shuffle:");
    // e_deck.iter().for_each(|m| println!("{:?}", m));

    // c2 scope end
    e_deck = e_deck.iter().map(|m| encrypt(m, &k2)).try_collect()?;
    e_deck.sort_unstable();
    // c2 scope end

    println!("e_deck second shuffle:");
    e_deck.iter().for_each(|m| println!("{m:?}"));

    // c1 scope start
    let n_deck = e_deck.split_off(2);
    let hand_for2: Vec<BoxedUint> = e_deck.iter().map(|m| decrypt(m, &k1)).try_collect()?;
    let mut e_deck = n_deck;
    // c1 scope end: hand -> c2

    // c2 scope start
    let n_deck = e_deck.split_off(2);
    let hand_for1: Vec<BoxedUint> = e_deck.iter().map(|m| decrypt(m, &k2)).try_collect()?;
    let e_deck = n_deck;
    let _ = e_deck;
    // c2 scope end: hand -> c1

    println!("c1 hand:");
    hand_for1.iter().for_each(|m| println!("{m:?}"));

    let hand_for1: Vec<Card> = hand_for1
        .iter()
        .map(|m| decrypt(m, &k1))
        .try_collect::<Vec<BoxedUint>>()?
        .iter()
        .map(Card::try_from)
        .try_collect()?;

    println!("c1 hand decrypted:");
    hand_for1.iter().for_each(|m| println!("{m:?}"));

    println!("c2 hand:");
    hand_for2.iter().for_each(|m| println!("{m:?}"));

    let hand_for2: Vec<Card> = hand_for2
        .iter()
        .map(|m| decrypt(m, &k2))
        .try_collect::<Vec<BoxedUint>>()?
        .iter()
        .map(Card::try_from)
        .try_collect()?;

    println!("c2 hand decrypted:");
    hand_for2.iter().for_each(|m| println!("{m:?}"));

    Ok(())
}

/*

All encryption must be able to be layered without order

> all clients:
  |-- encrypt deck with all PK
  V
> dealer: (until every client has cards)
| |-- chose 2 cards, decrypt once with SK and send to next client
| |-- send reduced deck to next client
| V
| next client:
| |-- become dealer for next client
'-|
  V
> all clients:
  |-- broadcast SK request
  V
  all clients:
  |-- check requester != dealt to
  |-- answer SK request
  V
  all clients:
  |-- decrypt hand with received SK
  '-- deck cant be decrypted; SK of dealer missing.

*/
