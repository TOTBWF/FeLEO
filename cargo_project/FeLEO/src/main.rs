// FeLEO
// 
// main file for BitTorrent protocol

// imports here
mod bencode;

fn main() {
    bencode::be_bencode("", 0);
    println!("End of main");
}
