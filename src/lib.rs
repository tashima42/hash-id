#[allow(unused_imports)]
#[allow(non_snake_case)]
mod algorithms;

use core::str;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use algorithms::*;

pub fn run(config: Config) {
    let hash_possible_algorithms = get_possible_algorithms(config);

    for algorithm in hash_possible_algorithms.iter() {
        println!("Hash: {}", algorithm.hash);
        if algorithm.possible_algorithms.len() < 1 {
            println!("  not found");
        } else {
            for alg in algorithm.possible_algorithms.iter() {
                println!("  [+] {}", alg);
            }
        };
        println!("------------------------------------------");
    }
}

pub struct Config {
    pub hash: String,
    pub file: String,
}

impl Config {
    pub fn new(hash: String, file: String) -> Config {
        Config { hash, file }
    }
}

struct PossibleAlgorithms {
    pub hash: String,
    pub possible_algorithms: Vec<String>,
}

impl PossibleAlgorithms {
    pub fn new(hash: String, possible_algorithms: Vec<String>) -> PossibleAlgorithms {
        PossibleAlgorithms {
            hash,
            possible_algorithms,
        }
    }
}

fn get_possible_algorithms(config: Config) -> Vec<PossibleAlgorithms> {
    let mut possible_algorithms = vec![];

    if config.hash.len() > 0 {
        let detected_algorithms = detect_algorithms(&config.hash);
        possible_algorithms.push(PossibleAlgorithms::new(
            config.hash.clone(),
            detected_algorithms,
        ));
    }

    if config.file.len() > 0 {
        let file_hashes = read_file_lines(config.file).unwrap();
        for hash in file_hashes.iter() {
            let detected_algorithms = detect_algorithms(hash);
            possible_algorithms.push(PossibleAlgorithms::new(
                hash.to_string(),
                detected_algorithms,
            ));
        }
    }

    possible_algorithms
}

fn detect_algorithms(hash: &String) -> Vec<String> {
    let algorithms_map: HashMap<u32, String> = create_algorithms_map();
    let match_algorithms = run_algorithms_test(&hash);
    let mut algorithms_name = vec![];

    for algorithm in match_algorithms.iter() {
        algorithms_name.push(algorithms_map.get(algorithm).unwrap().to_string());
    }
    algorithms_name
}

fn read_file_lines(file: String) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(file).unwrap();

    let mut lines: Vec<String> = vec![];

    for line in content.lines() {
        lines.push(line.to_string());
    }

    Ok(lines)
}

fn run_algorithms_test(hash: &str) -> Vec<u32> {
    let mut algorithms: Vec<u32> = vec![];

    if CRC16(hash) {
        algorithms.push(101020);
    }
    if CRC16CCITT(hash) {
        algorithms.push(101040);
    }
    if FCS16(hash) {
        algorithms.push(101060);
    }
    if ADLER32(hash) {
        algorithms.push(102020);
    }
    if CRC32(hash) {
        algorithms.push(102040);
    }
    if CRC32B(hash) {
        algorithms.push(102060);
    }
    if XOR32(hash) {
        algorithms.push(102080);
    }
    if GHash325(hash) {
        algorithms.push(103020);
    }
    if GHash323(hash) {
        algorithms.push(103040);
    }
    if DESUnix(hash) {
        algorithms.push(104020);
    }
    if MySQL(hash) {
        algorithms.push(105020);
    }
    if MD5Middle(hash) {
        algorithms.push(105040);
    }
    if MD5Half(hash) {
        algorithms.push(105060);
    }
    if MD5(hash) {
        algorithms.push(106020);
    }
    if DomainCachedCredentials(hash) {
        algorithms.push(106025);
    }
    if RAdminv2x(hash) {
        algorithms.push(106027);
    }
    if NTLM(hash) {
        algorithms.push(106029);
    }
    if MD4(hash) {
        algorithms.push(106040);
    }
    if MD2(hash) {
        algorithms.push(106060);
    }
    if MD5HMAC(hash) {
        algorithms.push(106080);
    }
    if MD4HMAC(hash) {
        algorithms.push(106100);
    }
    if MD2HMAC(hash) {
        algorithms.push(106120);
    }
    if MD5HMACWordpress(hash) {
        algorithms.push(106140);
    }
    if Haval128(hash) {
        algorithms.push(106160);
    }
    if Haval128HMAC(hash) {
        algorithms.push(106165);
    }
    if RipeMD128(hash) {
        algorithms.push(106180);
    }
    if RipeMD128HMAC(hash) {
        algorithms.push(106185);
    }
    if SNEFRU128(hash) {
        algorithms.push(106200);
    }
    if SNEFRU128HMAC(hash) {
        algorithms.push(106205);
    }
    if Tiger128(hash) {
        algorithms.push(106220);
    }
    if Tiger128HMAC(hash) {
        algorithms.push(106225);
    }
    if md5passsalt(hash) {
        algorithms.push(106240);
    }
    if md5saltmd5pass(hash) {
        algorithms.push(106260);
    }
    if md5saltpass(hash) {
        algorithms.push(106280);
    }
    if md5saltpasssalt(hash) {
        algorithms.push(106300);
    }
    if md5saltpassusername(hash) {
        algorithms.push(106320);
    }
    if md5saltmd5pass(hash) {
        algorithms.push(106340);
    }
    if md5saltmd5passsalt(hash) {
        algorithms.push(106360);
    }
    if md5saltmd5passsalt1(hash) {
        algorithms.push(106380);
    }
    if md5saltmd5saltpass2(hash) {
        algorithms.push(106400);
    }
    if md5saltmd5md5passsalt(hash) {
        algorithms.push(106420);
    }
    if md5username0pass(hash) {
        algorithms.push(106440);
    }
    if md5usernameLFpass(hash) {
        algorithms.push(106460);
    }
    if md5usernamemd5passsalt(hash) {
        algorithms.push(106480);
    }
    if md5md5pass(hash) {
        algorithms.push(106500);
    }
    if md5md5passsalt(hash) {
        algorithms.push(106520);
    }
    if md5md5passmd5salt(hash) {
        algorithms.push(106540);
    }
    if md5md5saltpass(hash) {
        algorithms.push(106560);
    }
    if md5md5saltmd5pass(hash) {
        algorithms.push(106580);
    }
    if md5md5usernamepasssalt(hash) {
        algorithms.push(106600);
    }
    if md5md5md5pass(hash) {
        algorithms.push(106620);
    }
    if md5md5md5md5pass(hash) {
        algorithms.push(106640);
    }
    if md5md5md5md5md5pass(hash) {
        algorithms.push(106660);
    }
    if md5sha1pass(hash) {
        algorithms.push(106680);
    }
    if md5sha1md5pass(hash) {
        algorithms.push(106700);
    }
    if md5sha1md5sha1pass(hash) {
        algorithms.push(106720);
    }
    if md5strtouppermd5pass(hash) {
        algorithms.push(106740);
    }
    if MD5Wordpress(hash) {
        algorithms.push(107020);
    }
    if MD5phpBB3(hash) {
        algorithms.push(107040);
    }
    if MD5Unix(hash) {
        algorithms.push(107060);
    }
    if LineageIIC4(hash) {
        algorithms.push(107080);
    }
    if MD5APR(hash) {
        algorithms.push(108020);
    }
    if SHA1(hash) {
        algorithms.push(109020);
    }
    if MySQL5(hash) {
        algorithms.push(109040);
    }
    if MySQL160bit(hash) {
        algorithms.push(109060);
    }
    if Tiger160(hash) {
        algorithms.push(109080);
    }
    if Haval160(hash) {
        algorithms.push(109100);
    }
    if RipeMD160(hash) {
        algorithms.push(109120);
    }
    if SHA1HMAC(hash) {
        algorithms.push(109140);
    }
    if Tiger160HMAC(hash) {
        algorithms.push(109160);
    }
    if RipeMD160HMAC(hash) {
        algorithms.push(109180);
    }
    if Haval160HMAC(hash) {
        algorithms.push(109200);
    }
    if SHA1MaNGOS(hash) {
        algorithms.push(109220);
    }
    if SHA1MaNGOS2(hash) {
        algorithms.push(109240);
    }
    if sha1passsalt(hash) {
        algorithms.push(109260);
    }
    if sha1saltpass(hash) {
        algorithms.push(109280);
    }
    if sha1saltmd5pass(hash) {
        algorithms.push(109300);
    }
    if sha1saltmd5passsalt(hash) {
        algorithms.push(109320);
    }
    if sha1saltsha1pass(hash) {
        algorithms.push(109340);
    }
    if sha1saltsha1saltsha1pass(hash) {
        algorithms.push(109360);
    }
    if sha1usernamepass(hash) {
        algorithms.push(109380);
    }
    if sha1usernamepasssalt(hash) {
        algorithms.push(109400);
    }
    if sha1md5passsalt(hash) {
        algorithms.push(109440);
    }
    if sha1md5sha1pass(hash) {
        algorithms.push(109460);
    }
    if sha1sha1pass(hash) {
        algorithms.push(109480);
    }
    if sha1sha1passsalt(hash) {
        algorithms.push(109500);
    }
    if sha1sha1passsubstrpass03(hash) {
        algorithms.push(109520);
    }
    if sha1sha1saltpass(hash) {
        algorithms.push(109540);
    }
    if sha1sha1sha1pass(hash) {
        algorithms.push(109560);
    }
    if sha1strtolowerusernamepass(hash) {
        algorithms.push(109580);
    }
    if Tiger192(hash) {
        algorithms.push(110020);
    }
    if Haval192(hash) {
        algorithms.push(110040);
    }
    if Tiger192HMAC(hash) {
        algorithms.push(110060);
    }
    if Haval192HMAC(hash) {
        algorithms.push(110080);
    }
    if MD5passsaltjoomla1(hash) {
        algorithms.push(112020);
    }
    if SHA1Django(hash) {
        algorithms.push(113020);
    }
    if SHA224(hash) {
        algorithms.push(114020);
    }
    if Haval224(hash) {
        algorithms.push(114040);
    }
    if SHA224HMAC(hash) {
        algorithms.push(114060);
    }
    if Haval224HMAC(hash) {
        algorithms.push(114080);
    }
    if SHA256(hash) {
        algorithms.push(115020);
    }
    if Haval256(hash) {
        algorithms.push(115040);
    }
    if GOSTR341194(hash) {
        algorithms.push(115060);
    }
    if RipeMD256(hash) {
        algorithms.push(115080);
    }
    if SNEFRU256(hash) {
        algorithms.push(115100);
    }
    if SHA256HMAC(hash) {
        algorithms.push(115120);
    }
    if Haval256HMAC(hash) {
        algorithms.push(115140);
    }
    if RipeMD256HMAC(hash) {
        algorithms.push(115160);
    }
    if SNEFRU256HMAC(hash) {
        algorithms.push(115180);
    }
    if SHA256md5pass(hash) {
        algorithms.push(115200);
    }
    if SHA256sha1pass(hash) {
        algorithms.push(115220);
    }
    if MD5passsaltjoomla2(hash) {
        algorithms.push(116020);
    }
    if SAM(hash) {
        algorithms.push(116040);
    }
    if SHA256Django(hash) {
        algorithms.push(117020);
    }
    if RipeMD320(hash) {
        algorithms.push(118020);
    }
    if RipeMD320HMAC(hash) {
        algorithms.push(118040);
    }
    if SHA384(hash) {
        algorithms.push(119020);
    }
    if SHA384HMAC(hash) {
        algorithms.push(119040);
    }
    if SHA256s(hash) {
        algorithms.push(120020);
    }
    if SHA384Django(hash) {
        algorithms.push(121020);
    }
    if SHA512(hash) {
        algorithms.push(122020);
    }
    if Whirlpool(hash) {
        algorithms.push(122040);
    }
    if SHA512HMAC(hash) {
        algorithms.push(122060);
    }
    if WhirlpoolHMAC(hash) {
        algorithms.push(122080);
    }
    if sha1md5pass(hash) {
        algorithms.push(1094202);
    }

    algorithms
}

fn create_algorithms_map() -> HashMap<u32, String> {
    let mut algorithms = HashMap::new();

    algorithms.insert(102020, "ADLER-32".to_string());
    algorithms.insert(101020, "CRC-16".to_string());
    algorithms.insert(101040, "CRC-16-CCITT".to_string());
    algorithms.insert(101060, "FCS-16".to_string());
    algorithms.insert(102020, "ADLER-32".to_string());
    algorithms.insert(102040, "CRC-32".to_string());
    algorithms.insert(102060, "CRC-32B".to_string());
    algorithms.insert(102080, "XOR-32".to_string());
    algorithms.insert(103020, "GHash-32-5".to_string());
    algorithms.insert(103040, "GHash-32-3".to_string());
    algorithms.insert(104020, "DES(Unix)".to_string());
    algorithms.insert(105020, "MySQL".to_string());
    algorithms.insert(105040, "MD5(Middle)".to_string());
    algorithms.insert(105060, "MD5(Half)".to_string());
    algorithms.insert(106020, "MD5".to_string());
    algorithms.insert(
        106025,
        "Domain Cached Credentials - MD4(MD4(($pass)).(strtolower($username)))".to_string(),
    );
    algorithms.insert(106027, "RAdmin v2.x".to_string());
    algorithms.insert(106029, "NTLM".to_string());
    algorithms.insert(106040, "MD4".to_string());
    algorithms.insert(106060, "MD2".to_string());
    algorithms.insert(106080, "MD5(HMAC)".to_string());
    algorithms.insert(106100, "MD4(HMAC)".to_string());
    algorithms.insert(106120, "MD2(HMAC)".to_string());
    algorithms.insert(106140, "MD5(HMAC(Wordpress))".to_string());
    algorithms.insert(106160, "Haval-128".to_string());
    algorithms.insert(106165, "Haval-128(HMAC)".to_string());
    algorithms.insert(106180, "RipeMD-128".to_string());
    algorithms.insert(106185, "RipeMD-128(HMAC)".to_string());
    algorithms.insert(106200, "SNEFRU-128".to_string());
    algorithms.insert(106205, "SNEFRU-128(HMAC)".to_string());
    algorithms.insert(106220, "Tiger-128".to_string());
    algorithms.insert(106225, "Tiger-128(HMAC)".to_string());
    algorithms.insert(106240, "md5($pass.$salt)".to_string());
    algorithms.insert(106260, "md5($salt.'-'.md5($pass))".to_string());
    algorithms.insert(106280, "md5($salt.$pass)".to_string());
    algorithms.insert(106300, "md5($salt.$pass.$salt)".to_string());
    algorithms.insert(106320, "md5($salt.$pass.$username)".to_string());
    algorithms.insert(106340, "md5($salt.md5($pass))".to_string());
    algorithms.insert(106360, "md5($salt.md5($pass).$salt)".to_string());
    algorithms.insert(106380, "md5($salt.md5($pass.$salt))".to_string());
    algorithms.insert(106400, "md5($salt.md5($salt.$pass))".to_string());
    algorithms.insert(106420, "md5($salt.md5(md5($pass).$salt))".to_string());
    algorithms.insert(106440, "md5($username.0.$pass)".to_string());
    algorithms.insert(106460, "md5($username.LF.$pass)".to_string());
    algorithms.insert(106480, "md5($username.md5($pass).$salt)".to_string());
    algorithms.insert(106500, "md5(md5($pass))".to_string());
    algorithms.insert(106520, "md5(md5($pass).$salt)".to_string());
    algorithms.insert(106540, "md5(md5($pass).md5($salt))".to_string());
    algorithms.insert(106560, "md5(md5($salt).$pass)".to_string());
    algorithms.insert(106580, "md5(md5($salt).md5($pass))".to_string());
    algorithms.insert(106600, "md5(md5($username.$pass).$salt)".to_string());
    algorithms.insert(106620, "md5(md5(md5($pass)))".to_string());
    algorithms.insert(106640, "md5(md5(md5(md5($pass))))".to_string());
    algorithms.insert(106660, "md5(md5(md5(md5(md5($pass)))))".to_string());
    algorithms.insert(106680, "md5(sha1($pass))".to_string());
    algorithms.insert(106700, "md5(sha1(md5($pass)))".to_string());
    algorithms.insert(106720, "md5(sha1(md5(sha1($pass))))".to_string());
    algorithms.insert(106740, "md5(strtoupper(md5($pass)))".to_string());
    algorithms.insert(107020, "MD5(Wordpress)".to_string());
    algorithms.insert(107040, "MD5(phpBB3)".to_string());
    algorithms.insert(107060, "MD5(Unix)".to_string());
    algorithms.insert(107080, "Lineage II C4".to_string());
    algorithms.insert(108020, "MD5(APR)".to_string());
    algorithms.insert(109020, "SHA-1".to_string());
    algorithms.insert(109040, "MySQL5 - SHA-1(SHA-1($pass))".to_string());
    algorithms.insert(109060, "MySQL 160bit - SHA-1(SHA-1($pass))".to_string());
    algorithms.insert(109080, "Tiger-160".to_string());
    algorithms.insert(109100, "Haval-160".to_string());
    algorithms.insert(109120, "RipeMD-160".to_string());
    algorithms.insert(109140, "SHA-1(HMAC)".to_string());
    algorithms.insert(109160, "Tiger-160(HMAC)".to_string());
    algorithms.insert(109180, "RipeMD-160(HMAC)".to_string());
    algorithms.insert(109200, "Haval-160(HMAC)".to_string());
    algorithms.insert(109220, "SHA-1(MaNGOS)".to_string());
    algorithms.insert(109240, "SHA-1(MaNGOS2)".to_string());
    algorithms.insert(109260, "sha1($pass.$salt)".to_string());
    algorithms.insert(109280, "sha1($salt.$pass)".to_string());
    algorithms.insert(109300, "sha1($salt.md5($pass))".to_string());
    algorithms.insert(109320, "sha1($salt.md5($pass).$salt)".to_string());
    algorithms.insert(109340, "sha1($salt.sha1($pass))".to_string());
    algorithms.insert(109360, "sha1($salt.sha1($salt.sha1($pass)))".to_string());
    algorithms.insert(109380, "sha1($username.$pass)".to_string());
    algorithms.insert(109400, "sha1($username.$pass.$salt)".to_string());
    algorithms.insert(109440, "sha1(md5($pass).$salt)".to_string());
    algorithms.insert(109460, "sha1(md5(sha1($pass)))".to_string());
    algorithms.insert(109480, "sha1(sha1($pass))".to_string());
    algorithms.insert(109500, "sha1(sha1($pass).$salt)".to_string());
    algorithms.insert(109520, "sha1(sha1($pass).substr($pass,0,3))".to_string());
    algorithms.insert(109540, "sha1(sha1($salt.$pass))".to_string());
    algorithms.insert(109560, "sha1(sha1(sha1($pass)))".to_string());
    algorithms.insert(109580, "sha1(strtolower($username).$pass)".to_string());
    algorithms.insert(110020, "Tiger-192".to_string());
    algorithms.insert(110040, "Haval-192".to_string());
    algorithms.insert(110060, "Tiger-192(HMAC)".to_string());
    algorithms.insert(110080, "Haval-192(HMAC)".to_string());
    algorithms.insert(112020, "md5($pass.$salt) - Joomla".to_string());
    algorithms.insert(113020, "SHA-1(Django)".to_string());
    algorithms.insert(114020, "SHA-224".to_string());
    algorithms.insert(114040, "Haval-224".to_string());
    algorithms.insert(114060, "SHA-224(HMAC)".to_string());
    algorithms.insert(114080, "Haval-224(HMAC)".to_string());
    algorithms.insert(115020, "SHA-256".to_string());
    algorithms.insert(115040, "Haval-256".to_string());
    algorithms.insert(115060, "GOST R 34.11-94".to_string());
    algorithms.insert(115080, "RipeMD-256".to_string());
    algorithms.insert(115100, "SNEFRU-256".to_string());
    algorithms.insert(115120, "SHA-256(HMAC)".to_string());
    algorithms.insert(115140, "Haval-256(HMAC)".to_string());
    algorithms.insert(115160, "RipeMD-256(HMAC)".to_string());
    algorithms.insert(115180, "SNEFRU-256(HMAC)".to_string());
    algorithms.insert(115200, "SHA-256(md5($pass))".to_string());
    algorithms.insert(115220, "SHA-256(sha1($pass))".to_string());
    algorithms.insert(116020, "md5($pass.$salt) - Joomla".to_string());
    algorithms.insert(116040, "SAM - (LM_hash:NT_hash)".to_string());
    algorithms.insert(117020, "SHA-256(Django)".to_string());
    algorithms.insert(118020, "RipeMD-320".to_string());
    algorithms.insert(118040, "RipeMD-320(HMAC)".to_string());
    algorithms.insert(119020, "SHA-384".to_string());
    algorithms.insert(119040, "SHA-384(HMAC)".to_string());
    algorithms.insert(120020, "SHA-256".to_string());
    algorithms.insert(121020, "SHA-384(Django)".to_string());
    algorithms.insert(122020, "SHA-512".to_string());
    algorithms.insert(122040, "Whirlpool".to_string());
    algorithms.insert(122060, "SHA-512(HMAC)".to_string());
    algorithms.insert(122080, "Whirlpool(HMAC)".to_string());
    algorithms.insert(1094202, "sha1(md5($pass))".to_string());

    algorithms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_map() {
        let key = 1094202;
        let map = create_algorithms_map();

        assert_eq!("sha1(md5($pass))", map.get(&key).unwrap());
    }
}
