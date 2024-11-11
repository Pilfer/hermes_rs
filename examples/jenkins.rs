use hermes_rs::jenkins::hash_string;

fn main() {
  /*
  The Jenkins hash of 'a + b = ' is: 1959788596
  The Jenkins hash of 'add' is: 1492819387
  The Jenkins hash of 'add() function called' is: 970028757
  The Jenkins hash of 'global' is: 615793799
  The Jenkins hash of 'print' is: 2794059355                <---- this built-in type is present if we use print.
  The Jenkins hash of 'a - b = ' is: 3187819469
  The Jenkins hash of 'sub' is: 3122764749
  The Jenkins hash of 'sub() function called' is: 4183975178
   */
  let strings = ["a + b = ", "add", "add() function called", "global", "print", "a - b = ", "sub", "sub() function called"];
  for string in strings.iter() {
    let hashed_value = hash_string(string);
    println!("The Jenkins hash of '{}' is: {}", string, hashed_value);
  }
  // let input = "global"; // 2794059355
  // let hashed_value = hash_string(input);
  // println!("The Jenkins hash of '{}' is: {}", input, hashed_value);
}
