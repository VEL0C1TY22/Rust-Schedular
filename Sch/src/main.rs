use tokio;
use std::{collections::HashMap, io};


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut x = String::new();
    let mut hsh: HashMap<String, i32> = HashMap::new();
    io::stdin().read_line(&mut x).expect("Expected a number");
    let y:i32 = x.trim().parse().expect("Number not numbering");
    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for i in 0..y {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Expected a String");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Expected a Number");
        let rn:i32 = n.trim().parse().expect("Number not numbering");
        hsh.insert(s.trim_end().to_string(), rn);
        // println!("spawning number {}",i);
        // handles.push(sppawn(i));
    }
    println!("{:?}",hsh);
    let mut v: Vec<(&String, &i32)> = hsh.iter().collect();
    v.sort_by(|a,b| a.1.cmp(b.1));
    println!("{:?}",v);
    let mut last_value: Option<i32> = None;

    for (nme,value) in v.iter(){
        if let Some(prev) = last_value{
            if prev != **value{
                for handle in handles.drain(..){
                    println!("Drained thread number: {}", prev);
                    handle.await.unwrap();
                }
            }
        }
        println!("spawning name, number: {} {}", nme, **value);
        handles.push(sppawn(**value));
        last_value = Some(**value);
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    }
    for handle in handles {
        handle.await.unwrap();
    }
}
    


fn sppawn(num: i32) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        println!("Spawned thread {}", num);
    })
}

