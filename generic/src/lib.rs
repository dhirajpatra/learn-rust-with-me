// Define a struct named `NewsArticle` with relevant fields
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Define a trait named `summary` with a method `summarize`
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Method to summarize content only signature
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement the `summary` trait for the `NewsArticle` struct
impl Summary for NewsArticle {
    // Provide the implementation for the `summarize` method
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Define a struct named `SocialPost` with relevant fields
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement the `Summary` trait for the `SocialPost` struct
impl Summary for SocialPost {
    // Provide the implementation for the `summarize_author` method
    fn summarize_author(&self) -> String {
        format!("@{}", &self.username)
    }

    // Provide the implementation for the `summarize` method
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", &self.username)
    }
}

// Function that takes any item implementing the Summary trait using trait as a parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Generic function that takes any item implementing the Summary trait using trait bounds
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

