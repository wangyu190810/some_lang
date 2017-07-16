use time;
use time::Timespec;
use mysql as my;
// create table tb_fileupload(

// fseq int primary key auto_increment, #自增序列
// fusername varchar(50),   #上传者
// fhashcode varchar(100),  #hash码
// fsize int, #文件大小
// fblocks int, #块数
// ftype varchar(50),  #文件类型 
// fready varchar(1024),  #已上传完成的块编号
// finerpath varchar(200), #内部存储路径
// fouterpath varchar(200), #外部存储路径
// fisfinished int default 0,  #要否上传完成
// ftime datetime #创建时间
// )



#[derive(Debug, PartialEq, Eq)]
struct FCopy {
    fseq: i32,
    fusername: String,
    fhashcode: String,
    fsize: i32,
    fblocks: i32,
    ftype: String,
    fready: String,
    finerpath: String,
    fouterpath: String,
    fisfinished: i8, 
    ftime: Timespec,
}



pub fn run() {
    let pool = my::Pool::new("mysql://root:2016@localhost:3306").unwrap();
    pool.prep_exec("create table tb_fileupload(
                fseq int primary key auto_increment, 
                fusername varchar(50),  
                fhashcode varchar(100),  
                fsize int, 
                fblocks int, 
                ftype varchar(50),  
                fready varchar(1024),  
                finerpath varchar(200), 
                fouterpath varchar(200), 
                fisfinished int default 0,  
                ftime datetime 
                ) DEFAULT CHARSET=UTF8", ()).unwrap();

//     let fc = FCopy {
//         fseq: 1,
//         fusername: "test".to_string(),
//         fhashcode: "fhashcode".to_string(),
//         fsize: 10,
//         fblocks: 100,
//         ftype: "exe".to_string(),
//         fready: "asdfa".to_string(),
//         finerpath: "/a/b".to_string(),
//         fouterpath: "/b/c".to_string(),
//         fisfinished: 1,
//         ftime: time::get_time(),
//     };
//     conn.execute("INSERT INTO tb_fileupload 
//                   VALUES (?1, ?2, ?3,?4, ?5,
//                   ?6,?7,?8,?9,?10,
//                   ?11)",
//                  &[&fc.fseq,&fc.fusername, &fc.fhashcode, &fc.fsize, &fc.fblocks, 
//                  &fc.ftype, &fc.fready, &fc.finerpath, &fc.fouterpath, &fc.fisfinished, 
//                  &fc.ftime]).unwrap();

//     let mut stmt = conn.prepare("SELECT fusername FROM tb_fileupload").unwrap();
//   //  let person_iter = stmt.query_map(&[], |row| {
//   //      FCopy {
  //          fusername : row.get(0),
  //      }
  //  }).unwrap();

  //  for person in person_iter {
  //      println!("Found person {:?}", person.unwrap());
  //  }
}