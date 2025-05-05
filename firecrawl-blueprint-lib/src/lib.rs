// Declare and re-export context
pub mod context;
pub use context::{BlueprintEnvironment, Context};

// Declare jobs module and re-export handlers and IDs
pub mod jobs;
pub use jobs::{
    batch_scrape::{JOB_BATCH_SCRAPE_ID, handle_batch_scrape},
    crawl::{JOB_CRAWL_ID, handle_crawl},
    map::{JOB_MAP_ID, handle_map},
    scrape::{JOB_SCRAPE_ID, handle_scrape},
};

// Remove old example code
// pub const SAY_HELLO_JOB_ID: u32 = 0;
// #[derive(Clone)]
// pub struct MyContext { ... }
// impl MyContext { ... }
// pub async fn say_hello(...) -> TangleResult<String> { ... }

// Remove old tests related to say_hello
// #[cfg(test)]
// mod tests { ... }
