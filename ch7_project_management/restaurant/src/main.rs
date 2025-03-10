// Bring in the hosting and serving modules from our library.
use restaurant::front_of_house::hosting;
use restaurant::front_of_house::serving;

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}

