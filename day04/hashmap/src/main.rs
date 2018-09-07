
fn main() {
    // use std::collections::HashMap;
    
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"),0);
    // scores.insert(String::from("Yellow"),20);


// 用teams vec和initial_scores vec创建HashMap;
    // use std::collections::HashMap;

    // let teams = vec![String::from("Blue"),String::from("Yellow")];

    // let initial_scores = vec![10,50];

    // let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // use std::collections::HashMap;

    // let field_name = String::from("Fav color");

    // let field_value = String::from("blue");

    // let mut map = HashMap::new();

    // map.insert(field_name,field_value);

// 访问hashmap中存储的蓝队的分数
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"),10);
    // scores.insert(String::from("Yellow"),20);

    // let team_name = String::from("Blue");
    // let socre = scores.get(&team_name);

//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Yellow"),20);
//     scores.insert(String::from("Blue"),40);
// // 会以任意顺序打印出每一个键值对
//     for(key,value) in &scores{
//         println!("{}:{}",key,value);
//     }
    
    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"),20);
    // scores.insert(String::from("Blue"),50);

    // println!("{:?}",scores);

    // use std::collections::HashMap;

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"),10);
    
    // // 如果有值就不插入，只在没有值的时候插入
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(33);

    // println!("{:?}",scores);

    // 通过hashmap存储单词和计数来统计出现次数
    // use std::collections::HashMap;

    // let text = "Hello world w world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}",map);
}
