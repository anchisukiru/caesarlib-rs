// Copyright 2017 ______.  All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

extern crate caesarlib;
use caesarlib::{encipher, decipher, rdm_encipher};


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ciphered_text_can_be_deciphered() {
        let enciphered = encipher(13, "FooBar");
        assert_eq!("FooBar", decipher(13, &enciphered));
    }
    #[test]
    fn ciphered_text_is_different_with_regards_to_offset() {
        assert_ne!(encipher(13, "FooBar"), encipher(14, "FooBar"));
    }
    #[test]
    fn ciphered_text_can_contains_unknown_chars() {
        let enciphered = encipher(13, "Foo:bar.");
        assert_eq!("Foo:bar.", decipher(13, &enciphered));
    }
    #[test]
    fn encipher_offset_cannot_be_negative() {
        let enciphered = encipher(-13, "Foo:bar.");
        assert_ne!("Foo:bar.", &enciphered);
    }
    #[test]
    fn decipher_offset_cannot_be_negative() {
        let deciphered = encipher(-13, "Foo:bar.");
        assert_ne!("Foo:bar.", &deciphered);
    }
    #[test]
    fn random_enciphered_text_can_be_deciphered() {
        let test_word = "FooBar";
        let (offset, enciphered) = rdm_encipher(test_word);
        assert_eq!(test_word, decipher(offset, &enciphered));
    }
}
