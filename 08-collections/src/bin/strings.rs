use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut s = String::from("Hello");
    s.push_str(" World");
    s.push('!');
    
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    format!("{}{}", s1, s2);
    let s3 = s1 + &s2;

    // rust uses UTF-8 by default 
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते"); 


    // the behavior of this loop might be unexpected
    // but this grapheme cluster is made up of more characters than visible
    // 4 grapheme cluster(s): ["न", "म", "स्", "ते"]
    // each character can be made up of multiple scalar values
    // 6 scalar values: ['न', 'म', 'स', '्', 'त', 'े']
    // or in technical terms: each letters is made up of multiple bytes
    // instead of one
    // 18 bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // rust interprets chars as scalar values which means you get 6 letters as output instead of 4
    let hello = String::from("नमस्ते");
    for char in hello.chars() {
        println!("{}", char);
    }
    // or we can iterate over the bytes
    for byte in hello.bytes() {
        println!("{}", byte);
    }
    // with a third party package we can iterate over the graphemes as well
    for grapheme in hello.graphemes(true) {
        println!("{}", grapheme);
    }
}