/* fn main() {
    let mut s= String::from("Hello world");
    let ref_to_s= &s;
    let result= find_first_word(&s);
    
    s=String::from("Hello");
    // println!("ref_to_s= {}",ref_to_s);
    println!("s= {} | ressult={}",s,result);
    s.clear();
    println!("s= {} | ressult={}",s,result);

}

fn find_first_word(input: &String) -> usize{
      let s=input.as_bytes();

      for (i, &item) in s.iter().enumerate(){
        if item == b' '{
            return i;
        }
      }
      s.len()
} */


/* fn main(){
  let mut s=String::from("Hello world");
  println!("s= {}",&s[0..7]);
} */

/* fn main(){
  let mut s=String::from("Hello world");
   
  let find_first_word= find_first_word(&s);
  println!("find_first_word= {}",find_first_word);
  s.clear();
}

fn find_first_word(input: &String) -> &str{

  for (i, &item) in input.as_bytes().iter().enumerate(){
     if item == b' '{
       return &input[..i];
     }
  }

  &input[..]
} */

fn main(){
  let arr= [1,2,3,4,5];
  let slice= &arr[1..3];
  println!("{:?}",assert_eq!(slice, &[2,3]));
}