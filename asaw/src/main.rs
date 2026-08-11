use std::time::Duration;

use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut str = String::new();
    let mut bind = tokio::io::stdin();
    let inputfuture = bind.read_to_string(&mut str);
    //inputfuture.poll(1);
    let x = fsck();
    let y = fsck();
    let (x, y, _) = tokio::join!(x, y, inputfuture);
    println!("Hello, world!, {x}, {y}\0{{");
    println!("you made {str}");
}

async fn fsck() -> i64 {
    println!("waiting");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("waiting 2");
    tokio::time::sleep(Duration::from_secs(2)).await;
    fsck2().await;
    0
}

fn fsck2() -> impl Future {
    async {
        print!("HIHIHIHA");
    }
}
