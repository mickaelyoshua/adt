use adt::hashing::HashMap;

fn main() {
    let words = vec!["listen", "silent", "enlist", "hello", "world", "thing"];

    // Choose a size for your hash map (a prime number is often a good choice).
    let mut hash_map = HashMap::new(101);

    for word in words {
        hash_map.insert(word);
    }

    let anagrams = hash_map.get_anagram_groups();
    println!("{:?}", anagrams);
}
