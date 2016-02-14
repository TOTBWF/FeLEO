// FeLEO
//
// bencode.rs

// imports here

fn int_handler(data: Vec<char>, data_len: i64) -> Vec<char> {
    // TODO
    // gets "i[int]e" returns int
    let mut decoded_int: Vec<char>;

    let mut i = 1; // start after i
    while data[i] != 'e' {
        decoded_int[i-1] = data[i];
    }

    if let decoded_int = decoded_int {
        return decoded_int;
    }
    println!("Error: data NULL");
    vec![]
}

pub fn be_bencode(data: Vec<char>, data_len: i64) -> Vec<char> {
    let mut ret: Vec<char>;

    if data_len == 0 {
        return vec![];
    }

    // match is similar to a switch statement
    match data[0] {

        // integers
        'i' => {
            // indexing string by character doesn't work in rust
            //let mut i = 0;
            //let mut decoded_int: str = "";
            //while i < data.len() {
            //    let mut ch: str = data.char_range_at(i);
            //    decoded_int = format!{"{}{}", decoded_int, ch};
            //    i += 1;
            //}
            //let after_int: str = data.char_range_at(i);
            //if after_int != "e" {
            //    println!("Invalid value; rejecting it");
            //    ""
            //}
            //decoded_int
            return int_handler(data, data_len);
        }

        // strings
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => { ; } // TODO

        // lists
        'l' => { ; } // TODO

        // dictionaries
        'd' => { ; } // TODO

        // else: error
        _ => { println!("error: bencoded string not recognized"); }
    }

    ret

}

