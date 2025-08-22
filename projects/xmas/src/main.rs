fn main() {

    let lines = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 1..13 {
        println!("On the {day} day of Christmas my true love send to me:");
        for thing in (0..day).rev() {
            println!("{}", lines[thing]);
        }
        println!("");
    }

    println!("Hello, world!");
}
