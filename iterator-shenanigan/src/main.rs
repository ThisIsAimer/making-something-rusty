fn main() {
    let colors = vec![
        "red".to_string(),
        "blue".to_string(),
        "green".to_string(),
    ];

    let mut colors_iter = colors.iter();


    println!("{:#?}", colors_iter);
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());

}
