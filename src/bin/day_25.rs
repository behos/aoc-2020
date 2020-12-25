const DOOR_PUB: usize = 18499292;
const CARD_PUB: usize = 8790390;
const SUB: usize = 7;
const DIV: usize = 20201227;

fn main() {
    let door_loop_size = get_loop_size(DOOR_PUB);
    let card_loop_size = get_loop_size(CARD_PUB);
    let enc_from_card = get_key(card_loop_size, DOOR_PUB);
    let enc_from_door = get_key(door_loop_size, CARD_PUB);

    assert_eq!(enc_from_door, enc_from_card);
    println!("The enc key is {}", enc_from_door);
}

fn get_loop_size(pub_key: usize) -> usize {
    let mut num = 1;
    let mut loop_size = 0;
    while num != pub_key {
        loop_size += 1;
        num *= SUB;
        num %= DIV;
    }
    loop_size
}

fn get_key(loop_size: usize, subject: usize) -> usize {
    let mut enc = 1;
    for _ in 0..loop_size {
        enc *= subject;
        enc %= DIV;
    }
    enc
}
