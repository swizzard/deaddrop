use deaddrop::{get_once, insert, RawKey};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let n = 5;
    let db = Arc::new(sled::open("db").unwrap());
    for i in 0..n {
        let db = Arc::clone(&db);
        tokio::spawn(async move {
            let k = format!("key{}", i);
            let pwd = format!("password{}", i);
            let text = format!("text{}", i);
            let key = RawKey::new(k.as_bytes(), pwd.as_bytes());
            println!("insert k {} pwd {} text {}", k, pwd, text);
            insert(&db, &key, text.as_bytes()).unwrap();
        })
        .await
        .unwrap();
    }
    for i in 0..n {
        let db = Arc::clone(&db);
        tokio::spawn(async move {
            let k = format!("key{}", i);
            let pwd = format!("password{}", i);
            println!("get k {} pwd {}", k, pwd);
            let key = RawKey::new(k.as_bytes(), pwd.as_bytes());
            let res = get_once(&db, &key).unwrap();
            println!("get res {:?}", res);
        })
        .await
        .unwrap();
    }
}
