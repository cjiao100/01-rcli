use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut password = Vec::new();
    let mut rng = rand::rng();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        // 如果开启了对应配置，就从UPPER中随机选择一个字符
        password.push(*UPPER.choose(&mut rng).expect("UPPER is empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("UPPER is empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("UPPER is empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("UPPER is empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        // let idx = rng.random_range(0..chars.len());
        let c = chars.choose(&mut rng).expect("chars is empty");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    // 打印密码强度
    let estimate = zxcvbn(&password, &[]);
    println!("score: {}", estimate.score());
    Ok(password)
}
