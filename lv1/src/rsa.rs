use base64::prelude::*;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pkcs8::LineEnding,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};
// Funkcija za nasumicno generiranje kljuceva
pub fn generate_keys() -> (String, String) {
    // Lazy inicijalizacija sistemskog random generatora
    let mut rng = rand::thread_rng();
    let bits = 2048;
    // Generiraj RSA private key(RSA algoritam se koristi za enkripciju poruka javnim kljucem i citanje privatnim)
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    // Iz privatnog kljuca napravi javni
    let pub_key = RsaPublicKey::from(&priv_key);

    // Funkcija vraca oba kljuca
    (
        priv_key
            .to_pkcs1_pem(LineEnding::default())
            .unwrap()
            .to_string(),
        pub_key
            .to_pkcs1_pem(LineEnding::default())
            .unwrap()
            .to_string(),
    )
}
// Funckija za potpisivanje poruke
pub fn sign_messge(message: &str, public_key_string: &str) -> String {
    let mut rng = rand::thread_rng();
    // Prebaci string public keya u struktruru RsaPublicKey
    let pub_key = RsaPublicKey::from_pkcs1_pem(public_key_string).unwrap();

    // Enkriptiraj poruku sa javnim kkjucem, enkodiraj ju u base_64 i vrati iz funckije
    BASE64_STANDARD.encode(
        pub_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes())
            .unwrap(),
    )
}

pub fn decode_messge(message: &str, private_key_string: &str) -> String {
    // Kreiraj strukturu RsaPrivateKey iz private key stringa
    let priv_key = RsaPrivateKey::from_pkcs1_pem(private_key_string).unwrap();

    // Dekriptiraj poruku privatnim kljucem, ako je neispravan program zaustavlja s radom i vraca pogresku.
    // Ako je ispravan dekpritira i dekodira s base_64 te vraca string poruke
    String::from_utf8(
        priv_key
            .decrypt(Pkcs1v15Encrypt, &BASE64_STANDARD.decode(message).unwrap())
            .expect("Wrong signature"),
    )
    .unwrap()
}

#[cfg(test)]
mod rsa_tests {
    use crate::rsa::{decode_messge, generate_keys, sign_messge};

    #[test]
    fn test_all() {
        let (priv_key, pub_key) = generate_keys();
        let signed_message = sign_messge("This is a test message", &pub_key);
        let decoded_message = decode_messge(&signed_message, &priv_key);
        assert_eq!(decoded_message, "This is a test message");
    }
}
