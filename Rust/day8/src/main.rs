fn main() {
    let input = include_str!("../input_small.txt");
    for raw_inputs in input.split('|') {
        println!("raw_inputs: {:?}", raw_inputs);
    }
}
