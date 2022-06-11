use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCryptError};

pub struct Encrypter {}

impl Encrypter {
    pub fn Encrypt(data: &mut Vec<u8>, secret: String) -> Vec<u8> {
        let mc = new_magic_crypt!(secret, 256);

        let base64 = mc.encrypt_bytes_to_bytes(data);
        base64
    }

    pub fn Decrypt(data: &[u8], secret: String) -> Result<Vec<u8>, MagicCryptError> {
        let mc = new_magic_crypt!(secret, 256);

        let base64 = mc.decrypt_bytes_to_bytes(data);
        base64
    }
}
