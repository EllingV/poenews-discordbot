
use lazy_static::lazy_static;

//use crate::rsstestdata; // 1.4.0

/// A struct for `news posts` found in the `path of exile` rss feed
/// - `title` is used as, well, the title of the news post
/// - `description` is like a tldr of the post, including an embedded link
/// - `pub_date` is used to differentiate posts, letting us know if it's new or not
pub struct Post {
    pub title: String,
    pub description: String,
    pub pub_date: String,
}

/// Link to the `path of exile news` rss feed
const RSS_FEED: &str = "https://www.pathofexile.com/news/rss";

lazy_static! {
    /// This is the client used when sending `get requests`
    static ref POE_CLIENT: reqwest::Client = reqwest::Client::builder().user_agent("POE_NEWSBOT").build().expect("Err building poe_client");
}



/// fetch latest news posts using an `http get` request
///
/// return value will be a `Vec<Post>`:  
/// containing 30 posts if the response status is `ok`  
/// containing 0 posts if it isn't
pub async fn load_feed() -> Vec<Post> {

    // We will "save" the newest 30 posts in the rss feed here
    let mut posts: Vec<Post> = Vec::new();

    // Get request on poe news rss feed
    let poe_client = &POE_CLIENT;
    let response = poe_client.get(RSS_FEED)
        .send()
        .await
        .expect("Err sending get request to RSS feed");


    // If the status isn't OK, that means that RSS-feed is either down, or we got blocked
    if response.status() != reqwest::StatusCode::OK{
        println!("Error getting RSS feed!\n{}", reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap());
        println!("Returning empty posts list");
        return posts;
    }

    // Get content from rss feed
    let content = response.bytes().await.expect("Err getting contents from RSS feed as bytes");

    // Parse data from rss feed
    let channel = rss::Channel::read_from(&content[..])
        .expect("Err parsing contents from rss feed channel");

    // Loop over items parsed from feed, adding them to our posts
    for item in channel.items {
        let p = Post {
            title: item.title.unwrap(),
            description: item.description.unwrap(),
            pub_date: item.pub_date.unwrap(),
        };
        
        posts.push(p);
    }

    // Return statement
    posts

}


// A function that effectively loads "rss feed" from saved 'const &str' variables in rsstestdata.rs
// We have two of these so we can test content updates with static data
// pub async fn load_feed_testdata() -> Vec<Post> {
//     // We will "save" the newest 30 posts in the rss feed here
//     let mut posts: Vec<Post> = Vec::new();

//     let content = rsstestdata::RSS_FEED_DATA_1.as_bytes();

//     // Parse data from rss feed
//     let channel = rss::Channel::read_from(content)
//         .expect("Err parsing contents from rss feed channel");

//     // Loop over items parsed from feed, adding them to our posts
//     for item in channel.items {
//         let p = Post {
//             title: item.title.unwrap(),
//             description: item.description.unwrap(),
//             pub_date: item.pub_date.unwrap(),
//         };
        
//         posts.push(p);
//     }

//     // Return statement
//     posts
// }

// Identical to `load_feed_testdata()`, except for the `content` referring to another piece of testdata
// pub async fn load_feed_testdata2() -> Vec<Post> {
//     // We will "save" the newest 30 posts in the rss feed here
//     let mut posts: Vec<Post> = Vec::new();

//     let content = rsstestdata::RSS_FEED_DATA_2.as_bytes();

//     // Parse data from rss feed
//     let channel = rss::Channel::read_from(content)
//         .expect("Err parsing contents from rss feed channel");

//     // Loop over items parsed from feed, adding them to our posts
//     for item in channel.items {
//         let p = Post {
//             title: item.title.unwrap(),
//             description: item.description.unwrap(),
//             pub_date: item.pub_date.unwrap(),
//         };
        
//         posts.push(p);
//     }

//     // Return statement
//     posts
// }