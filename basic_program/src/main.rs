fn main() {
   
   let s1: &str = "Chizaa";
   let s2: &str ="Kim";
   
    concatenate_strings(s1, s2);
}

fn concatenate_strings(s1: &str, s2: &str) -> String{   
     println!("{} {}", s1, s2);

    format!("{} {}", s1, s2)

}