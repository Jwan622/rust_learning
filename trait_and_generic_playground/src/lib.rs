pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
    pub byline: String
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("News: {}, Content: {}, Byline: {}", self.headline, self.content, self.byline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summarizable>(item: &T) {
    println!("Breaking news! {}", item.summary());
}
/*
The angle brackets (<>) after notify indicate that the function is generic. The T inside the brackets is a placeholder for any type. When you call notify, Rust will determine what concrete type T should be based on the argument you pass.

Trait Bound T: Summarizable
The T: Summarizable syntax is a trait bound. It restricts the types that T can be to only those that implement the Summarizable trait. This ensures that any type used as T in this function has the methods defined by Summarizable, particularly the summary method.
*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_article_summary() {
        let article = NewsArticle {
            headline: String::from("Rust is awesome!"),
            content: String::from("Rust provides memory safety without garbage collection."),
            byline: String::from("this is a byline")
        };

        assert_eq!(article.summary(), "News: Rust is awesome!, Content: Rust provides memory safety without garbage collection., Byline: this is a byline");
    }

    #[test]
    fn test_tweet_summary() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Check out the new features in Rust!"),
        };

        assert_eq!(tweet.summary(), "@rustlang: Check out the new features in Rust!");
    }

    #[test]
    fn test_notify() {
        let article = NewsArticle {
            headline: String::from("Rust is awesome!"),
            content: String::from("Rust provides memory safety without garbage collection."),
            byline: String::from("this is a byline")
        };

        notify(&article); // Should print "Breaking news! News: Rust is awesome!"

        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("Check out the new features in Rust!"),
        };

        notify(&tweet); // Should print "@rustlang: Check out the new features in Rust!"
    }
}