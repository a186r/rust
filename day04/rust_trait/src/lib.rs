// pub trait Summarizable {
//     // add code here
//     fn summary(&self) -> String;
// }

// pub struct NewsArticle{
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summarizable for NewsArticle{
//     fn summary(&self) -> String{
//         format!("{},by {} ({})",self.headline,self.author,self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summarizable for Tweet {
//     fn summary(&self) -> String {
//         format!("{}:{}",self.username,self.content)
//     }
// }

// let tweet = Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from("of course .x..x.x.x..x..x");
//     reply: false,
//     retweet: false,
// }

// println!("1 new tweet: {}",tweet.summary());

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast{
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast{
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}