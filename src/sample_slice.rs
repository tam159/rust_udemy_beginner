pub fn example_slice() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    println!("Slice: {:?}", slice);

    let mut colors = ["red", "green", "blue", "pink"];
    println!("Colors: {:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("Colors: {:?}", colors);
}

fn update_colors(colors_slice: &mut [&str]) {
    colors_slice[0] = "yellow";
    colors_slice[1] = "orange";
    // colors_slice[2] = "brown";
}
