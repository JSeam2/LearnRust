fn main() {
    // In general {} is replaced by arguments
    println!("{} is in the args", "lol");

    // Positional arguments
    println!("{0} meet {1}. {1} meet {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
             subject="1",
             verb="2",
             object="3")

    // Special formatting using :

}
