#[derive(Clone)] // allow this to be cloned so can be initialized at new function
struct KeyValuePair {
    key: String, // canonical key
    values: Vec<String>, // a list of anagrams
}

#[derive(Default)]
pub struct HashMap {
    buckets: Vec<Vec<KeyValuePair>>,
}

impl HashMap {
    fn get_alphabet_position(c: char) -> usize {
        let lower = c.to_ascii_lowercase();
        let position = (lower as u32) - ('a' as u32) + 1;
        position as usize
    }

    fn create_canonical_key(word: &str) -> String {
        let mut letters: Vec<char> = word.chars().collect();
        letters.sort_unstable();
        letters.into_iter().collect()
    }

    fn hash(&self, key: &str) -> usize {
        let prime_number: usize = 31; // a prime number helps to scramble better the input
        let mut hash_value: usize = 0;
        for c in key.chars() {
            let alphabet_position = Self::get_alphabet_position(c);
            // Horner's Method
            hash_value = (hash_value * prime_number) + alphabet_position;
        }
        
        hash_value % self.buckets.len()
    }

    pub fn new(size: usize) -> Self {
        Self {
            buckets: vec![Vec::new(); size],
        }
    }

    pub fn insert(&mut self, word: &str) {
        let key = Self::create_canonical_key(word);
        let bucket_index = self.hash(&key);

        // Search inside the corresponding bucket
        for pairs in self.buckets[bucket_index].iter_mut() {
            // If find a corresponding key, insert inside the anagram list
            if pairs.key == key {
                pairs.values.push(String::from(word));
                return
            }
        }

        // If didnt find, create a new group and insert inside the corresponding bucket
        let new_pair = KeyValuePair {
            key,
            values: vec![String::from(word)],
        };
        self.buckets[bucket_index].push(new_pair);
    }

    pub fn get_anagram_groups(&self) -> Vec<& Vec<String>> {
        let mut groups = Vec::new();

        for bucket in self.buckets.iter() {
            for pair in bucket.iter() {
                // if there is more then one word is a group of anagrams
                if pair.values.len() > 1 {
                    groups.push(&pair.values);
                }
            }
        }

        groups
    }
}

