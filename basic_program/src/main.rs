fn main() {
   
   let string1: &str = "Chizaa";
   let string2: &str ="Kim";

    concatenate_strings(string1, string2);
}

fn concatenate_strings(string1: &str, string2: &str) -> String{   

     let mut result = String::from(string1);

     result.push_str(string2);

     println!("{}", result);
     
     result


}