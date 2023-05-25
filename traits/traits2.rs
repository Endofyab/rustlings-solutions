// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar  {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {
    // TODO: Implement `AppendBar` for type `String`.
    //approach 1 
    // fn append_bar(self) -> Self {
        // let mut cloned_self = self.clone();
        // cloned_self.push("Bar".to_string());
        // cloned_self
    // }
    
    //apprach 2 
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar")); // ("Bar".to_string()) also works 
        self
        
        //misunderstood the problem
        // for values.iter() 
        // for i in &self {
        //     i + "Bar";
        // }
        
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
