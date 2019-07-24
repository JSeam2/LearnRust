fn main() {
    // In general {} is replaced by arguments
    println!("{} is in the args", "lol");

    // Positional arguments
    println!("{0} meet {1}. {1} meet {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
             subject="1",
             verb="2",
             object="3");

    // Special formatting using :
    // format using binary
    println!("Normal: {}, Binary: {:b}", 2, 2);

    // Right Align with Specific width
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra 0
    println!("{number:>0width$}", number=1, width=6);


    /*
     * WONT PRINTS
     */
    // It will even check to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure which contains an `i32`. Name it `Structure`.
    // #[allow(dead_code)]
    // struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
