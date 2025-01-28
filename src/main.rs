fn main() {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEF0123456789";
    const PASSWORD_LEN: usize = 64;
    let mut rng = rand::rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{password}");
}
