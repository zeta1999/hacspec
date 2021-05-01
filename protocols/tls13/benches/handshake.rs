#[macro_use]
extern crate criterion;
extern crate bertie;
extern crate rand;

use bertie::*;
use criterion::{BatchSize, Criterion};
use hacspec_dev::prelude::*;

fn load_hex(s: &str) -> Bytes {
    let s_no_ws: String = s.split_whitespace().collect();
    Bytes::from_hex(&s_no_ws)
}

fn name(alg: &Algorithms) -> &'static str {
    if alg.3 == NamedGroup::X25519 {
        "TLS_AES_128_GCM_SHA256_X25519"
    } else if alg.3 == NamedGroup::SECP256r1 {
        "TLS_AES_128_GCM_SHA256_P256"
    } else {
        " === ERROR: UNKNOWN CIPHER SUITE ==="
    }
}

fn bench(c: &mut Criterion) {
    const TLS_AES_128_GCM_SHA256_X25519: Algorithms = Algorithms(
        HashAlgorithm::SHA256,
        AEADAlgorithm::AES_128_GCM,
        SignatureScheme::ECDSA_SECP256r1_SHA256,
        NamedGroup::X25519,
        false,
        false,
    );
    const TLS_AES_128_GCM_SHA256_P256: Algorithms = Algorithms(
        HashAlgorithm::SHA256,
        AEADAlgorithm::AES_128_GCM,
        SignatureScheme::ECDSA_SECP256r1_SHA256,
        NamedGroup::SECP256r1,
        false,
        false,
    );
    const CIPHERSUITES: [Algorithms; 2] =
        [TLS_AES_128_GCM_SHA256_X25519, TLS_AES_128_GCM_SHA256_P256];

    const CLIENT_X25519_PRIV: &str = "49 af 42 ba 7f 79 94 85 2d 71 3e f2 78
    4b cb ca a7 91 1d e2 6a dc 56 42 cb 63 45 40 e7 ea 50 05";

    const SERVER_X25519_PRIV: &str = "b1 58 0e ea df 6d d5 89 b8 ef 4f 2d 56
    52 57 8c c8 10 e9 98 01 91 ec 8d 05 83 08 ce a2 16 a2 1e";

    const CLIENT_P256_PRIV: &str = "06 12 46 5c 89 a0 23 ab 17 85 5b 0a 6b ce
    bf d3 fe bb 53 ae f8 41 38 64 7b 53 52 e0 2c 10 c3 46";

    const SERVER_P256_PRIV: &str = "0a 0d 62 2a 47 e4 8f 6b c1 03 8a ce 43 8c
    6f 52 8a a0 0a d2 bd 1d a5 f1 3e e4 6b f5 f6 33 d7 1a";

    const ECDSA_P256_SHA256_CERT: [u8; 522] = [
        0x30, 0x82, 0x02, 0x06, 0x30, 0x82, 0x01, 0xAC, 0x02, 0x09, 0x00, 0xD1, 0xA2, 0xE4, 0xD5,
        0x78, 0x05, 0x08, 0x61, 0x30, 0x0A, 0x06, 0x08, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03,
        0x02, 0x30, 0x81, 0x8A, 0x31, 0x0B, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02,
        0x44, 0x45, 0x31, 0x0F, 0x30, 0x0D, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0C, 0x06, 0x42, 0x65,
        0x72, 0x6C, 0x69, 0x6E, 0x31, 0x0F, 0x30, 0x0D, 0x06, 0x03, 0x55, 0x04, 0x07, 0x0C, 0x06,
        0x42, 0x65, 0x72, 0x6C, 0x69, 0x6E, 0x31, 0x10, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x04, 0x0A,
        0x0C, 0x07, 0x68, 0x61, 0x63, 0x73, 0x70, 0x65, 0x63, 0x31, 0x0F, 0x30, 0x0D, 0x06, 0x03,
        0x55, 0x04, 0x0B, 0x0C, 0x06, 0x62, 0x65, 0x72, 0x74, 0x69, 0x65, 0x31, 0x17, 0x30, 0x15,
        0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x0E, 0x62, 0x65, 0x72, 0x74, 0x69, 0x65, 0x2E, 0x68,
        0x61, 0x63, 0x73, 0x70, 0x65, 0x63, 0x31, 0x1D, 0x30, 0x1B, 0x06, 0x09, 0x2A, 0x86, 0x48,
        0x86, 0xF7, 0x0D, 0x01, 0x09, 0x01, 0x16, 0x0E, 0x62, 0x65, 0x72, 0x74, 0x69, 0x65, 0x40,
        0x68, 0x61, 0x63, 0x73, 0x70, 0x65, 0x63, 0x30, 0x1E, 0x17, 0x0D, 0x32, 0x31, 0x30, 0x34,
        0x32, 0x39, 0x31, 0x31, 0x34, 0x37, 0x34, 0x35, 0x5A, 0x17, 0x0D, 0x33, 0x31, 0x30, 0x34,
        0x32, 0x37, 0x31, 0x31, 0x34, 0x37, 0x34, 0x35, 0x5A, 0x30, 0x81, 0x8A, 0x31, 0x0B, 0x30,
        0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x44, 0x45, 0x31, 0x0F, 0x30, 0x0D, 0x06,
        0x03, 0x55, 0x04, 0x08, 0x0C, 0x06, 0x42, 0x65, 0x72, 0x6C, 0x69, 0x6E, 0x31, 0x0F, 0x30,
        0x0D, 0x06, 0x03, 0x55, 0x04, 0x07, 0x0C, 0x06, 0x42, 0x65, 0x72, 0x6C, 0x69, 0x6E, 0x31,
        0x10, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x04, 0x0A, 0x0C, 0x07, 0x68, 0x61, 0x63, 0x73, 0x70,
        0x65, 0x63, 0x31, 0x0F, 0x30, 0x0D, 0x06, 0x03, 0x55, 0x04, 0x0B, 0x0C, 0x06, 0x62, 0x65,
        0x72, 0x74, 0x69, 0x65, 0x31, 0x17, 0x30, 0x15, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x0E,
        0x62, 0x65, 0x72, 0x74, 0x69, 0x65, 0x2E, 0x68, 0x61, 0x63, 0x73, 0x70, 0x65, 0x63, 0x31,
        0x1D, 0x30, 0x1B, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x01, 0x16,
        0x0E, 0x62, 0x65, 0x72, 0x74, 0x69, 0x65, 0x40, 0x68, 0x61, 0x63, 0x73, 0x70, 0x65, 0x63,
        0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01, 0x06, 0x08,
        0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0xD8, 0xE0, 0x74,
        0xF7, 0xCB, 0xEF, 0x19, 0xC7, 0x56, 0xA4, 0x52, 0x59, 0x0C, 0x02, 0x70, 0xCC, 0x9B, 0xFC,
        0x45, 0x8D, 0x73, 0x28, 0x39, 0x1D, 0x3B, 0xF5, 0x26, 0x17, 0x8B, 0x0D, 0x25, 0x04, 0x91,
        0xE8, 0xC8, 0x72, 0x22, 0x59, 0x9A, 0x2C, 0xBB, 0x26, 0x31, 0xB1, 0xCC, 0x6B, 0x6F, 0x5A,
        0x10, 0xD9, 0x7D, 0xD7, 0x86, 0x56, 0xFB, 0x89, 0x39, 0x9E, 0x0A, 0x91, 0x9F, 0x35, 0x81,
        0xE7, 0x30, 0x0A, 0x06, 0x08, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02, 0x03, 0x48,
        0x00, 0x30, 0x45, 0x02, 0x21, 0x00, 0xA1, 0x81, 0xB3, 0xD6, 0x8C, 0x9F, 0x62, 0x66, 0xC6,
        0xB7, 0x3F, 0x26, 0xE7, 0xFD, 0x88, 0xF9, 0x4B, 0xD8, 0x15, 0xD1, 0x45, 0xC7, 0x66, 0x69,
        0x40, 0xC2, 0x55, 0x21, 0x84, 0x9F, 0xE6, 0x8C, 0x02, 0x20, 0x10, 0x7E, 0xEF, 0xF3, 0x1D,
        0x58, 0x32, 0x6E, 0xF7, 0xCB, 0x0A, 0x47, 0xF2, 0xBA, 0xEB, 0xBC, 0xB7, 0x8F, 0x46, 0x56,
        0xF1, 0x5B, 0xCC, 0x2E, 0xD5, 0xB3, 0xC4, 0x0F, 0x5B, 0x22, 0xBD, 0x02,
    ];
    const ECDSA_P256_SHA256_KEY: [u8; 32] = [
        0xA6, 0xDE, 0x48, 0x21, 0x0E, 0x56, 0x12, 0xDD, 0x95, 0x3A, 0x91, 0x4E, 0x9F, 0x56, 0xC3,
        0xA2, 0xDB, 0x7A, 0x36, 0x20, 0x08, 0xE9, 0x52, 0xEE, 0xDB, 0xCE, 0xAC, 0x3B, 0x26, 0xF9,
        0x20, 0xBD,
    ];

    for &ciphersuite in CIPHERSUITES.iter() {
        c.bench_function(
            &format!("Handshake performance client: {}", name(&ciphersuite)),
            |b| {
                b.iter_batched(
                    || {
                        let cr = Random::from_public_slice(&random_byte_vec(Random::length()));
                        let x = match ciphersuite.3 {
                            NamedGroup::X25519 => load_hex(CLIENT_X25519_PRIV),
                            NamedGroup::SECP256r1 => load_hex(CLIENT_P256_PRIV),
                        };
                        let ent_c = Entropy::from_seq(&cr.concat(&x));
                        let sn = load_hex("6c 6f 63 61 6c 68 6f 73 74");
                        let sn_ = load_hex("6c 6f 63 61 6c 68 6f 73 74");
                        let sr = Random::from_public_slice(&random_byte_vec(Random::length()));
                        let y = match ciphersuite.3 {
                            NamedGroup::X25519 => load_hex(SERVER_X25519_PRIV),
                            NamedGroup::SECP256r1 => load_hex(SERVER_P256_PRIV),
                        };
                        let ent_s = Entropy::from_seq(&sr.concat(&y));

                        let db = ServerDB(
                            sn_,
                            Bytes::from_public_slice(&ECDSA_P256_SHA256_CERT),
                            SIGK::from_public_slice(&ECDSA_P256_SHA256_KEY),
                            None,
                        );

                        let (ch, _cstate, _) =
                            client_init(ciphersuite, &sn, None, None, ent_c.clone()).unwrap();
                        let (sh, sf, _sstate, _, _server_cipher) =
                            server_init(ciphersuite, db, &ch, ent_s).unwrap();
                        (sh, sf, sn, ent_c)
                    },
                    |(sh, sf, sn, ent_c)| {
                        let (_ch, cstate, _) =
                            client_init(ciphersuite, &sn, None, None, ent_c).unwrap();
                        let (cstate, _) = client_set_params(&sh, cstate).unwrap();
                        let (_cf, _cstate, _client_cipher) = client_finish(&sf, cstate).unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        c.bench_function(
            &format!("Handshake performance server: {}", name(&ciphersuite)),
            |b| {
                b.iter_batched(
                    || {
                        let cr = Random::from_public_slice(&random_byte_vec(Random::length()));
                        let x = match ciphersuite.3 {
                            NamedGroup::X25519 => load_hex(CLIENT_X25519_PRIV),
                            NamedGroup::SECP256r1 => load_hex(CLIENT_P256_PRIV),
                        };
                        let ent_c = Entropy::from_seq(&cr.concat(&x));
                        let sn = load_hex("6c 6f 63 61 6c 68 6f 73 74");
                        let sn_ = load_hex("6c 6f 63 61 6c 68 6f 73 74");
                        let sr = Random::from_public_slice(&random_byte_vec(Random::length()));
                        let y = match ciphersuite.3 {
                            NamedGroup::X25519 => load_hex(SERVER_X25519_PRIV),
                            NamedGroup::SECP256r1 => load_hex(SERVER_P256_PRIV),
                        };
                        let ent_s = Entropy::from_seq(&sr.concat(&y));

                        let db = ServerDB(
                            sn_.clone(),
                            Bytes::from_public_slice(&ECDSA_P256_SHA256_CERT),
                            SIGK::from_public_slice(&ECDSA_P256_SHA256_KEY),
                            None,
                        );

                        let (ch, cstate, _) =
                            client_init(ciphersuite, &sn, None, None, ent_c.clone()).unwrap();
                        let (sh, sf, _sstate, _, _server_cipher) =
                            server_init(ciphersuite, db, &ch, ent_s.clone()).unwrap();
                        let (cstate, _) = client_set_params(&sh, cstate).unwrap();
                        let (cf, _cstate, _client_cipher) = client_finish(&sf, cstate).unwrap();

                        let db = ServerDB(
                            sn_,
                            Bytes::from_public_slice(&ECDSA_P256_SHA256_CERT),
                            SIGK::from_public_slice(&ECDSA_P256_SHA256_KEY),
                            None,
                        );

                        (ch, cf, ent_s, db)
                    },
                    |(ch, cf, ent_s, db)| {
                        let (_sh, _sf, sstate, _, _server_cipher) =
                            server_init(ciphersuite, db, &ch, ent_s).unwrap();
                        let _sstate = server_finish(&cf, sstate).unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
