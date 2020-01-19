pub mod welcome {
  pub fn table(data: u32) {
    println!("We are in lib.rs -> table function");
    for count in 1..=3 {
      println!("{}*{}={}", data, count, data * count);
    }
  }
}
