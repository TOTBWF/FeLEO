// FeLEO
//
// main file for BitTorrent protocol

//use collections::string::String;
mod bencode;

fn main() {
    //let mut str = String::new();
    bencode::be_decode("", 0);
    println!("So far, this does nothing.");
}
