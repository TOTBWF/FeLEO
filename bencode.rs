// FeLEO
// 
// bencode.rs

// imports here

fn decode_int(data: &str, data_len: i64) -> &str {
    // TODO
    ""
}

pub fn be_decode(data: &str, data_len: i64) -> &str {
    let mut ret = "";
    
    if data_len == 0 {
        return ret;
    }

    // match is similar to a switch statement
    match data {
        // integers
        "i" => { 

            ret = decode_int(&data[1..], data_len -1);
            if data != "e" {
                println!("invalid value; rejecting it");
                return "";
            }
            return ret;
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

