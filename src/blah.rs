mod lib;

use lib::list_tables;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    list_tables().await;
}
