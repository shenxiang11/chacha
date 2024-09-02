mod cli;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit};
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::consts::U12;
use clap::Parser;
use crate::cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Encrypt(args) => {
            let key = Key::from_slice(args.key.as_ref());
            let cipher = ChaCha20Poly1305::new(key);
            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
            let ciphertext = cipher.encrypt(&nonce, args.text.as_bytes()).expect("encryption failure");

            let mut combined = Vec::new();
            combined.extend_from_slice(&nonce); // 添加 nonce
            combined.extend_from_slice(&ciphertext); // 添加密文

            println!("Combined (nonce + ciphertext): {:?}", BASE64_STANDARD.encode(combined));
        }
        Command::Decrypt(args) => {
            let key = Key::from_slice(args.key.as_ref());
            let cipher = ChaCha20Poly1305::new(key);

            let combined = BASE64_STANDARD.decode(args.text).expect("base64 decode failure");

            // 另一个程序接收到 combined 后的解密过程
            let (received_nonce, received_ciphertext) = combined.split_at(12); // 分离前 12 字节的 nonce

            // 从接收到的 nonce 字节中创建 Nonce 对象
            let nonce = GenericArray::<u8, U12>::from_slice(received_nonce);
            let decrypted_plaintext = cipher
                .decrypt(nonce, received_ciphertext)
                .expect("decryption failure!");


            println!("result: {:?}", String::from_utf8_lossy(&decrypted_plaintext));
        }
    }

    Ok(())
}
