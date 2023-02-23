pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...)")
    }
}

pub fn notify<T: Summary>(item: &T) -> String {
    println!("Running notify!");
    format!("Breaking news! {}", item.summarize())
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let news_article = NewsArticle{
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburg, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", news_article.summarize());
        println!("Output of notify: {}", notify(&news_article));

        let tweet = Tweet{
            username: String::from("jack"),
            content: String::from("blah blah"),
            reply: false,
            retweet: false,
        };

        println!("New tweet available! {}", tweet.summarize());
    }
}
