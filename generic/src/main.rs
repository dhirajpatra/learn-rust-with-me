// Importing the necessary items from the generic module lib.rs
use generic::{NewsArticle, SocialPost, Summary};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest(&number_list);
    println!("The largest number is {}", largest);

    let largest_i32 = largest_i32(&number_list);
    println!("The largest i32 number is {}", largest_i32);

    // Create an instance of SocialPost
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", post.summarize());

    // Create an instance of NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("K. R."),
        content: String::from("The Pittsburgh Penguins are the best hockey team in the NHL."),
    };
    println!("New article: {}", article.summarize());
}

// Non-generic function to find the largest i32 in a list
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list.iter() {
        // Compare each item with the current largest item which are both &i32 references
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

// Generic function to find the largest element in a list Parameterized over type T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Function that returns a type implementing the Summary trait
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("K. R."),
//             content: String::from("The Pittsburgh Penguins are the best hockey team in the NHL."),
//         }
//     }
// }
