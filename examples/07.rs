use futures::executor::block_on;
use futures::stream;
use futures::StreamExt;

fn main() {
    // Define an async function that uses a stream
    async fn run_stream() {
        let mut stream = stream::iter(vec![1, 2, 3]);

        while let Some(item) = stream.next().await {
            println!("{}", item);
        }
    }

    // Run the async function using `block_on`
    block_on(run_stream());
}
