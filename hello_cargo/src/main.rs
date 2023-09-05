fn main() {
    println!("Hello, world!");
    greet_world();
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "hello,world!";
    let regions = [southern_germany, chinese, english];

    for region in regions.iter() {
        println!("{}", &region)
    }
}