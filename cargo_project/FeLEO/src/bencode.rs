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
            return int_handler(data, data_len);
        }

        // strings
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => { // TODO
            let mut i = 0;
            let mut string_len = 0;
            while data[i] == '0' || data[i] == '1' || data[i] ==  '2' || data[i] == '3' || 
                  data[i] == '4' || data[i] == '5' || data[i] ==  '6' || data[i] == '7' || 
                  data[i] == '8' || data[i] == '9'
                {
                    // cast data[i] as int and add it (w/offset) to string_len
                    i += 1;
            }
            let mut j = 0;
            i += 1;
            while j < string_len {
                ret[j] = data[i];
                j += 1;
                i += 1;
            }
            return ret;
        }

        // lists
        'l' => { 
            if data[data_len-1] == 'e' { 
                // recurse
                be_bencode(data[1..data_len-2], data_len-2)
            } else {
                // error
                println!("List should end in e");
                []
            }
        } // TODO

        // dictionaries
        'd' => { 
            if data[data_len-1] == 'e' {
                // same as 'l'
                be_bencode(data[1..data_len-2], data_len-2)
            } else {
                // error
                println!("Dictionary should end in e");
                []
            }
        } // TODO

        // else: error
        _ => { println!("error: bencoded string not recognized"); }
    }

    ret

}

