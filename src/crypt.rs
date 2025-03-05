use anyhow::Result;
use anyhow::anyhow;
use crypto_bigint::{
    BoxedUint, Gcd, NonZero, Odd, RandomMod,
    modular::{BoxedMontyForm, BoxedMontyParams},
    rand_core::{CryptoRngCore, OsRng},
};

#[derive(Debug, Clone)]
pub struct EncryptionKey {
    e: BoxedUint,
    n: NonZero<BoxedUint>,
    n_params: BoxedMontyParams,
}

impl EncryptionKey {
    pub fn new(n: BoxedUint, e: BoxedUint) -> Result<Self> {
        let n_odd = Odd::new(n.clone())
            .into_option()
            .ok_or(anyhow!("Invalid Modulo"))?;
        let n_params = BoxedMontyParams::new(n_odd);
        let n = NonZero::new(n).unwrap();

        Ok(Self { e, n, n_params })
    }
}

#[derive(Debug, Clone)]
pub struct DecryptionKey {
    d: BoxedUint,
    n: NonZero<BoxedUint>,
    n_params: BoxedMontyParams,
}

impl DecryptionKey {
    pub fn new(n: BoxedUint, d: BoxedUint) -> Result<Self> {
        let n_odd = Odd::new(n.clone())
            .into_option()
            .ok_or(anyhow!("Invalid Modulo"))?;
        let n_params = BoxedMontyParams::new(n_odd);
        let n = NonZero::new(n).unwrap();

        Ok(Self { d, n, n_params })
    }
}

pub fn gen_sra_keys(p: &BoxedUint, q: &BoxedUint) -> Result<(EncryptionKey, DecryptionKey)> {
    let n = p.mul(q);
    let phi = (p - BoxedUint::one()).mul(&(q - BoxedUint::one()));
    let phi_nz = NonZero::new(phi.clone()).unwrap();
    let mut rng = OsRng::default();
    let mut e = BoxedUint::random_mod(rng.as_rngcore(), &phi_nz);

    while phi.gcd(&e) != BoxedUint::one() {
        println!("GCD not 1, trying again.");
        e = BoxedUint::random_mod(rng.as_rngcore(), &phi_nz);
    }

    let d = e.inv_mod(&phi).unwrap();

    Ok((
        EncryptionKey::new(n.clone(), e)?,
        DecryptionKey::new(n.clone(), d)?,
    ))
}

pub fn encrypt(msg: &BoxedUint, key: EncryptionKey) -> Result<BoxedUint> {
    Ok(reduce(msg, &key.n_params).pow(&key.e).retrieve())
}

pub fn decrypt(msg: &BoxedUint, key: DecryptionKey) -> Result<BoxedUint> {
    Ok(reduce(msg, &key.n_params).pow(&key.d).retrieve())
}

fn reduce(n: &BoxedUint, p: &BoxedMontyParams) -> BoxedMontyForm {
    let precision = p.modulus().bits_precision();
    let modulus = p.modulus().as_nz_ref().clone();

    let n = match n.bits_precision().cmp(&precision) {
        std::cmp::Ordering::Less => n.widen(precision),
        std::cmp::Ordering::Equal => n.clone(),
        std::cmp::Ordering::Greater => n.shorten(precision),
    };

    let n_reduced = n.rem_vartime(&modulus).widen(p.bits_precision());
    BoxedMontyForm::new(n_reduced, p.clone())
}
