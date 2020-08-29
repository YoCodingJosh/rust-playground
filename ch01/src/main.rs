fn main() {
    println!("Let's play around a bit with logic and repetition.");

    // let declares a variable as immutable.
    let x:i8 = 10;
    
    // but if you declare it as let mut, it's just a normal variable declaration
    let mut y:i8 = 13;

    // println is kinda like printf but is a bit better because you don't have to figure out which % to use,
    //  also supports positional params.
    println!("x is {} and y is {}", x, y);

    // if statements just like C(++/#)/Java(Script) but without parentheses.
    if x % 2 == 0 {
        println!("x is even!");
    } else {
        println!("x is odd!");
    }

    // for loops by default use an interator of i32, also supports ranges (inclusive ranges are x..=y)
    // pretty much like any ol' for-each loop but without parentheses.
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else {
            if i % 3 == 0 {
                println!("Fizz");
            } else if i % 5 == 0 {
                println!("Buzz");
            } else {
                println!("{}", i);
            }
        }
    }

    y = 1;

    // Just your normal while loop.
    while y < 100 {
        if y % 2 == 0 {
            println!("Pizza {}", y);

            y += 3;
        } else {
            println!("Sushi {}", y);

            // Rust doesn't have y++ or ++y.
            y += 1;
        }

        if y == 77 {
            println!("wow such cool {}", y);
            // Like your normal while loop, you can break/continue
            break;
        }
    };

    y = 0;

    // Rust also has a special loop called "loop" that is for infinite loops.
    loop {
        y += 1;

        println!("{}", y);

        if y % 7 == 0 {
            for z in y..=y+3 {
                // You were probably wondering... "What about switch statements?"
                // Rust has them but they're a bit weird coming from C/C++/JavaScript/C#/Java
                match z {
                    z if z == y + 3 => println!("baa {}", z),
                    z if z == y + 2 => println!("moo {}", z),
                    z if z == y + 1 => println!("woof {}", z),
                    z if z == y + 0 => println!("meow {}", z),
                    _ => println!("Ekke Ekke Ekke Ekke Ptang Zoo Boing! {}", z) // This is unreachable, but Rust complains if this ain't here.
                }
            }
        }

        if y == 100 {
            // Like with any other loop, you can break out easily from this one as well.
            break;
        }
    }
}
