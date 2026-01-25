fn main() {
    let gifts = [
        "And a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];
    
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    
    for day in 0..12 {
        println!("\nOn the {} day of Christmas,", days[day]);
        println!("my true love sent to me");
        
        if day == 0 {
            println!("A partridge in a pear tree");
        } else {
            for gift_num in (0..=day).rev() {
                println!("{}", gifts[gift_num]);
            }
        }
    }
}