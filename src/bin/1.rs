fn main(){
    let a=Some(1);
    let b=a.and_then(|x| Some(x+1))
    .and_then(|x| Some(x+1))
    .and_then(|x| Some(x+1))
    .and_then(|x| Some(x+1))
    .and_then(|x| Some(x+1));
    println!("{:?}",b);
}