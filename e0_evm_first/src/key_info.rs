use alloy::
{
    primitives::
    {
        Address, 
        FixedBytes, 
        B256,
    }, 
    signers::
    {
        self, 
        local::LocalSigner,
    }
};
use k256::
{
    ecdsa::
    {
        SigningKey, 
        VerifyingKey,
    }, 
    elliptic_curve::
    {
        rand_core::
        {
            CryptoRng, RngCore
        }, 
        NonZeroScalar
    },
    Secp256k1, SecretKey,
};
// use std::time;
pub type GenericSigner = LocalSigner<SigningKey>;

/// Generates a new GenericSigner instance with a random signing key.
/// 
/// This function creates a new GenericSigner, which is a LocalSigner
/// using a SigningKey, by generating a random signing key.
pub fn generate_new_signer() -> GenericSigner
{
    GenericSigner::random()
}

/// Generates a new GenericSigner instance with a random signing key, seeded with a custom RNG.
/// 
/// This function creates a new GenericSigner, which is a LocalSigner
/// using a SigningKey, by generating a random signing key using the
/// provided RNG.
pub fn generate_new_seeded_signer<R: RngCore + CryptoRng>(rng: &mut R) -> GenericSigner
{
    GenericSigner::random_with(rng)
}

/// Returns the private key of the given GenericSigner as a NonZeroScalar.
/// 
///The private key is a NonZeroScalar of the Secp256k1 curve.
///
/// note: `Secp256k1` implements Display, not Debug
pub fn derive_private_key(generic_signer: &GenericSigner) -> NonZeroScalar<Secp256k1>
{
    *generic_signer.as_nonzero_scalar()
}

/// Returns the private key of the given GenericSigner as a String.
/// 
///The private key is a NonZeroScalar of the Secp256k1 curve.
pub fn derive_private_key_as_string(generic_signer: &GenericSigner) -> String
{
    generic_signer.as_nonzero_scalar().to_string()
}

/// Returns the private key of the given GenericSigner as a B256.
/// 
///The private key is a B256 of the Secp256k1 curve.
pub fn derive_private_key_as_bytes(generic_signer: &GenericSigner) -> B256
{
    generic_signer.to_bytes()
}

/// Tries to return the private key of the given GenericSigner as a String without the "0x" prefix.
/// 
/// The private key is a B256 of the Secp256k1 curve.
pub fn get_b256_string_without_hex_identifier(b256: &B256) -> String
{
    let string = b256.to_string();

    if let Some(truncated_string) = string.strip_prefix("0x")
    {
        truncated_string.to_string()
    }
    else
    {
        string
    }
}   

/// Returns a string representation of a FixedBytes type, without the "0x" prefix.
/// 
/// The returned string is a substring of the original FixedBytes::to_string() output,
/// starting from the 3rd character (index 2).
/// 
/// note: all FixedBytes types print with the prefix "0x" when `#` flag is not set.
pub fn remove_hex_prefix_from_fixed_bytes<const N: usize>(fixed_bytes: &FixedBytes<N>) -> String
{
    fixed_bytes.to_string()[2..].to_string()
}

/// Returns the Ethereum address of the given GenericSigner.
/// 
/// This function retrieves the address associated with the GenericSigner,
/// which is derived from the signer's public key.
/// 
/// note: the address can be compressed when printed with the `#` flag.
pub fn derive_address_from_generic_signer(generic_signer: &GenericSigner) -> Address
{
    generic_signer.address()
}

/// Returns the Ethereum address derived from the given SigningKey.
/// 
/// This function retrieves the address associated with the given SigningKey,
/// which is derived from the public key of the SigningKey.
/// 
/// note: the address can be compressed when printed with the `#` flag.
pub fn derive_address_from_signing_key(signing_key: &SigningKey) -> Address
{
    signers::utils::secret_key_to_address(signing_key)
}

/// Derives a VerifyingKey from a SigningKey.
///
/// This function returns a VerifyingKey associated with the given SigningKey.
/// The returned VerifyingKey is derived from the public key of the SigningKey.
pub fn derive_verifying_key_from_signing_key(signing_key: &SigningKey) -> VerifyingKey
{
    *signing_key.verifying_key()
}

/// Returns the Ethereum address derived from the given VerifyingKey.
///
/// This function retrieves the address associated with the given VerifyingKey,
/// which is derived from the public key of the VerifyingKey.
/// 
/// note: the address can be compressed when printed with the `#` flag.
pub fn derive_address_from_verifying_key(verifying_key: &VerifyingKey) -> Address
{
    signers::utils::public_key_to_address(verifying_key)
}

/// Derives a SecretKey from a FixedBytes input.
/// 
/// This function takes a FixedBytes type, which can also be a B256, and attempts
/// to convert it into a SecretKey. The conversion involves extracting the raw bytes
/// from the FixedBytes input and converting them into a NonZeroScalar of the Secp256k1 curve.
/// An error is raised if the byte array is invalid.
/// 
/// note: this also takes B256
pub fn derive_secret_key_from_fixed_bytes<const N: usize>(fixed_bytes: &FixedBytes<N>) -> SecretKey
{
    let raw_bytes: &[u8] = fixed_bytes.as_slice();

    let nonzero_scalar: NonZeroScalar<Secp256k1> = NonZeroScalar::try_from(raw_bytes).expect("invalid byte array");

    SecretKey::from(nonzero_scalar)
}

pub fn test<const N: usize>(fixed_bytes: &FixedBytes<N>) -> SecretKey
{
    SecretKey::from_slice(fixed_bytes.as_slice()).expect("invalid byte array")
}

// pub fn derive_public_key_from_secret_key