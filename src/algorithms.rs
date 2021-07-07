#[allow(non_snake_case)]

use helpers::{is_alnum, is_alpha, is_digit, is_lower};

pub fn CRC16(hash: &str) -> bool {
    let hs: &str = "4607";
    if hash.len() == hs.len() && is_alpha(hash) == false && is_alnum(hash) == true {
        return true;
    } else {
        return false;
    }
}

pub fn CRC16CCITT(hash: &str) -> bool {
    let hs: &str = "3d08";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn FCS16(hash: &str) -> bool {
    let hs: &str = "0e5b";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn CRC32(hash: &str) -> bool {
    let hs: &str = "b33fd057";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn ADLER32(hash: &str) -> bool {
    let hs: &str = "0607cb42";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn CRC32B(hash: &str) -> bool {
    let hs: &str = "b764a0d9";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn XOR32(hash: &str) -> bool {
    let hs: &str = "0000003f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn GHash323(hash: &str) -> bool {
    let hs: &str = "80000000";
    if hash.len() == hs.len()
        && is_digit(hash) == true
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn GHash325(hash: &str) -> bool {
    let hs: &str = "85318985";
    if hash.len() == hs.len()
        && is_digit(hash) == true
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn DESUnix(hash: &str) -> bool {
    let hs: &str = "ZiY8YtDKXJwYQ";
    if hash.len() == hs.len() && is_digit(hash) == false && is_alpha(hash) == false {
        return true;
    } else {
        return false;
    }
}

pub fn MD5Half(hash: &str) -> bool {
    let hs: &str = "ae11fd697ec92c7c";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5Middle(hash: &str) -> bool {
    let hs: &str = "7ec92c7c98de3fac";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MySQL(hash: &str) -> bool {
    let hs: &str = "63cea4673fd25f46";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn DomainCachedCredentials(hash: &str) -> bool {
    let hs: &str = "f42005ec1afe77967cbc83dce1b4d714";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval128(hash: &str) -> bool {
    let hs: &str = "d6e3ec49aa0f138a619f27609022df10";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval128HMAC(hash: &str) -> bool {
    let hs: &str = "3ce8b0ffd75bc240fc7d967729cd6637";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD2(hash: &str) -> bool {
    let hs: &str = "08bbef4754d98806c373f2cd7d9a43c4";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD2HMAC(hash: &str) -> bool {
    let hs: &str = "4b61b72ead2b0eb0fa3b8a56556a6dca";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD4(hash: &str) -> bool {
    let hs: &str = "a2acde400e61410e79dacbdfc3413151";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD4HMAC(hash: &str) -> bool {
    let hs: &str = "6be20b66f2211fe937294c1c95d1cd4f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5(hash: &str) -> bool {
    let hs: &str = "ae11fd697ec92c7c98de3fac23aba525";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5HMAC(hash: &str) -> bool {
    let hs: &str = "d57e43d2c7e397bf788f66541d6fdef9";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5HMACWordpress(hash: &str) -> bool {
    let hs: &str = "3f47886719268dfa83468630948228f6";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn NTLM(hash: &str) -> bool {
    let hs: &str = "cc348bace876ea440a28ddaeb9fd3550";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RAdminv2x(hash: &str) -> bool {
    let hs: &str = "baea31c728cbf0cd548476aa687add4b";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD128(hash: &str) -> bool {
    let hs: &str = "4985351cd74aff0abc5a75a0c8a54115";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD128HMAC(hash: &str) -> bool {
    let hs: &str = "ae1995b931cf4cbcf1ac6fbf1a83d1d3";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SNEFRU128(hash: &str) -> bool {
    let hs: &str = "4fb58702b617ac4f7ca87ec77b93da8a";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SNEFRU128HMAC(hash: &str) -> bool {
    let hs: &str = "59b2b9dcc7a9a7d089cecf1b83520350";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Tiger128(hash: &str) -> bool {
    let hs: &str = "c086184486ec6388ff81ec9f23528727";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Tiger128HMAC(hash: &str) -> bool {
    let hs: &str = "c87032009e7c4b2ea27eb6f99723454b";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5passsalt(hash: &str) -> bool {
    let hs: &str = "5634cc3b922578434d6e9342ff5913f7";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltmd5pass(hash: &str) -> bool {
    let hs: &str = "245c5763b95ba42d4b02d44bbcd916f1";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltpass(hash: &str) -> bool {
    let hs: &str = "22cc5ce1a1ef747cd3fa06106c148dfa";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltpasssalt(hash: &str) -> bool {
    let hs: &str = "469e9cdcaff745460595a7a386c4db0c";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltpassusername(hash: &str) -> bool {
    let hs: &str = "9ae20f88189f6e3a62711608ddb6f5fd";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltmd5passsalt(hash: &str) -> bool {
    let hs: &str = "de0237dc03a8efdf6552fbe7788b2fdd";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltmd5passsalt1(hash: &str) -> bool {
    //TODO: note that is different from original project
    let hs: &str = "5b8b12ca69d3e7b2a3e2308e7bef3e6f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltmd5saltpass2(hash: &str) -> bool {
    //TODO: note that is different from original project
    let hs: &str = "d8f3b3f004d387086aae24326b575b23";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5saltmd5md5passsalt(hash: &str) -> bool {
    let hs: &str = "81f181454e23319779b03d74d062b1a2";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5username0pass(hash: &str) -> bool {
    let hs: &str = "e44a60f8f2106492ae16581c91edb3ba";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5usernameLFpass(hash: &str) -> bool {
    let hs: &str = "654741780db415732eaee12b1b909119";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5usernamemd5passsalt(hash: &str) -> bool {
    let hs: &str = "954ac5505fd1843bbb97d1b2cda0b98f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5pass(hash: &str) -> bool {
    let hs: &str = "a96103d267d024583d5565436e52dfb3";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5passsalt(hash: &str) -> bool {
    let hs: &str = "5848c73c2482d3c2c7b6af134ed8dd89";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5passmd5salt(hash: &str) -> bool {
    let hs: &str = "8dc71ef37197b2edba02d48c30217b32";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5saltpass(hash: &str) -> bool {
    let hs: &str = "9032fabd905e273b9ceb1e124631bd67";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5saltmd5pass(hash: &str) -> bool {
    let hs: &str = "8966f37dbb4aca377a71a9d3d09cd1ac";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5usernamepasssalt(hash: &str) -> bool {
    let hs: &str = "4319a3befce729b34c3105dbc29d0c40";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5md5pass(hash: &str) -> bool {
    let hs: &str = "ea086739755920e732d0f4d8c1b6ad8d";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5md5md5pass(hash: &str) -> bool {
    let hs: &str = "02528c1f2ed8ac7d83fe76f3cf1c133f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5md5md5md5md5pass(hash: &str) -> bool {
    let hs: &str = "4548d2c062933dff53928fd4ae427fc0";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5sha1pass(hash: &str) -> bool {
    let hs: &str = "cb4ebaaedfd536d965c452d9569a6b1e";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5sha1md5pass(hash: &str) -> bool {
    let hs: &str = "099b8a59795e07c334a696a10c0ebce0";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5sha1md5sha1pass(hash: &str) -> bool {
    let hs: &str = "06e4af76833da7cc138d90602ef80070";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn md5strtouppermd5pass(hash: &str) -> bool {
    let hs: &str = "519de146f1a658ab5e5e2aa9b7d2eec8";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn LineageIIC4(hash: &str) -> bool {
    let hs: &str = "0x49a57f66bd3d5ba6abda5579c264a0e4";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
        && hash[..2].contains("0x") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5phpBB3(hash: &str) -> bool {
    let hs: &str = "$H$9kyOtE8CDqMJ44yfn9PFz2E.L2oVzL1";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..3].contains("$H$") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5Unix(hash: &str) -> bool {
    let hs: &str = "$1$cTuJH0Ju$1J8rI.mJReeMvpKUZbSlY/";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..3].contains("$1$") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5Wordpress(hash: &str) -> bool {
    let hs: &str = "$P$BiTOhOj3ukMgCci2juN0HRbCdDRqeh.";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..3].contains("$P$") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5APR(hash: &str) -> bool {
    let hs: &str = "$apr1$qAUKoKlG$3LuCncByN76eLxZAh/Ldr1";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && hash[..4].contains("$apr") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval160(hash: &str) -> bool {
    let hs: &str = "a106e921284dd69dad06192a4411ec32fce83dbb";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval160HMAC(hash: &str) -> bool {
    let hs: &str = "29206f83edc1d6c3f680ff11276ec20642881243";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MySQL5(hash: &str) -> bool {
    let hs: &str = "9bb2fb57063821c762cc009f7584ddae9da431ff";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MySQL160bit(hash: &str) -> bool {
    let hs: &str = "*2470c0c06dee42fd1618bb99005adca2ec9d1e19";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..1].contains("*") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD160(hash: &str) -> bool {
    let hs: &str = "dc65552812c66997ea7320ddfb51f5625d74721b";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD160HMAC(hash: &str) -> bool {
    let hs: &str = "ca28af47653b4f21e96c1235984cb50229331359";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA1(hash: &str) -> bool {
    let hs: &str = "4a1d4dbc1e193ec3ab2e9213876ceb8f4db72333";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA1HMAC(hash: &str) -> bool {
    let hs: &str = "6f5daac3fee96ba1382a09b1ba326ca73dccf9e7";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA1MaNGOS(hash: &str) -> bool {
    let hs: &str = "a2c0cdb6d1ebd1b9f85c6e25e0f8732e88f02f96";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA1MaNGOS2(hash: &str) -> bool {
    let hs: &str = "644a29679136e09d0bd99dfd9e8c5be84108b5fd";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Tiger160(hash: &str) -> bool {
    let hs: &str = "c086184486ec6388ff81ec9f235287270429b225";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Tiger160HMAC(hash: &str) -> bool {
    let hs: &str = "6603161719da5e56e1866e4f61f79496334e6a10";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1passsalt(hash: &str) -> bool {
    let hs: &str = "f006a1863663c21c541c8d600355abfeeaadb5e4";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1saltpass(hash: &str) -> bool {
    let hs: &str = "299c3d65a0dcab1fc38421783d64d0ecf4113448";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1saltmd5pass(hash: &str) -> bool {
    let hs: &str = "860465ede0625deebb4fbbedcb0db9dc65faec30";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1saltmd5passsalt(hash: &str) -> bool {
    let hs: &str = "6716d047c98c25a9c2cc54ee6134c73e6315a0ff";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1saltsha1pass(hash: &str) -> bool {
    let hs: &str = "58714327f9407097c64032a2fd5bff3a260cb85f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1saltsha1saltsha1pass(hash: &str) -> bool {
    let hs: &str = "cc600a2903130c945aa178396910135cc7f93c63";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1usernamepass(hash: &str) -> bool {
    let hs: &str = "3de3d8093bf04b8eb5f595bc2da3f37358522c9f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1usernamepasssalt(hash: &str) -> bool {
    let hs: &str = "00025111b3c4d0ac1635558ce2393f77e94770c5";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1md5pass(hash: &str) -> bool {
    let hs: &str = "fa960056c0dea57de94776d3759fb555a15cae87";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1md5passsalt(hash: &str) -> bool {
    let hs: &str = "1dad2b71432d83312e61d25aeb627593295bcc9a";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1md5sha1pass(hash: &str) -> bool {
    let hs: &str = "8bceaeed74c17571c15cdb9494e992db3c263695";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1sha1pass(hash: &str) -> bool {
    let hs: &str = "3109b810188fcde0900f9907d2ebcaa10277d10e";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1sha1passsalt(hash: &str) -> bool {
    let hs: &str = "780d43fa11693b61875321b6b54905ee488d7760";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1sha1passsubstrpass03(hash: &str) -> bool {
    let hs: &str = "5ed6bc680b59c580db4a38df307bd4621759324e";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1sha1saltpass(hash: &str) -> bool {
    let hs: &str = "70506bac605485b4143ca114cbd4a3580d76a413";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1sha1sha1pass(hash: &str) -> bool {
    let hs: &str = "3328ee2a3b4bf41805bd6aab8e894a992fa91549";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn sha1strtolowerusernamepass(hash: &str) -> bool {
    let hs: &str = "79f575543061e158c2da3799f999eb7c95261f07";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval192(hash: &str) -> bool {
    let hs: &str = "cd3a90a3bebd3fa6b6797eba5dab8441f16a7dfa96c6e641";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval192HMAC(hash: &str) -> bool {
    let hs: &str = "39b4d8ecf70534e2fd86bb04a877d01dbf9387e640366029";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Tiger192(hash: &str) -> bool {
    let hs: &str = "c086184486ec6388ff81ec9f235287270429b2253b248a70";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Tiger192HMAC(hash: &str) -> bool {
    let hs: &str = "8e914bb64353d4d29ab680e693272d0bd38023afa3943a41";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5passsaltjoomla1(hash: &str) -> bool {
    let hs: &str = "35d1c0d69a2df62be2df13b087343dc9:BeKMviAfcXeTPTlX";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[32..33].contains(":") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA1Django(hash: &str) -> bool {
    let hs: &str = "sha1$Zion3R$299c3d65a0dcab1fc38421783d64d0ecf4113448";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..5].contains("sha1$") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval224(hash: &str) -> bool {
    let hs: &str = "f65d3c0ef6c56f4c74ea884815414c24dbf0195635b550f47eac651a";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval224HMAC(hash: &str) -> bool {
    let hs: &str = "f10de2518a9f7aed5cf09b455112114d18487f0c894e349c3c76a681";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA224(hash: &str) -> bool {
    let hs: &str = "e301f414993d5ec2bd1d780688d37fe41512f8b57f6923d054ef8e59";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA224HMAC(hash: &str) -> bool {
    let hs: &str = "c15ff86a859892b5e95cdfd50af17d05268824a6c9caaa54e4bf1514";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA256(hash: &str) -> bool {
    let hs: &str = "2c740d20dab7f14ec30510a11f8fd78b82bc3a711abe8a993acdb323e78e6d5e";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA256HMAC(hash: &str) -> bool {
    let hs: &str = "d3dd251b7668b8b6c12e639c681e88f2c9b81105ef41caccb25fcde7673a1132";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval256(hash: &str) -> bool {
    let hs: &str = "7169ecae19a5cd729f6e9574228b8b3c91699175324e6222dec569d4281d4a4a";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Haval256HMAC(hash: &str) -> bool {
    let hs: &str = "6aa856a2cfd349fb4ee781749d2d92a1ba2d38866e337a4a1db907654d4d4d7a";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn GOSTR341194(hash: &str) -> bool {
    let hs: &str = "ab709d384cce5fda0793becd3da0cb6a926c86a8f3460efb471adddee1c63793";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD256(hash: &str) -> bool {
    let hs: &str = "5fcbe06df20ce8ee16e92542e591bdea706fbdc2442aecbf42c223f4461a12af";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD256HMAC(hash: &str) -> bool {
    let hs: &str = "43227322be1b8d743e004c628e0042184f1288f27c13155412f08beeee0e54bf";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SNEFRU256(hash: &str) -> bool {
    let hs: &str = "3a654de48e8d6b669258b2d33fe6fb179356083eed6ff67e27c5ebfa4d9732bb";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SNEFRU256HMAC(hash: &str) -> bool {
    let hs: &str = "4e9418436e301a488f675c9508a2d518d8f8f99e966136f2dd7e308b194d74f9";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA256md5pass(hash: &str) -> bool {
    let hs: &str = "b419557099cfa18a86d1d693e2b3b3e979e7a5aba361d9c4ec585a1a70c7bde4";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA256sha1pass(hash: &str) -> bool {
    let hs: &str = "afbed6e0c79338dbfe0000efe6b8e74e3b7121fe73c383ae22f5b505cb39c886";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn MD5passsaltjoomla2(hash: &str) -> bool {
    let hs: &str = "fb33e01e4f8787dc8beb93dac4107209:fxJUXVjYRafVauT77Cze8XwFrWaeAYB2";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[32..33].contains(":") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SAM(hash: &str) -> bool {
    let hs: &str = "4318B176C3D8E3DEAAD3B435B51404EE:B7C899154197E8A2A33121D76A240AB5";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && is_lower(hash) == false
        && hash[32..33].contains(":") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA256Django(hash: &str) -> bool {
    let hs: &str = "sha256$Zion3R$9e1a08aa28a22dfff722fad7517bae68a55444bb5e2f909d340767cec9acf2c3";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..6].contains("sha256") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD320(hash: &str) -> bool {
    let hs: &str =
        "b4f7c8993a389eac4f421b9b3b2bfb3a241d05949324a8dab1286069a18de69aaf5ecc3c2009d8ef";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn RipeMD320HMAC(hash: &str) -> bool {
    let hs: &str =
        "244516688f8ad7dd625836c0d0bfc3a888854f7c0161f01de81351f61e98807dcd55b39ffe5d7a78";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA384(hash: &str) -> bool {
    let hs: &str = "3b21c44f8d830fa55ee9328a7713c6aad548fe6d7a4a438723a0da67c48c485220081a2fbc3e8c17fd9bd65f8d4b4e6b";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA384HMAC(hash: &str) -> bool {
    let hs: &str = "bef0dd791e814d28b4115eb6924a10beb53da47d463171fe8e63f68207521a4171219bb91d0580bca37b0f96fddeeb8b";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA256s(hash: &str) -> bool {
    let hs: &str = "$6$g4TpUQzk$OmsZBJFwvy6MwZckPvVYfDnwsgktm2CckOlNJGy9HNwHSuHFvywGIuwkJ6Bjn3kKbB6zoyEjIYNMpHWBNxJ6g.";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..3].contains("$6$") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA384Django(hash: &str) -> bool {
    let hs: &str = "sha384$Zion3R$88cfd5bc332a4af9f09aa33a1593f24eddc01de00b84395765193c3887f4deac46dc723ac14ddeb4d3a9b958816b7bba";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == false
        && hash[..6].contains("sha384") == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA512(hash: &str) -> bool {
    let hs: &str = "ea8e6f0935b34e2e6573b89c0856c81b831ef2cadfdee9f44eb9aa0955155ba5e8dd97f85c73f030666846773c91404fb0e12fb38936c56f8cf38a33ac89a24e";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn SHA512HMAC(hash: &str) -> bool {
    let hs: &str = "dd0ada8693250b31d9f44f3ec2d4a106003a6ce67eaa92e384b356d1b4ef6d66a818d47c1f3a2c6e8a9a9b9bdbd28d485e06161ccd0f528c8bbb5541c3fef36f";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn Whirlpool(hash: &str) -> bool {
    let hs: &str = "76df96157e632410998ad7f823d82930f79a96578acc8ac5ce1bfc34346cf64b4610aefa8a549da3f0c1da36dad314927cebf8ca6f3fcd0649d363c5a370dddb";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

pub fn WhirlpoolHMAC(hash: &str) -> bool {
    let hs: &str = "77996016cf6111e97d6ad31484bab1bf7de7b7ee64aebbc243e650a75a2f9256cef104e504d3cf29405888fca5a231fcac85d36cd614b1d52fce850b53ddf7f9";
    if hash.len() == hs.len()
        && is_digit(hash) == false
        && is_alpha(hash) == false
        && is_alnum(hash) == true
    {
        return true;
    } else {
        return false;
    }
}

mod helpers {
    pub fn is_digit(s: &str) -> bool {
        s.chars().all(char::is_numeric)
    }

    pub fn is_alpha(s: &str) -> bool {
        s.chars().all(char::is_alphabetic)
    }

    pub fn is_alnum(s: &str) -> bool {
        s.chars().all(char::is_alphanumeric)
    }

    pub fn is_lower(s: &str) -> bool {
        s.chars().all(char::is_lowercase)
    }
}
