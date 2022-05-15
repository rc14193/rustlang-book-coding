mod front_of_house;

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order (){

    }
}

// abs
use crate::front_of_house::hosting;
// rel
//use self::front_of_house::hosting

pub fn eat_at_restaurant() {

    //absolute
    crate::front_of_house::hosting::add_to_waitlist();

    //relative
    front_of_house::hosting::add_to_waitlist();

    // use
    hosting::add_to_waitlist();

}