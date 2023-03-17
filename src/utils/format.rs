use regex::Regex;
use std::collections::HashMap;

pub fn string_variable_content1(str: &str, variable: HashMap<&str, &str>) -> String {
    let keys = variable.keys();
    let mut v: Vec<String> = Vec::new();
    for key in keys {
        let k = ["\\[", key, "\\]"].join("");
        v.push(k);
    }
    let reg_str = v.join("|");
    let reg = Regex::new(&reg_str).unwrap();
    let cd = reg
        .replace_all(str, |captures: &regex::Captures| {
            let a = Regex::new(r"\[|]").unwrap().replace_all(&captures[0], "");
            let v = variable.get(&*a).unwrap();
            v
        })
        .to_string();
    cd
}

pub fn string_variable_content2(str: &str, variable: HashMap<&str, &str>) -> String {
    let mut res = String::new();
    let content = str.chars().collect::<Vec<char>>();
    let len = content.len();

    let mut start = 0;
    for i in 0..len - 1 {
        let item = content[i];
        if item == '[' {
            if start > 0 {
                let key = &content[start - 1..i];
                res += &String::from_iter(key);
            }
            start = i + 1;
        } else if start > 0 && (item == ']' || len - 1 == i) {
            let key = String::from_iter(&content[start..i]);
            let value = variable.get(&*key);
            match value {
                Some(v) => res += v,
                None => res += &String::from_iter(&content[start - 1..i + 1]),
            }
            start = 0
        } else if start == 0 {
            res.push(item)
        }
    }
    res
}
