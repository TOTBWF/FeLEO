// FeLEO
// 
// bencode.rs

// imports here

pub fn be_decode(data: &'static str, data_len: i64) -> &str {
    let mut ret = "";

    if data_len == 0 {
        return ret;
    }

    // match is similar to a switch statement
    match data {
        // integers
        "i" => { ; } // do a thing
        // strings
        "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => { ; } // do a thing
        // lists
        "l" => { ; } // do a thing
        // dictionaries
        "d" => { ; } // do a thing
        // else: error
        _ => { println!("error: bencoded string not recognized"); }
    }

    ret

}
