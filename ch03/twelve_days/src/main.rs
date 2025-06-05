fn main() {
    let gifts=[
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];
    for day in 1..=12 {
        let suffix;
        match day {
            1 => suffix = "st",
            2 => suffix = "nd",
            3 => suffix = "rd",
            _ => suffix = "th",
        }
        println!("On the {day}{suffix} day of Christmas,\nmy true love gave to me");
        for gift_index in (0..day).rev() {
            println!("{}", gifts[gift_index]);
        }
        println!();
    }
}
