use futures::executor::block_on;

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn async_main() {
    let one_future = one();
    let second_future = two();
    futures::join!(one_future, second_future);
}

fn main() {
    block_on(async_main());
}
