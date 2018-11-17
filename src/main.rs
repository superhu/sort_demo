extern crate rand;

//use rand::Rng;
use rand::prelude::*;
//use std::convert::TryInto;

fn main() {
//    let  arr =  &mut [2,3,1,5,6,4];

//    sort(arr);
    gen_code();

}

#[warn(dead_code)]
fn sort(arr: &mut [i32])->&mut [i32]{
    println!("before:{:?}",arr);
    let mut min;
    for i in 0..arr.len(){
        println!("{}",arr[i]);
        min = i;
        for j in (i+1)..arr.len(){
            if arr[j] < arr[min]{
                min = j;
            }
        }
        let temp = arr[i];
        arr[i] = arr[min];
        arr[min]=temp;
    };
    println!("after:{:?}",arr);
    arr

}


fn gen_code() -> String{

    let pool1 = (1..36).collect::<Vec<i32>>();

    println!("{:?}",pool1);

    let p1 = &mut gen_num(1,36,5);

    let p2 = &mut gen_num(1,13,2);

    p1.sort();


    p2.sort();



    println!("{:?}",p1);

    println!("{:?}",p2);
    //补零
    let mut pp1 =vec![];
    for &mut i in p1{
        if i <  10 {
            pp1.push("0".to_string() + &i.to_string());
        }else{
            pp1.push("".to_string() + &i.to_string());
        }
    }
    println!("pp1:{:?}",pp1);


    let mut pp2 =vec![];
    for &mut i in p2{
        if i <  10 {
            pp2.push("0".to_string() + &i.to_string());
        }else{
            pp2.push("".to_string() + &i.to_string());
        }
    }
    println!("pp2：{:?}",pp2);

    let result = pp1.join(" ").to_string() +"_"+ &pp2.join(" ");


    println!("result：{:?}",result);
    result



}



fn gen_num(start:i32,end:i32,size:usize)->Vec<i32>{
    let mut list1 = Vec::new();


    loop{
        let n1 = rand::thread_rng().gen_range(start,end);
        if !list1.contains(&n1) {
            list1.push(n1);
        }
        if list1.len()==size {
            break;
        }
    }
    list1
}




#[test]
fn test_sort(){
    let  arr =  &mut [2,3,1,5,6,4];

    assert_eq!([1,2,3,4,5,6],sort(arr))
}

#[test]
fn test_gen_code(){
    println!("{}",gen_code());
}