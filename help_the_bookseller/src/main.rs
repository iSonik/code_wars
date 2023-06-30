// A bookseller has lots of books classified in 26 categories labeled A, B, ... Z. Each book has a code c of 3, 4, 5 or more characters. The 1st character of a code is a capital letter which defines the book category.

// In the bookseller's stocklist each code c is followed by a space and by a positive integer n (int n >= 0) which indicates the quantity of books of this code in stock.

// For example an extract of a stocklist could be:

// L = {"ABART 20", "CDXEF 50", "BKWRK 25", "BTSQZ 89", "DRTYM 60"}.
// or
// L = ["ABART 20", "CDXEF 50", "BKWRK 25", "BTSQZ 89", "DRTYM 60"] or ....
// You will be given a stocklist (e.g. : L) and a list of categories in capital letters e.g :

// M = {"A", "B", "C", "W"} 
// or
// M = ["A", "B", "C", "W"] or ...
// and your task is to find all the books of L with codes belonging to each category of M and to sum their quantity according to each category.

// For the lists L and M of example you have to return the string (in Haskell/Clojure/Racket/Prolog a list of pairs):

// (A : 20) - (B : 114) - (C : 50) - (W : 0)
// where A, B, C, W are the categories, 20 is the sum of the unique book of category A, 114 the sum corresponding to "BKWRK" and "BTSQZ", 50 corresponding to "CDXEF" and 0 to category 'W' since there are no code beginning with W.

// If L or M are empty return string is "" (Clojure/Racket/Prolog should return an empty array/list instead).

// Notes:
// In the result codes and their values are in the same order as in M.
// See "Samples Tests" for the return.


// Solution:
use std::collections::HashMap;
use itertools::Itertools;  // itertools = "0.8"



fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut counted_books: HashMap<char, i32> = HashMap::new();

    
    if list_art.is_empty() || list_cat.is_empty() {
        return "".to_string();
    }
    else {
    for cat in list_cat.iter() {
        counted_books.insert(cat.chars().nth(0).unwrap(), 0);
    }
    for art in list_art.iter() {
        let letter = art.chars().nth(0).unwrap();
        let count = art.split(" ").nth(1).unwrap();
    match counted_books.get_mut(&letter) {
    None => () ,
    _ => *counted_books.get_mut(&letter).unwrap()+= count.parse::<i32>().unwrap()
    }
        }
    let  mut final_string: String = "".to_string();   
    for (i, x) in list_cat.iter().enumerate() {
        let n = x.chars().nth(0).unwrap();
        
        if i > 0  {
        final_string.push_str(&format!(" - ({} : {})", &n , &counted_books[&n]));
    }   
        else {
                final_string.push_str(&format!("({} : {})", &n , counted_books[&n]));
}
        }
    return final_string;
        }
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

    }
}
