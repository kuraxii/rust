mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        fn some_function(){}
    }
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::some_function();
}




fn server_order(){}

mod back_of_house{
    fn fix_incorrect_order()
    {
        cook_order();
        super::server_order();
        
    }
    fn cook_order(){}
}