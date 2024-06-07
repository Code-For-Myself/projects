use std::collections::HashMap;

fn main() {
    let mut v: Vec<f32> = vec![9.0,8.0,7.0,6.0,4.0,4.0,3.0,2.0,2.0,1.0, 1.0, 1.0];
    let s = v.len();
    let mut map = HashMap::new();
    
    loop {
        let mut counter = 0;
        for i in 0..s-1 {
            map.insert(i, v[i]<v[i+1]);
            if (v[i]<=v[i+1]) == true {
                counter += 1;
            }
        }
        for (i, wor) in &map{
            if *wor == false{
                v.insert(i+2,v[*i]);
                v.remove(*i);
                
            } 
        }
        if counter == s-1 {
            break;
        }
    };
    for i in &v {
        println!("{i}");
    }
    if s % 2 == 1{
        let median = v[(s-1)/2+1];
        println!("{median}");
    } else {
        let median = (v[(s-1)/2]+v[(s-1)/2+1])/2.0;
        println!("{median}");
    }
    

}    
        

