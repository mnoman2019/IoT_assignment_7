pub mod welcome {
  pub mod pakistan {
    pub fn table(data: u32) {
      println!("We are in lib.rs -> table function");
      for count in 1..=5 {
        println!("{}*{}={}", data, count, data * count);
      }
    }
  }
}
