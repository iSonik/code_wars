// DESCRIPTION:
// Your job is to write a function which increments a string, to create a new string.

// If the string already ends with a number, the number should be incremented by 1.
// If the string does not end with a number. the number 1 should be appended to the new string.
// Examples:

// foo -> foo1

// foobar23 -> foobar24

// foo0042 -> foo0043

// foo9 -> foo10

// foo099 -> foo100

// Attention: If the number has leading zeros the amount of digits should be considered.


// SOLUTION:
fn increment_string(s: &str) -> String {


    let mut string = String::from("");
    let mut numbers = String::from("");
    let mut zero_count = 0;
    
    for c in s.chars() {
    
        if c.is_numeric(){
            if c.to_string() == "0".to_string() {
                numbers.push(c);
    
                zero_count = zero_count + 1
            }
    
            if c.to_string() != "0".to_string() {
                numbers.push(c);
    
            }
        }
        if !c.is_numeric(){
                string.push(c)
        }
    }
    
    if !numbers.is_empty() {
    
        let number: u32 = numbers.parse().unwrap();
        println!("Number:{:?}", number);
    
    numbers = (numbers.parse::<i32>().unwrap()+1).to_string();
    
    if  number == 0 {
        zero_count = zero_count - 1
    
    }
    
    if number >= 9 && number -1 < number {
        zero_count = zero_count - 1
    
    }
    
    
    
    println!("Zeros:{}", zero_count);
    
    
    for c in 0..zero_count {
        println!("pushing string");
        let v = vec!["0".to_string(), numbers];
        numbers = v.concat();
    
    }
    }
    if numbers.is_empty() {
        println!("number is empty");
        numbers = "1".to_string();
    }
    
    
    
    
    let v = vec![string, numbers.to_string()];
    
        return v.concat()
    
    }
    
// TEST:
#[cfg(test)]
mod tests {
    use super::increment_string;

    fn dotest(s: &str, expected: &str) {
        let actual = increment_string(s);
        assert!(actual == expected, 
            "Test failed with s = \"{s}\"\nExpected \"{expected}\"\nBut got \"{actual}\"")
    }
    
    #[test]
    fn sample_tests() {
        dotest("foo", "foo1");
        dotest("foobar001", "foobar002");
        dotest("foobar1", "foobar2");
        dotest("foobar00", "foobar01");
        dotest("foobar99", "foobar100");
        dotest("foobar099", "foobar100");
        dotest("", "1");
    }
}