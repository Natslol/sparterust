use std::io;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Instant, Duration};
use std::sync::Arc;

fn main() {
    println!("Shares id (met l'id de la video pas le lien pour l'instant) -> ");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Impossible de lire la ligne");
    id.pop();
    if id.contains("\r") { id.pop(); }
    share(id);
}
fn share(id: String) {
    const LIMIT_THREAD: i32 = 500;
    let count = Arc::new(AtomicUsize::new(0));
    for _ in 0..LIMIT_THREAD {
        let clone = id.clone();
        let clone_arc = count.clone();
        thread::spawn(move || loop {
            let r = ureq::agent().post("https://api19.tiktokv.com/aweme/v1/aweme/stats/?channel=tiktok_web&device_type=SM-G9900&device_id=9999999999999999999&os_version=11&version_code=220400&app_name=tiktok_web&device_platform=android&aid=1988").set("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8").query("item_id", &clone).query("share_delta", "1").call();
            if r.is_ok() {
                clone_arc.fetch_add(1, Ordering::Relaxed);
            }
        });
    }
    let now = Instant::now();
    thread::sleep(Duration::from_secs(1));
    loop {
        let time = now.elapsed().as_secs() as usize;
        winconsole::console::set_title(format!("Vitesse: {}/s, Temps: {}s", (count.load(Ordering::Relaxed) / time), time).trim()).unwrap();
    }
}
