fn main() {
    let s = String::from("Hello world");

    println!("{}", first_word(&s))
}

fn first_word(s: &str) -> &str{ //str instead of String allows for slices to get passed in. And can pass in string literals without worry. let s = "hello world";
    let bytes = s.as_bytes(); //Turns the string into bytes
    
    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' { //if byte is equal to the byte value of space return the s from 0 to index of byte
            return &s[0..i];
        }
    }

    &s[..] //else return all of s. Could just return &s instead of indexing it
}