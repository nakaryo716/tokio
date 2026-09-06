use std::sync::Arc;

use tokio::runtime;

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime::Builder::build failed");

    let handle = rt.handle();
    handle.spawn(async move {
        let _a = 10 + 10;
    });

    let task = rt.spawn(async move {
        let _a = 10 + 10;
    });

    let rt = Arc::new(rt);

    let th1 = std::thread::spawn({
        let rt = rt.clone();
        move || {
            rt.block_on(async move {
                println!("thread1");
                let _a = 10 + 10;
                task.await.unwrap();
            });
        }
    });
    std::thread::spawn({
        let rt = rt.clone();
        move || {
            let a = rt.spawn(async {
                println!("isitwork?");
            });
            rt.block_on(async move {
                println!("thread2");
                let _a = 10 + 10;
            });
        }
    });

    th1.join().unwrap();
}
