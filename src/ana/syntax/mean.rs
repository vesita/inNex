struct Mean {
    name: String,
    mean_type: String,
    inputs: Vec<Box<Mean>>,
    action: fn(&Mean) -> i32,
}
