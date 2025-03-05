#![feature(iterator_try_collect)]
use anyhow::Result;
use crypt::{decrypt, encrypt, gen_sra_keys};
use crypto_bigint::{BoxedUint, Limb};
use crypto_primes::generate_prime;

mod crypt;

const P_BITS: u32 = 128;

fn main() -> Result<()> {
    let p: BoxedUint = generate_prime(P_BITS);
    let q: BoxedUint = generate_prime(P_BITS);

    let (e1, d1) = gen_sra_keys(&p, &q)?;
    let (e2, d2) = gen_sra_keys(&p, &q)?;

    let deck: Vec<u32> = vec![2, 3, 4, 5, 6, 7];

    let mut e_deck: Vec<BoxedUint> = deck
        .iter()
        .map(|n| BoxedUint::from(Limb::from_u32(*n)))
        .collect();

    println!("e_deck unencrypted:");
    e_deck.iter().for_each(|m| println!("{:?}", m));

    // c1 scope start
    e_deck = e_deck
        .iter()
        .map(|m| encrypt(&m, e1.clone()))
        .try_collect()?;
    e_deck.sort_unstable();
    // c1 scope end

    println!("e_deck first shuffle:");
    e_deck.iter().for_each(|m| println!("{:?}", m));

    // c2 scope end
    e_deck = e_deck
        .iter()
        .map(|m| encrypt(&m, e2.clone()))
        .try_collect()?;
    e_deck.sort_unstable();
    // c2 scope end

    println!("e_deck second shuffle:");
    e_deck.iter().for_each(|m| println!("{:?}", m));

    // c1 scope start
    let n_deck = e_deck.split_off(2);
    let hand_for2: Vec<BoxedUint> = e_deck
        .iter()
        .map(|m| decrypt(m, d1.clone()))
        .try_collect()?;
    let mut e_deck = n_deck;
    // c1 scope end: hand -> c2

    println!("e_deck first deal:");
    e_deck.iter().for_each(|m| println!("{:?}", m));

    // c2 scope start
    let n_deck = e_deck.split_off(2);
    let hand_for1: Vec<BoxedUint> = e_deck
        .iter()
        .map(|m| decrypt(m, d2.clone()))
        .try_collect()?;
    let e_deck = n_deck;
    // c2 scope end: hand -> c1

    println!("e_deck second deal:");
    e_deck.iter().for_each(|m| println!("{:?}", m));

    println!("c1 hand:");
    hand_for1.iter().for_each(|m| println!("{:?}", m));

    let hand_for1: Vec<BoxedUint> = hand_for1
        .iter()
        .map(|m| decrypt(m, d1.clone()))
        .try_collect()?;

    println!("c1 hand decrypted:");
    hand_for1.iter().for_each(|m| println!("{:?}", m));

    println!("c2 hand:");
    hand_for2.iter().for_each(|m| println!("{:?}", m));

    let hand_for2: Vec<BoxedUint> = hand_for2
        .iter()
        .map(|m| decrypt(m, d2.clone()))
        .try_collect()?;

    println!("c2 hand decrypted:");
    hand_for2.iter().for_each(|m| println!("{:?}", m));

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
