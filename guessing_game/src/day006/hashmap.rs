use std::collections::HashMap;
fn main() {

    let mut scores = HashMap :: new();

   scores.insert(String::from("Blue"), 10);
   scores.insert(String::from("Yellow"), 50);

   let teams  = vec![String::from("Blue"), String::from("Yellow")];
   let initial_scores = vec![20, 60];

   // HashMap<_, _> key and value's type based on the type of vector.
   let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

   println!("{:?}", scores);

   let field_name = String::from("Favorite color");
   let field_value = String::from("Blue");

   let mut map = HashMap::new();
   map.insert(field_name, field_value);

   insert_values_no_key();

   counting_occurrences_of_word();

} 

fn insert_values_no_key () {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn counting_occurrences_of_word () {
    let text = "hello world wonderful world world";
    let mut map = HashMap :: new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
