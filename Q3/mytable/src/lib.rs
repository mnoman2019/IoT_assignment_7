pub mod welcome {
  pub mod pakistan {

    pub fn table2(data: u32) {
      println!("We are in library package and table function");
      for count in 1..=4 {
        println!("{}*{}={}", data, count, data * count);
      }
    }
  }
}
