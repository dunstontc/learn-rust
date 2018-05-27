// /// Print out the contents of a list
// pub fn print(list: Vec<i8>) {
//     for i in &list {
//         print!("{} ", i);
//     }
//     println!("")
// }

// fn average(numbers: &[i32]) -> f32 {
//     numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
// }

/// Given a list of numbers, returns the average value of the list.
pub fn mean(list: &Vec<i8>) {

}

/// Given a list of numbers, returns the value in the middle position of the sorted list.
pub fn median(list: &Vec<i8>) -> i8 {
    //   list.sort();
      let mid = list.len() / 2;

      return list[mid];
}

/// Given a list of numbers, returns the value that occurs most often in the list.
/// (*use a hashmap*)
pub fn mode(list: &Vec<i8>) {

}
