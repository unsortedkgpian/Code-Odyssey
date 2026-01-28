// agenda 
// Trie -> focus on tiee

use std::collections::{HashMap, HashSet};
use fxhash::FxBuildHasher;

type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

#[derive(Debug, Default)]
struct Node {
    at_end:bool,
    children: HashMap<u8, Node>,
}

#[derive(Debug, Default)]
struct Trie{
    root: Node, 
    len: usize,
}

impl Trie{
    fn new() -> Self{
        Trie::default()
    }

    fn insert(&mut self, text:&str) {
        let mut current_node = &mut self.root;

        for c in text.bytes() {
            current_node = current_node.children.entry(c).or_default()
        }

        current_node.at_end = true;
        // not confirm 
        self.len +=1;
    }

    fn contains(&mut self, text:&str) -> bool{
        let mut  current_node = &self.root;
        for c in text.bytes() {
            match current_node.children.get(&c){
                Some(node) => current_node = node,
                None =>  return false,
            }
        }
        current_node.at_end
        // true
    }

    fn len(&self) -> usize{
        self.len
    }

}

fn main() {

    // let mut urls = HashSet::new();
    // urls.insert("https://www.portfolio.unsortedbytes.in/");
    // urls.insert("https://www.portfolio.amazon.in/");
    // urls.insert("https://www.portfolio.google.in/");

    // let contains_reddit = urls.contains("https://www.reddit.com/r/rust");

    // println!("Does urls  dontain Reddit?\n{contains_reddit}");


    let mut urls = Trie::new();
    println!("{urls:#?}");
    urls.insert("https://www.portfolio.unsortedbytes.in/");
    urls.insert("https://www.portfolio.amazon.in/");
    urls.insert("https://www.portfolio.google.in/");

    // println!("{urls:#?}");

    let contains_stub = urls.contains("https://www");

    println!("{}   {}", contains_stub, urls.len());
}
