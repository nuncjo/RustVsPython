use std::fs::File;
use std::error::Error;
use std::io::Write;


fn raise_error() {
    let x = 1;
    let y = 0;
    if y == 0 {
        panic!("Division by zero occured, exiting");
    } else {
        print!("{} / {} = {}", x, y, x/y);
    }
}


fn div(a: i32, b:i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("b can't be 0")
    } else {
        Ok(a/b)
    }
}


fn handle_error() {
    match div(1, 0) {
        Ok(_result) => println!("Result: {:?}", _result),
        Err(error) => println!("An error occured: {:?}", error),
    };
}


fn handle_error_question_mark() -> Result<String, Box<Error>> {
    try!(File::create("src/example_try.txt")).write_all(b"Just a test!");
    // question mark is shorter try!
    File::create("src/example.txt")?.write_all(b"Just another test!");
    /*
    works like:
    match File::create("src/example.txt") {
        Ok(t) => t.write_all(b"Just another test!"),
        Err(e) => return Err(e.into()),
    }
    */
    Ok("Done".into())
}


fn custom_error() {
    // To be extended i the future
}


fn main() {
    handle_error();
    handle_error_question_mark();
    raise_error();
    //custom_error();
}
