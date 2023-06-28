// 猜猜看？ 输出到多少？
fn how_far_can_baby_thread_run_after_daddy_thread_born_it_in_one_nanosecond(){
    let handle = thread::spawn(move || {
        println!("Hello I am a daddy thread");
        thread::spawn(|| {
            let mut i = 0;
            loop {
                i+=1;
                println!("{}",i)
            }
        })
    });

    handle.join().unwrap();
    thread::sleep(Duration::from_nanos(1));
}
