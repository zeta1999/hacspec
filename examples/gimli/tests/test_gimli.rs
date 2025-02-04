use hacspec_dev::prelude::*;
use hacspec_gimli::*;
use hacspec_lib::prelude::*;

// All KAT tests are generated with the hacspec python code from
// https://csrc.nist.gov/CSRC/media/Projects/lightweight-cryptography/documents/round-2/submissions-rnd2/gimli.zip

#[test]
fn kat_gimli() {
    let state = State::from_public_slice(&[1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let expected = State::from_public_slice(&[
        0x2be627ff, 0x7458fba0, 0xee1215e3, 0xcedd078, 0xd040ca63, 0xdc423706, 0x95e37aaf,
        0xcaf8c20, 0x47a98cd5, 0xa0024ee2, 0x946e3cb0, 0xc90bbffd,
    ]);
    
    let out = gimli(state);
    assert_secret_array_eq!(expected, out, U32);
}

#[test]
fn self_gimli_aead() {
    let key = Key::from_public_slice(&random_byte_vec(Key::length()));
    let nonce = Nonce::from_public_slice(&random_byte_vec(Nonce::length()));
    let msg = ByteSeq::from_public_slice(&random_byte_vec(65));
    let ad = ByteSeq::from_public_slice(&random_byte_vec(123));

    let (ct, tag) = gimli_aead_encrypt(&msg, &ad, nonce, key);
    let msg_out = gimli_aead_decrypt(&ct, &ad, tag, nonce, key);

    assert_eq!(msg, msg_out);
}

#[test]
fn kat_gimli_aead() {
    let key = Key::from_public_slice(&[
        25, 166, 231, 234, 181, 41, 122, 159, 233, 86, 201, 207, 224, 221, 169, 35, 153, 170, 188,
        121, 20, 239, 204, 1, 226, 147, 82, 163, 107, 96, 0, 234,
    ]);
    let nonce = Nonce::from_public_slice(&[
        82, 210, 223, 174, 249, 234, 56, 205, 101, 144, 11, 142, 133, 145, 210, 41,
    ]);
    let msg = ByteSeq::from_public_slice(&[
        56, 143, 145, 12, 68, 248, 22, 122, 159, 136, 55, 241, 117, 193, 204, 169, 133, 170, 219,
        39, 234, 120, 99, 152, 23, 255, 2, 107, 23, 214, 203, 48, 1, 98, 126, 187, 43, 38, 228,
        185, 160, 144, 53, 232, 158, 1, 192, 255, 117, 51, 246, 133, 146, 176, 113, 188, 50, 227,
        48, 217, 17, 66, 39, 198, 63, 119, 215, 241, 131, 159, 96, 36, 53, 184, 112, 126, 2, 191,
        180, 238, 24, 25, 14, 195, 77, 78, 192, 246, 39, 99, 136, 212, 219, 225, 144, 244, 206, 90,
        111, 189, 188, 201, 220, 91, 27, 43, 60, 124, 3, 65, 43, 251, 54, 186, 162, 199, 86, 148,
        81, 124, 185, 245, 185,
    ]);
    let ad = ByteSeq::from_public_slice(&[
        209, 73, 234, 150, 81, 104, 159, 62, 173, 227, 108, 174, 208, 45, 184, 125, 214, 100, 222,
        3, 211, 70, 210, 54, 31, 151, 217, 108, 27, 157, 62, 209, 5, 56, 184, 20, 35, 250, 189,
        171, 177, 49, 253, 164, 114, 108, 235, 238, 83, 125, 253, 38, 199, 10, 12, 226, 198, 164,
        116, 126, 127, 137, 32, 248, 153, 131, 129, 201, 194, 62, 76, 228, 191, 60, 25, 250, 78,
        175, 29, 118, 3, 62, 232, 168, 62, 30, 4, 93, 88, 183, 139, 195, 152, 154, 168, 0, 218,
        196, 95, 249,
    ]);

    let expected_ct = ByteSeq::from_public_slice(&[
        0x22, 0x8c, 0x3a, 0xed, 0x83, 0x8c, 0xe4, 0x9b, 0x87, 0x9a, 0x4f, 0x13, 0xe8, 0x39, 0x33,
        0x56, 0x1, 0xda, 0xe, 0xa5, 0xaa, 0x8b, 0x29, 0xea, 0xd1, 0xe5, 0x15, 0xd1, 0x76, 0x50,
        0x9c, 0x89, 0x75, 0x28, 0xae, 0x4c, 0x6d, 0xbb, 0xed, 0xd9, 0x5c, 0xdb, 0x84, 0x3f, 0x3b,
        0x74, 0x9e, 0x36, 0x68, 0xd2, 0x83, 0xb5, 0x6e, 0x4c, 0x86, 0x1, 0xf1, 0xbc, 0xbd, 0x9d,
        0xf5, 0x50, 0x9c, 0xe1, 0xbf, 0x9a, 0xff, 0xfa, 0x44, 0x59, 0xed, 0xd6, 0x26, 0xf0, 0x20,
        0x5e, 0xc9, 0x19, 0xc, 0x7d, 0x48, 0x8f, 0x71, 0xf5, 0xfb, 0x6b, 0x23, 0x63, 0x6a, 0x73,
        0x7, 0xc6, 0xb5, 0x1c, 0xef, 0x21, 0x8b, 0x3e, 0x68, 0x14, 0x14, 0xee, 0xef, 0x3, 0x9f,
        0xe3, 0xfd, 0x63, 0x8e, 0x12, 0x69, 0x78, 0x84, 0xb3, 0xf4, 0x90, 0x0, 0x74, 0x6d, 0xc9,
        0xd4, 0xc9, 0xe5,
    ]);
    let expected_tag = Tag::from_public_slice(&[
        0x1d, 0x67, 0xd9, 0x60, 0x57, 0x83, 0xf1, 0x4c, 0x55, 0xd7, 0x8, 0x96, 0x68, 0x3d, 0x78,
        0x5d,
    ]);

    let (ct, tag) = gimli_aead_encrypt(&msg, &ad, nonce, key);
    assert_eq!(expected_ct, ct);
    assert_secret_array_eq!(expected_tag, tag, U8);

    let msg_out = gimli_aead_decrypt(&ct, &ad, tag, nonce, key);
    assert_eq!(msg, msg_out);
}
