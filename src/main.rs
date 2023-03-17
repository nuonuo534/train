mod example;
mod utils;
// use std::collections::HashMap;
// use utils::format::string_variable_content;
use example::{algorithm, cli, tar};
// use utils::format::{string_variable_content2, string_variable_content1};
// use utils::class::{test};

fn main() {
    // let str = "可插入以下标签至内容文本中：获奖教师：[[[teacher]]]、项[item]目名称：[item]、奖[a]项名称：[award]";
    // let map = HashMap::from([
    //     ("teacher", "1"),
    //     ("item", "2"),
    //     ("c", "3"),
    // ]);
    // let reg_str = string_variable_content1(&str, map);
    // println!("{} {}",std::mem::size_of_val(&reg_str), reg_str.len());
    // test();
    // println!("{:?}", char_str);
    // // words.push_words(['可插入以', '内容'])
    // println!("{}", reg_str);

    println!("{:?}", algorithm::create_rng::<f64>());
    println!("{:?}", algorithm::create_rng_scope(1, 3));
    println!("{:?}", algorithm::create_rng_point());
    println!("{:?}", algorithm::create_rng_str(32));
    let mut arr = vec![3, 1, 4];
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);
    cli::create_cli();

    tar::decode_file("./public/test.tar.gz");
    tar::tar_file("./public/test.md");
}
