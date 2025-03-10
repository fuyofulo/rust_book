// src/lib.rs

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Customer added to the waitlist!");
        }
        
        pub fn seat_at_table() {
            println!("Customer seated at the table!");
        }
    }
    
    pub mod serving {
        pub fn take_order() {
            println!("Order has been taken!");
        }
        
        pub fn serve_order() {
            println!("Order has been served!");
        }
        
        pub fn take_payment() {
            println!("Payment has been taken!");
        }
    }
}

