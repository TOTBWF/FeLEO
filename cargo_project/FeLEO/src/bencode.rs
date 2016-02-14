// FeLEO
//
// bencode.rs

// imports here

fn int_handler(data: &str, data_len: i64) -> &str {
    // TODO
    // gets "i[int]e" returns int
    ""
}

pub fn be_bencode(data: &str, data_len: i64) -> &str {
    let mut ret = "";

    if data_len == 0 {
        return ret;
    }

    // match is similar to a switch statement
    match data {

        // integers
        "i" => {
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
        "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => { ; } // TODO

        // lists
        "l" => { ; } // TODO

        // dictionaries
        "d" => { ; } // TODO

        // else: error
        _ => { println!("error: bencoded string not recognized"); }
    }

    ret

}

