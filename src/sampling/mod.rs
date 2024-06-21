fn fill (shape: [i32;2], filler: i32) -> Vec<Vec<i32>> {
    let mut output = vec![];
    for dimension in shape {
        output.insert(vec![filler; dimension])
    }
    output
}

