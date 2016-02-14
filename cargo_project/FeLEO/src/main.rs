// FeLEO
// 
// main file for BitTorrent protocol

// imports here
mod bencode;

fn main() {
    bencode::be_bencode(vec![], 0);
    println!("End of main");
}
