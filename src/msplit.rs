// use stfu8;
// use unicode_segmentation::UnicodeSegmentation;

pub fn chars(input: &str) -> Vec<&str> {
    if input.is_empty(){
        return vec![""]

    }
    input.split_terminator("").skip(1).collect::<Vec<_>>()
}