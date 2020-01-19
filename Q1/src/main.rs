mod student {
    pub mod data {
        pub fn my_name() {
            println!("\nWe are in name function");
            println!("The student name is: Noman\n");
        }
    }
}

fn main() {
    
    crate::student::data::my_name(); // absolute path

    student::data::my_name(); //relative path
}
