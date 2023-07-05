fn test<T: serde::de::DeserializeOwned + std::fmt::Debug>(data: &[u8]) {
    let result = ctap_types::serde::cbor_deserialize::<T>(data);
    assert!(result.is_ok(), "{:?}", result);
}

#[test]
fn test_rp_id() {
    test::<ctap_types::String<64>>(b"kexample.com");
}

#[test]
fn test_client_data_hash() {
    test::<ctap_types::Bytes<32>>(
        b"X \xac5\xb4\xc4S\xba\xe8\xd1\x8d\"a[\xdd\x17L\xa4\xcf\x1d8\xc0\x9f!\xea\xd9\xc1\'\x8c/F\xd8\x1c\x03"
    );
}

#[test]
fn test_allow_list() {
    test::<ctap_types::ctap2::get_assertion::AllowList>(
        b"\x81\xa2bidX\xdd\xa3\x00X\xb9\xd5p\xe7\x13>YCz\xf8\xe2{\x08A\x08\xae!@\x1c\xa4XS\x8a\xd1\x87\x1a\x97>\xa9\xf0\x95\xcc\xf4\x87\xb2\xe7\xec\xa0O`_\xe3\xc4\xc8XF\n\xb1}\x0c\xc0\\IA4\xb2`\x94\xa7\xc0[\xe2g\xb0\xe9\xc5\xb5\xd5\xf2\xfe\xb6G\x00\x8ec$\x87\xe9\x026V\xfdz\x93\x18\xdb\xc4\xa2\x13~\xdcb\t\x95\xf5\x94\x8b\xc09fT\xcae\xb1\x0cX+\x87x=<%d\xfb\x14\xf90\x17z\x99\xd0vCn\x9aI\xe8N\x06\xf2raH\xe9\x8c\x86F\x8e\xc0[\xc9\xcb\x9b\x00E$ \xe1\x0cK4\x17A\xce\xc0\x04\x87\x8d\x03=\xf4\xceKbM\x011N\xbc\xa6+\xc5p\xe3\xb5:\x9a\xe7\x13\xc7\x1c\x9fHm\xab\x90\x01L\xb83\xf0r -\xe7\xeaoj`^\x02PQ\x01~\x7f\x9c'\x12Y\x07\x9a\x85I\x961}\xe1dtypejpublic-key"
    );
}

#[test]
fn test_extensions() {
    test::<ctap_types::ctap2::get_assertion::ExtensionsInput>(
        b"\xa1khmac-secret\xa4\x01\xa5\x01\x02\x038\x18 \x01!X \xae\x97\x87\xc5C,\x14\x18\xa4\xba(\\2\xc9\x1f\xf84\xd7\xa7\xaeM\xc4V\x13\x9b\x9b\x96\xcd\xa2\"\xa8\xec\"X \x0c\x8d\xc0\xeaB\x80\xca\x0ff\x91\xdal\xb5a7@\xc6\xe2\x13\xa5\x08\" \xce\x94\x83\"\xfd}\x1c\xbdM\x02X \xb9.\xb6\xaa\xdcS6;r\'q\x93j\xb5~3\x1eN\xa1\xcc%\x0f\x8dVV\n\x87o\t\xc0\xb1\xcb\x03PaU/RA\xb9\x1a\x935\x8d<\xfd8\xabXs\x04\x01"
    );
}

#[test]
fn test_extensions_hmac_secret_input() {
    test::<ctap_types::ctap2::get_assertion::HmacSecretInput>(
        b"\xa4\x01\xa5\x01\x02\x038\x18 \x01!X \xae\x97\x87\xc5C,\x14\x18\xa4\xba(\\2\xc9\x1f\xf84\xd7\xa7\xaeM\xc4V\x13\x9b\x9b\x96\xcd\xa2\"\xa8\xec\"X \x0c\x8d\xc0\xeaB\x80\xca\x0ff\x91\xdal\xb5a7@\xc6\xe2\x13\xa5\x08\" \xce\x94\x83\"\xfd}\x1c\xbdM\x02X \xb9.\xb6\xaa\xdcS6;r\'q\x93j\xb5~3\x1eN\xa1\xcc%\x0f\x8dVV\n\x87o\t\xc0\xb1\xcb\x03PaU/RA\xb9\x1a\x935\x8d<\xfd8\xabXs\x04\x01"
    );
}

#[test]
fn test_extensions_hmac_secret_input_key_agreement() {
    test::<ctap_types::cose::EcdhEsHkdf256PublicKey>(
        b"\xa5\x01\x02\x038\x18 \x01!X \xae\x97\x87\xc5C,\x14\x18\xa4\xba(\\2\xc9\x1f\xf84\xd7\xa7\xaeM\xc4V\x13\x9b\x9b\x96\xcd\xa2\"\xa8\xec\"X \x0c\x8d\xc0\xeaB\x80\xca\x0ff\x91\xdal\xb5a7@\xc6\xe2\x13\xa5\x08\" \xce\x94\x83\"\xfd}\x1c\xbdM"
    );
}

#[test]
fn test_extensions_hmac_secret_salt_enc() {
    test::<ctap_types::Bytes<64>>(
        b"X \xb9.\xb6\xaa\xdcS6;r'q\x93j\xb5~3\x1eN\xa1\xcc%\x0f\x8dVV\n\x87o\t\xc0\xb1\xcb",
    );
}

#[test]
fn test_extensions_hmac_secret_salt_auth() {
    test::<ctap_types::Bytes<16>>(b"PaU/RA\xb9\x1a\x935\x8d<\xfd8\xabXs");
}