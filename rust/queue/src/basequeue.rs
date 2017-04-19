
// static STATIC: Type = init; save = String::new("save")

#[derive(Debug,Default)]
pub struct queueData {
    pub data: Vec<String>,
    pub size: usize,
    pub path: String,
}



impl queueData {
    pub fn new(size: usize) -> queueData{
        // let mut queue_data:queueData = queueData::Default();
        // queue_data.size = size;
        // queue_data.data = vec![];
        // return queue_data
        queueData{
            data: vec![],
            size: size,
            path: "./".to_string()
        }
    }

    pub fn put(&mut self, value: String) -> bool {
        let mut flag = false;
        if self.data.len() < self.size {
            self.data.push(value);
            flag = true;
        }
        return flag;
    }

    pub fn get(&mut self) -> Option<String> {
        self.data.pop()
    }

    pub fn save(&self){
        println!("save data to path {}", self.path);
    }
}

pub fn base_test() {
    let mut queue = queueData::new(2);
    let world = String::from("world");
    for row in 0..4 {
        if queue.put(world.clone()) {
            println!("put success!")
        } else {
            println!("put false ! queue size is {}, queue len is {} ",
                     queue.size,
                     queue.data.len());
        }
    }
    for row in 0..queue.size {
        let get_world = queue.get();
        println!("{:?}", get_world);
    }
}

pub fn thread_test() {
    // use std::thread;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};
    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    // use std::sync::mpsc;
    // let mut  'a base_queue = Box::new(queueData::new(2));
    let mut queue = Arc::new(Mutex::new(queueData::new(1000)));
    let (tx, rx) = channel();

    // 需要调用lock之后，相当于对T做了lock，这个时候就可以调用T的方法了。
    {
        let (queue, tx) = (queue.clone(), tx.clone());
        let mut row = 1;
        let put = thread::spawn(move || loop {
            
            let world = String::from(format!("world {:}", row.to_string()));
            let mut queue = queue.lock().unwrap();
            if queue.data.len() < queue.size {
                tx.send(queue.put(world)).unwrap();
                // thread::sleep(ten_millis);
                row += 1;
                
            } else {
                tx.send(false).unwrap();
                // break;
            }
        });
        // put.join();
    }

    {
        let recv = thread::spawn(move || loop {
            // println!("{}", rx.recv().unwrap());
            // rx.recv().unwrap();
            // 同样的操纵，需要调用lock之后，相当于对T做了lock，这个时候就可以调用T的方法了，
            //如果不做lock，那样T就不是我们定义的struct。
            // let (queue, tx) = (queue.clone(), tx.clone());
            let mut queue = queue.lock().unwrap();
            // println!("{:?}", queue.get());
            // println!("queue len {:?}", queue.data.len());
        });
        // recv.join();
    }

    loop {
        // 防止主进程挂掉
        thread::sleep(ten_millis);
    }
    
    // base_queue.get();
    // let mut queue: =
    // let mut queue =  rx.recv(queue.get()).unwrap();
    // println!("{:?}");
    // let get_world = queue.get();
    // println!("{:?}", get_world);
}