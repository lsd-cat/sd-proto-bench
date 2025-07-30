use wasm_bindgen::prelude::*;
use libcrux_ecdh as ecdh;
use hpke_rs::{self as hpke};
use hpke_rs::hpke_types::{AeadAlgorithm, KemAlgorithm, KdfAlgorithm};
use hpke_rs::Mode;
use hpke_rs_libcrux::HpkeLibcrux;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[wasm_bindgen]
pub struct KeyPair {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
}

#[wasm_bindgen]
impl KeyPair {
    #[wasm_bindgen(getter)]
    pub fn private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }
}

#[wasm_bindgen]
pub fn generate_x25519_keypair() -> KeyPair {
    let mut rng = StdRng::from_os_rng();
    let (sk, pk) = ecdh::key_gen(ecdh::Algorithm::X25519, &mut rng).unwrap();
    KeyPair {
        private_key: sk,
        public_key: pk,
    }
}

#[wasm_bindgen]
pub struct HpkeCiphertext {
    enc: Vec<u8>,
    ct: Vec<u8>,
}

#[wasm_bindgen]
impl HpkeCiphertext {
    #[wasm_bindgen(getter)]
    pub fn enc(&self) -> Vec<u8> {
        self.enc.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ct(&self) -> Vec<u8> {
        self.ct.clone()
    }
}

#[wasm_bindgen]
pub fn hpke_encrypt(pk_r: &[u8], msg: &[u8]) -> HpkeCiphertext {
    let mut hpke = hpke::Hpke::<HpkeLibcrux>::new(
        Mode::Base,
        KemAlgorithm::DhKem25519,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
    );

    let pk = hpke::HpkePublicKey::new(pk_r.to_vec());
    let info: &[u8] = b"bench";
    let aad: &[u8] = b"";
    let (enc, ct) = hpke
        .seal(&pk, info, aad, msg, None, None, None)
        .unwrap();
    HpkeCiphertext { enc, ct }
}
