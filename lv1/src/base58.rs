use std::str;
const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(input_bytes: &[u8]) -> String {
    // Izbroji sve bajtove koji su nula na pocetku
    let zero_counter = input_bytes.iter().take_while(|&&byte| byte == 0).count();
    let mut b58_encoding = Vec::new();
    let mut b58_bytes = Vec::new();

    // Za svaki bajt u ulaznim bajtovima, preskoci prve nule
    for byte in input_bytes.iter().skip(zero_counter) {
        // Postavi carry na vrijednost bajta
        let mut carry = *byte as usize;
        /* Za svaki bajt koji je vec prebacen u b58 zapis:
        Shiftamo vrijednost za jedan bajt u lijevo(isto kao da mnozimo s 256) i dodajemo ga u carry.
        Prepisujemo vrijednost bajta u b58 zapisu(bajtovi postavljeni u prijasnjim iteracijama)
        s ostatkom pri dijeljenju s 58 i dijelimo carry s 58.
        */
        for value in b58_bytes.iter_mut() {
            carry += *value << 8;
            *value = carry % 58;
            carry /= 58;
        }
        // Ako je nesto ostalo u carryu nadodajemo ostatak pri dijeljenu s 58 na pocetak polja dok carry ne dode do nule
        while carry > 0 {
            b58_bytes.push(carry % 58);
            carry /= 58;
        }
    }
    // Okrecemo polje jer se MSB nalazi na kraju
    b58_bytes.reverse();
    // Mapiramo svaki broj iz polja u odgovarajucu base58 reprezentaciju
    for &byte in b58_bytes.iter() {
        b58_encoding.push(BASE58_ALPHABET[byte] as char);
    }

    let mut result = String::new();
    // Dodajemo 1 za svaki null byte s pocetka
    for _ in 0..zero_counter {
        result.push('1');
    }
    // Nadodajemo mapirane znakove nakon jedinica
    result.extend(b58_encoding);
    result
}

pub fn decode_base58(input_str: &str) -> Vec<u8> {
    let mut carry = 0u128;

    // Za svaki ulazni znak
    for (_index, c) in input_str.chars().enumerate() {
        // Pronalazimo indeks znaka u BASE58_ALPHABET
        let position = BASE58_ALPHABET
            .iter()
            .position(|&b| b == c as u8)
            .expect("Invalid character in input string");
        // Mnozimo prijasnju vrijednost s 58 i dodajemo vrijednost pronadenog znaka
        carry = carry * 58 + position as u128;
    }

    let mut bytes = Vec::new();
    /* Dok je ukupna vrijednost veca od 0 dodajemo ostatak pri dijeljenju s 256(1 bajt)
    na pocetak niza. Carry postaje vrijednost dobivena dijeljenjem s 256.
    */
    while carry > 0 {
        bytes.insert(0, (carry % 256) as u8);
        carry /= 256;
    }

    bytes
}

#[cfg(test)]
mod base58_tests {
    use crate::base58::{decode_base58, encode_base58};

    #[test]
    fn test_encode() {
        assert_eq!(encode_base58(b"FERIT"), "8vpytW7");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode_base58("8vpytW7"), b"FERIT");
    }
}
