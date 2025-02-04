use alloy::signers::local::
{
    LocalSigner,
    PrivateKeySigner,
};
use k256::
{
    ecdsa::SigningKey, 
    elliptic_curve::
    {
        rand_core::
        {
            CryptoRng, RngCore
        }, 
        NonZeroScalar
    },
    Secp256k1,
};
// use rand::{self, rng::Rng};
// use rand::prelude::*;

pub type GenericSigner = LocalSigner<SigningKey>;

pub fn generate_new_signer() -> GenericSigner
{
    GenericSigner::random()
}

pub fn generate_new_seeded_signer<R: RngCore + CryptoRng>(rng: &mut R) -> GenericSigner
{
    GenericSigner::random_with(rng)
}

pub fn derive_private_key(generic_signer: &GenericSigner) -> &NonZeroScalar<Secp256k1>
{
    generic_signer.as_nonzero_scalar()
}

pub fn generate_seeded_private_key<R: RngCore + CryptoRng>(rng: &mut R) -> GenericSigner
{
    GenericSigner::random_with(rng)
}