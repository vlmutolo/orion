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

/// These are the different types used by the high-level interface. Meaning, eveything not in `hazardous`.
use errors::UnknownCryptoError;

construct_secret_key_variable_size! {
    /// A type to represent a secret key.
    ///
    /// As default it will randomly generate a `SecretKey` of 32 bytes.
    ///
    /// # Exceptions:
    /// An exception will be thrown if:
    /// - `slice` is empty.
    /// - The `OsRng` fails to initialize or read from its source.
    /// - `length` is 0.
    (SecretKey, 32)
}

construct_salt_variable_size! {
    /// A type to represent the `Salt` that PBKDF2 uses during key derivation.
    ///
    /// As default it will randomly generate a `Salt` of 64 bytes.
    ///
    /// # Exceptions:
    /// An exception will be thrown if:
    /// - `slice` is empty.
    /// - The `OsRng` fails to initialize or read from its source.
    /// - `length` is 0.
    (Salt, 64)
}

construct_tag! {
    /// A type to represent the `PasswordHash` that PBKDF2 returns when used for password hashing.
    ///
    /// A `PasswordHash`'s first 64 bytes are the salt used to hash the password, and the last 64
    /// bytes are the actual password hash.
    ///
    /// # Exceptions:
    /// An exception will be thrown if:
    /// - `slice` is not 128 bytes.
    (PasswordHash, 128)
}