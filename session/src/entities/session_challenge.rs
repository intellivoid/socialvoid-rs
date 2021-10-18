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
        &format!(
            "{}{}",
            key.to_uppercase(),
            "=".repeat(((8_i8 - key.len() as i8) % 8) as usize)
        ),
    )
    .expect("Couldn't decode base32");

    let mut hmac = Hmac::new(sha1::Sha1::new(), &key);
    let mut msg = vec![];
    msg.write_u64::<BigEndian>(counter).unwrap();
    hmac.input(&msg);
    let result = hmac.result().code().to_vec();
    let offset = result.last().unwrap() & 0x0f;
    let mut rdr = Cursor::new(&result[offset as usize..(offset + 4) as usize]);
    let binary = rdr.read_u32::<BigEndian>().unwrap() & 0x7fffffff;

    let binstr = binary.to_string();
    let start_i = binstr.len() as i32 - 6;
    let end_i = binstr.len() - 1;
    if start_i > 0 {
        return binstr[start_i as usize..=end_i as usize].to_string();
    }
    //OTP =
    binstr[start_i as usize..=end_i as usize].pad(digits as usize, '0', Alignment::Right, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_the_correct_number_of_hotp_digits() {
        let digits = 6;
        let counter = 23;
        let otp_string = hotp(String::from("ff"), counter, digits);
        println!("{}", otp_string);
        assert_eq!(otp_string.len(), 6);
    }

    #[tokio::test]
    async fn it_should_calculate_the_correct_answer() {
        //This test creates a session and calculates the challenge answer
        // using the python script in the standard and the current implementation and
        // compares both
        use crate::ClientInfo;
        use crate::SessionHolder;
        use std::process::Command;

        let client_info = ClientInfo::generate();
        let private_hash = client_info.private_hash.clone();
        let mut session = SessionHolder::new(client_info);
        let client = socialvoid_rawclient::new();
        session
            .create(&client)
            .await
            .expect("Couldn't create the session");
        let established = session.established.unwrap();
        let challenge_answer =
            answer_challenge(private_hash.clone(), established.challenge.clone());

        let output = Command::new("python3")
            .arg("test-hotp.py")
            .arg(&private_hash)
            .arg(&established.challenge)
            .output()
            .expect("Couldn't run python script");
        println!(
            "output: {}, err: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(String::from_utf8_lossy(&output.stdout), challenge_answer);
    }
}
