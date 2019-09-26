//可以使用new创建一个空的HashMap，并使用insert增加元素
//记录两支队伍的分数，蓝队10分，黄队50分
use std::collections:HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

//与vector一样，哈希map将数据存储在堆上，这个HashMap的了；类型是String而值类型是i32
use std::collections:HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();