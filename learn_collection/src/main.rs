use std::collections::HashMap;

fn mean(array: &Vec<i32>) -> f64{

    let sum: i32 = array.iter().sum();
    // Ниже код делает тоже самое
    // let mut sum: i32 = 0;
    // for i in array{
    //     sum+=i;
    // }


    sum as f64 / array.len() as f64

}


fn moda(array: &Vec<i32>)->i32{
    let mut map = HashMap::new();
    for i in array{

        let count = map.entry(i).or_insert(0);
            *count += 1;
    }
    let mut score = 0;
    let mut ind = 0;
    for (key, value) in &map {
        if score < *value {
            score = *value;
            ind = **key;
        }
    }
    ind
}

fn median(array: &Vec<i32>)->f64{
        if (array.len() % 2)==0 {
            let ind_left = array.len()/2-1; 
            let ind_right = array.len()/2 ;
            (array[ind_left]+array[ind_right]) as f64 / 2.0

        } else {
                array[(array.len()/2)] as f64
        }
}

fn main() {

  
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 равна {}", s2);
    println!("s1 равна {}", s1);
    //let v: Vec<i32> = Vec::new();

    let v = [1,1,2,2,2,3,3,4,5,6,6,6,6,6,70,70,80].to_vec();
    println!("list {:?}", v);
    let sred_v = mean(&v);
    println!("Mean array = {:?}", sred_v);
    let moda_v = moda(&v);
    println!("Moda = {:?}", moda_v);
    let median_v = median(&v);
    println!("Mediana = {:?}", median_v);
    
}
