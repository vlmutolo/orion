// MIT License

// Copyright (c) 2018 brycx

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use hazardous::constants::{HLenArray, HLEN};
use hazardous::hmac;
use utilities::{errors::*, util};

/// HKDF (HMAC-based Extract-and-Expand Key Derivation Function) as specified in the
/// [RFC 5869](https://tools.ietf.org/html/rfc5869).
///
/// Fields `salt`, `ikm` and `info` are zeroed out on drop.

/// HKDF (HMAC-based Extract-and-Expand Key Derivation Function) as specified in the
/// [RFC 5869](https://tools.ietf.org/html/rfc5869).
/// # Parameters:
/// - `salt`:  Optional salt value
/// - `ikm`: Input keying material
/// - `info`: Optional context and application specific information (can be a zero-length string)
/// - `okm_out`: Destination buffer for the derivec key. The length of the derived key is implied by the length of `okm_out`
///
/// See [RFC](https://tools.ietf.org/html/rfc5869#section-2.2) for more information.
///
/// # Exceptions:
/// An exception will be thrown if:
/// - The specified length is less than 1
/// - The specified length is greater than 255 * hash_output_size_in_bytes
///
/// # Security:
/// Salts should always be generated using a CSPRNG. The `gen_rand_key` function
/// in `util` can be used for this. The recommended length for a salt is 16 bytes as a minimum.
/// HKDF is not suitable for password storage. Even though a salt value is optional, it is strongly
/// recommended to use one.
///
/// # Example:
/// ### Generating derived key:
/// ```
/// use orion::hazardous::hkdf;
/// ```
/// ### Verifying derived key:
/// ```
/// use orion::hazardous::hkdf;
/// ```

#[inline(always)]
pub fn extract(salt: &[u8], ikm: &[u8]) -> HLenArray {
    let mut prk = hmac::init(salt);
    prk.update(ikm);

    prk.finalize()
}

#[inline(always)]
pub fn expand(prk: &[u8], info: &[u8], okm_out: &mut [u8]) -> Result<(), UnknownCryptoError> {
    if okm_out.len() > 16320 {
        return Err(UnknownCryptoError);
    }
    if okm_out.len() < 1 {
        return Err(UnknownCryptoError);
    }

    let mut hmac = hmac::init(prk);

    for (idx, hlen_block) in okm_out.chunks_mut(HLEN).enumerate() {
        hmac.update(info);
        hmac.update(&[idx as u8 + 1_u8]);

        let block_len = hlen_block.len();
        //hlen_block.copy_from_slice(&hmac.finalize_with_opad(&opad, )[..block_len]);
        hmac.finalize_with_dst(&mut hlen_block[..block_len]);

        // Check if it's the last iteration, if yes don't process anything
        if block_len < HLEN {
            break;
        } else {
            hmac.reset();
            hmac.update(&hlen_block);
        }
    }

    Ok(())
}

/// Combine Extract and Expand to return a derived key.
pub fn derive_key(
    salt: &[u8],
    ikm: &[u8],
    info: &[u8],
    okm_out: &mut [u8],
) -> Result<(), UnknownCryptoError> {
    expand(&extract(salt, ikm), info, okm_out)
}

/// Verify a derived key by comparing one from the current struct fields to the derived key
/// passed to the function. Comparison is done in constant time. Both derived keys must be
/// of equal length.
pub fn verify(
    expected_dk: &[u8],
    salt: &[u8],
    ikm: &[u8],
    info: &[u8],
    okm_out: &mut [u8],
) -> Result<bool, ValidationCryptoError> {
    expand(&extract(salt, ikm), info, okm_out).unwrap();

    if util::compare_ct(&okm_out, expected_dk).is_err() {
        Err(ValidationCryptoError)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    extern crate hex;
    use self::hex::decode;
    use hazardous::hkdf::*;

    #[test]
    fn hkdf_maximum_length_512() {
        // Max allowed length here is 16320
        let mut okm_out = [0u8; 17000];
        let prk = extract("".as_bytes(), "".as_bytes());

        assert!(expand(&prk, "".as_bytes(), &mut okm_out).is_err());
    }

    #[test]
    fn hkdf_zero_length() {
        let mut okm_out = [0u8; 0];
        let prk = extract("".as_bytes(), "".as_bytes());

        assert!(expand(&prk, "".as_bytes(), &mut okm_out).is_err());
    }

    #[test]
    fn hkdf_verify_true() {
        let ikm = decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let salt = decode("000102030405060708090a0b0c").unwrap();
        let info = decode("f0f1f2f3f4f5f6f7f8f9").unwrap();
        let mut okm_out = [0u8; 42];

        let expected_okm = decode(
            "832390086cda71fb47625bb5ceb168e4c8e26a1a16ed34d9fc7fe92c1481579338da362cb8d9f925d7cb",
        ).unwrap();


        assert_eq!(
            verify(&expected_okm, &salt, &ikm, &info, &mut okm_out).unwrap(),
            true
        );
    }

    #[test]
    fn hkdf_verify_wrong_salt() {
        let salt = "salt".as_bytes();
        let ikm = decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let info = "".as_bytes();
        let mut okm_out = [0u8; 42];

        let expected_okm = decode(
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d\
             9d201395faa4b61a96c8",
        ).unwrap();

        assert!(
            verify(&expected_okm, salt, &ikm, info, &mut okm_out).is_err()
        );
    }

    #[test]
    fn hkdf_verify_wrong_ikm() {
        let salt = "".as_bytes();
        let ikm = decode("0b").unwrap();
        let info = "".as_bytes();
        let mut okm_out = [0u8; 42];

        let expected_okm = decode(
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d\
             9d201395faa4b61a96c8",
        ).unwrap();

        assert!(
            verify(&expected_okm, salt, &ikm, info, &mut okm_out).is_err()
        );
    }

    #[test]
    fn verify_diff_length() {
        let salt = "".as_bytes();
        let ikm = decode("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b").unwrap();
        let info = "".as_bytes();
        let mut okm_out = [0u8; 72];

        let expected_okm = decode(
            "8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d\
             9d201395faa4b61a96c8",
        ).unwrap();

        assert!(
            verify(&expected_okm, salt, &ikm, info, &mut okm_out).is_err()
        );
    }
}
