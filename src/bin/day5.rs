use md5::{Md5, Digest};
use hex;
fn compute_hash(s: &str) -> [u8; 16] {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    hasher.finalize().into()
}

fn get_char(hex_hash: &str) -> Option<char> {
   match &hex_hash[0..5] {
       "00000" => hex_hash
           .chars()
           .nth(5),
       _ => None
   }
}

fn get_char_p2(hex_hash: &str) -> Option<(usize, char)> {

   match &hex_hash[0..5] {
       "00000" => {
           let pos = hex_hash
               .chars()
               .nth(5).unwrap()
               .to_string()
               .parse();
           match pos {
               Ok(pos) if pos >= 0 && pos <= 7 => {
                   let ch = hex_hash.chars().nth(6).unwrap();
                   Some((pos, ch))
               },
               _ => None
           }
       },
       _ => None
   }

}
fn extract_password_p1(s: &str) -> String {
    let mut i = 0;
    let mut cur;
    let mut passwd = String::new(); 
    loop {
        cur = s.to_string();
        cur.push_str(&format!("{}", i));
        match get_char(&hex::encode(&compute_hash(&cur))) {
            Some(c) => {
                passwd.push_str(&c.to_string());
                if passwd.len() == 8 {
                    return passwd
                }
            },
            _ => {} 
        }
        i += 1;
    }
    unreachable!("should never return without string")
}
fn extract_password_p2(s: &str) -> String {
    let mut i = 0;
    let mut cur;
    let mut passwd = "00000000".to_string().into_bytes();
    let mut found : [bool; 8] = [false; 8];
    loop {
        cur = s.to_string();
        cur.push_str(&format!("{}", i));
        match get_char_p2(&hex::encode(&compute_hash(&cur))) {
            Some((pos, c)) => {
                if !found[pos] { 
                    passwd[pos] = c as u8;
                    found[pos] = true;
                }
                if found.iter().all(|&v| v) {
                    return String::from_utf8_lossy(&passwd).to_string();
                }
            },
            _ => {} 
        }
        i += 1;
    }
    unreachable!("should never return without string")
}
fn main() {
    let pw = extract_password_p1("cxdnnyjw");
    println!("p1 password for cxdnnyjw:  {}", pw);

    let pw = extract_password_p2("cxdnnyjw");
    println!("p2 password for cxdnnyjw:  {}", pw);
}
