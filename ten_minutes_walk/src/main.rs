// DESCRIPTION:
// You live in the city of Cartesia where all roads are laid out in a perfect grid. 
// You arrived ten minutes too early to an appointment, so you decided to take the opportunity to go for a short walk. 
// The city provides its citizens with a Walk Generating App on their phones -- everytime you press the button it sends you an array of one-letter strings representing directions to walk (eg. ['n', 's', 'w', 'e']). You always walk only a single block for each letter (direction) and you know it takes you one minute to traverse one city block, so create a function that will return true if the walk the app gives you will take you exactly ten minutes (you don't want to be early or late!) and will, of course, return you to your starting point. Return false otherwise.

// Note: you will always receive a valid array containing a random assortment of direction letters ('n', 's', 'e', or 'w' only). It will never give you an empty array (that's not a walk, that's standing still!).

// SOLUTION:
fn is_valid_walk(walk: &[char]) -> bool {
    let mut n = 0;
    let mut s = 0;
    let mut w = 0;
    let mut e = 0;


  
    if walk.len() == 10 {
        for val in walk.iter() {
            
            if val.to_string() == "n"{
                n = n + 1; 
            }
               if val.to_string() == "s" {
                s = s + 1;
            }
               if val.to_string() == "w" {
                w = w + 1;
            }
               if val.to_string() == "e" {
                e = e + 1;
            }
        }
        
    if n == s && e == w {
        return true;
        }
        
    else {
        return false;
        }
    }
    else {
        return false;
    }


}
#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(  is_valid_walk(&['n','s','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['w','e','w','e','w','e','w','e','w','e','w','e']));
        assert!(! is_valid_walk(&['w']));
        assert!(! is_valid_walk(&['n','n','n','s','n','s','n','s','n','s']));
        assert!(! is_valid_walk(&['e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's']))
    }
}
