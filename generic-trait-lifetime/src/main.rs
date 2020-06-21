extern crate generic_trait_lifetime;
use generic_trait_lifetime::NewsArticle;
use generic_trait_lifetime::Summary;
use generic_trait_lifetime::Tweet;
use generic_trait_lifetime::notify;

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    //============================================
    println!("=================================");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    //============================================
    println!("=================================");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    //============================================
    println!("=================================");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());
    //============================================
    println!("=================================");
    notify(article);
    notify(tweet);
    //============================================
    //println!("=================================");
    //{
    //    let r;
    //    {
    //        let x = 5;
    //        // if comment in, can not compile
    //        //r = &x;
    //    }
    //    println!("r: {}", r);
    //}
    //============================================
    println!("=================================");
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    //============================================
    println!("=================================");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i;
    {
        {
            i = ImportantExcerpt { part: first_sentence };
        }
        println!("{:?}", i);
    }
    println!("{:?}", i);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
