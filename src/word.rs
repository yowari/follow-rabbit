/// Each node in the trie represent a letter
pub struct Letter {
    /// character representing the letter
    character: char,
    /// next possible letters
    children: Vec<Letter>,
    /// set true if this is the last letter of a word
    is_word: bool,
}

impl Letter {

    /// Construct the `Letter` by setting the `character`.
    pub fn new(character: char) -> Self {
        Letter {
            character: character,
            children: vec![],
            is_word: false,
        }
    }

    /// Returns the character symbole of the current `Letter`.
    pub fn character(&self) -> char {
        self.character
    }

    /// Returns the mutable liste of the next possible `Letter`s.
    pub fn children_mut(&mut self) -> &mut Vec<Letter> {
        &mut self.children
    }

    /// Returns the liste of the next possible `Letter`s.
    pub fn children(&self) -> &Vec<Letter> {
        &self.children
    }

    /// Set if this is the last letter of a word
    pub fn set_word(&mut self, is_word: bool) {
        self.is_word = is_word;
    }

    /// Returns if this is the last letter of a word 
    pub fn is_word(&self) -> bool {
        self.is_word
    }

}

/// Creates trie of letters.
/// 
/// Returns the head of the trie.
pub fn build_trie(words: &Vec<String>) -> Letter {
    // Not important what `char` is choosed. This must never be used!
    const HEAD_CHARACTER: char = '_';
    let mut head = Letter::new(HEAD_CHARACTER);

    for word in words {
        build_node(word, &mut head);
    }

    head
}

/// Inserts the `word` in the trie of `Letter`s.
fn build_node(word: &String, head: &mut Letter) {
    let mut parent = head;

    for c in word.chars() {
        let old_parent = parent;
        parent = find_or_insert_node(c, old_parent);
    }

    parent.set_word(true);
}

/// Find the `Letter` containing the `character` or creates it and insert it in
/// the trie.
fn find_or_insert_node(character: char, parent: &mut Letter) -> &mut Letter {
    let children = parent.children_mut();
    let node_found = children.iter_mut().find(|node| node.character == character);

    match node_found {
        Some(node) => {
            node
        },
        None => {
            children.push(Letter::new(character));
            children.last_mut().unwrap()
        },
    }
}

#[cfg(test)]
mod test_tree {
    use super::*;

    #[test]
    fn test_build_tree_from_simple_word() {
        let words = vec![String::from("abalone")];

        let tree = build_tree(&words);

        assert_eq!(1, tree.children.len());
    }

    #[test]
    fn test_build_tree_from_multiple_words() {
        let words = vec![
            String::from("abalone"),
            String::from("convene"),
        ];

        let tree = build_tree(&words);

        assert_eq!(2, tree.children.len());
    }
}
