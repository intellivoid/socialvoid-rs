use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1;
use pad::{Alignment, PadStr};

use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the challenge_answer using the SessionEstablished object
pub fn answer_challenge(client_private_hash: String, challenge: String) -> String {
    let mut hasher = sha1::Sha1::new();
    let totp_code = totp(challenge);
    //hashlib.sha1("{0}{1}".format(totp_code, client_private_hash).encode()).hexdigest()
    hasher.input(format!("{}{}", totp_code, client_private_hash).as_bytes());
    hasher.result_str()
}

fn totp(key: String) -> String {
    let time_step = 30;
    let now = SystemTime::now();
    let counter = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        / time_step;
    let digits = 6;
    hotp(key, counter, digits)
}
fn hotp(key: String, counter: u64, digits: u8) -> String {
    let key = base32::decode(
        base32::Alphabet::RFC4648 { padding: true },
        &format!("{}{}", key.to_uppercase(), "=".repeat((8 - key.len()) % 8)),
    )
    .expect("Couldn't decode base32");
    let mut hmac = Hmac::new(sha1::Sha1::new(), &key);
    let mut msg = vec![];
    msg.write_u64::<BigEndian>(counter).unwrap();
    hmac.input(&msg);
    let mut result = vec![];
    hmac.raw_result(&mut result);
    let offset = result.last().unwrap() & 0x0f;
    let mut rdr = Cursor::new(&result[offset as usize..(offset + 4) as usize]);
    let binary = rdr.read_u32::<BigEndian>().unwrap() & 0x7fffffff;

    //OTP =
    binary
        .to_string()
        .pad(digits as usize, '0', Alignment::Right, true)
}
