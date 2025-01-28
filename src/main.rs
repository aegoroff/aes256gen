use rand::Rng;

fn main() {
    let secret = rand::rng().random::<[u8; 32]>();
    println!("{}", hex::encode_upper(secret));
}
