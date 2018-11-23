extern crate rand;

use std::fs;
use std::path::Path;

use rand::prelude::*;
use std::fs::DirEntry;
use std::io;


fn main() {
//    let  arr =  &mut [2,3,1,5,6,4];
//    sort(arr);

//    gen_code();


//    let path = Path::new("D:\\Pictures\\images\\1111");
//    let _ = visit_dirs(path, &print_filter_dir_entry);

    let money = &mut 100;
    let coins = &mut vec![1, 5, 10, 20, 50];
    change(money, coins);
}

#[allow(dead_code)]
fn print_filter_dir_entry(p: &DirEntry) {
    let filename = &p.file_name();
    let ff = filename.to_str();
    match ff {
        Some(f) =>
            if f.to_lowercase().ends_with("jpg") || f.to_lowercase().ends_with("jpeg") || f.to_lowercase().ends_with("png") {
                println!("{:?}", f)
            },
        None => (),
    }
}

///## 1.数组排序
#[allow(dead_code)]
fn sort(arr: &mut [i32]) -> &mut [i32] {
    println!("before:{:?}", arr);
    let mut min;
    for i in 0..arr.len() {
        println!("{}", arr[i]);
        min = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        let temp = arr[i];
        arr[i] = arr[min];
        arr[min] = temp;
    };
    println!("after:{:?}", arr);
    arr
}

///## 2.超级大乐透摇奖
#[allow(dead_code)]
fn gen_code() -> String {
    let p1 = &mut gen_num(1, 36, 5);
    let p2 = &mut gen_num(1, 13, 2);

    p1.sort();
    p2.sort();

    println!("{:?}", p1);
    println!("{:?}", p2);
    //补零
    let mut pp1 = vec![];
    for &mut i in p1 {
        if i < 10 {
            pp1.push("0".to_string() + &i.to_string());
        } else {
            pp1.push("".to_string() + &i.to_string());
        }
    }
    println!("pp1:{:?}", pp1);


    let mut pp2 = vec![];
    for &mut i in p2 {
        if i < 10 {
            pp2.push("0".to_string() + &i.to_string());
        } else {
            pp2.push("".to_string() + &i.to_string());
        }
    }
    println!("pp2：{:?}", pp2);

    let result = pp1.join(" ").to_string() + "-" + &pp2.join(" ");


    println!("result：{:?}", result);
    result
}


fn gen_num(start: i32, end: i32, size: usize) -> Vec<i32> {
    let mut list1 = Vec::new();

    loop {
        let n1 = rand::thread_rng().gen_range(start, end);
        if !list1.contains(&n1) {
            list1.push(n1);
        }
        if list1.len() == size {
            break;
        }
    }
    list1
}

/// ## 3.遍历文件
#[allow(dead_code)]
fn list_file(path: &str) {
    let paths = fs::read_dir(path).unwrap();


    paths
        .filter_map(Result::ok)
        .filter_map(|d| d.path().to_str().and_then(|f| if f.ends_with(".jpg") { Some(d) } else { None }))
        .for_each(|f| println!("{:?}", f));
}

/// ## 3.遍历文件
#[allow(dead_code)]
fn visit_dirs(dir: &Path, fn_print_path: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, fn_print_path)?;
            } else {
                fn_print_path(&entry);
            }
        }
    }
    Ok(())
}

///
/// m 100
/// vec [1,5,10,20,50]
///
fn change(money: &mut i32, coins: &mut Vec<i32>) -> i32 {
    let len = coins.len();
    let mut count = vec![0; len];
    println!("count:{:?}",count);

    coins.sort();

    coins.reverse();

    println!("{:?}", coins);
    for i in 0..len {
        while money > &mut coins[i] {
            *money -= coins[i];
            count[i] = count[i] + 1;
        }
        if count[i] != 0 {
            println!("{}张{}元", count[i], coins[i]);
        }
    }

    0
}


#[test]
fn test_sort() {
    let arr = &mut [2, 3, 1, 5, 6, 4];

    assert_eq!([1, 2, 3, 4, 5, 6], sort(arr))
}

#[test]
fn test_gen_code() {
    println!("{}", gen_code());
}

