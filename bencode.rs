// FeLEO
// 
// bencode.rs

// imports here

pub fn be_decode(data: &str, data_len: i64) -> &str {
    let mut ret = "";
    
    if data_len == 0 {
        return ret;
    }

    // match is similar to a switch statement
    match data {

        // integers
        "i" => { 
            // indexing string by character doesn't work in rust
            let mut i = 0;
            let mut decoded_int: str = "";
            while i < data.len() {
                    //let CharRange {ch, next} = data.char_range_at(i);
                    let mut ch: str = data.char_range_at(i);
                    decoded_int = format!{"{}{}",decoded_int, ch};
                    i += 1;
            }
            //let decoded_int: &str = &data[1..data_len-1];
            //let CharRange {after_int, next} = data.char_range_at(i);
            let after_int: str = data.char_range_at(i);
            if after_int != "e" {
                println!("invalid value; rejecting it");
                return "";
            }
            return decoded_int;
        }

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

