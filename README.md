# yesorno

Useful for validating answers of a CLI prompt.

## Usage

```rs
extern crate yesorno;

fn main() {
  
  let is_yes = yesorno::is_yes("yes"); 
  println!("Is Yes ? : {}", is_yes);

  let is_no = yesorno::is_no("no"); 
  println!("Is No ? : {}", is_no);

  let is_lenient_yes = yesorno::is_lenient_yes("yrs"); 
  println!("Is lenient Yes ? : {}", is_lenient_yes);

  let is_lenient_no = yesorno::is_lenient_no("ni"); 
  println!("Is lenient No ? : {}", is_lenient_no);

  let is_yes_false = yesorno::is_yes("no"); 
  println!("Is Yes ? : {}", is_yes_false);
}
```

## What is lenient ?

Use key distance based score to leniently accept typos of `yes` and `no`. This is slightly simple
the original algorithm solved here in [perl](https://metacpan.org/release/KRBURTON/String-KeyboardDistance-1.01/view/KeyboardDistance.pm)

>  Keyboard distance for fuzzy string matching. Keyboard distance is a measure of the physical distance between two keys on a keyboard. For example, 'g' has a distance of 1 from the keys 'r', 't', 'y', 'f', 'h', 'v', 'b', and 'n'. Immediate diagonals (like ''r, 'y', 'v', and 'n') are considered to have a distance of 0.75 and others are considered 0.25.

[Similar Question](https://stackoverflow.com/questions/50144/a-good-algorithm-similar-to-levenshtein-but-weighted-for-qwerty-keyboards)
