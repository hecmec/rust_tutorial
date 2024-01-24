mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub toppings: String,
    }
    /**
     * Associated functions are functions that are associated with a struct
     */
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozarella"),
                toppings: String::from(topping),
            }
        }

        pub fn new(dough: String, cheese: String, toppings: String) -> Pizza {
            Pizza {
                dough,
                cheese,
                toppings,
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Please seat at table");
        }
        pub fn take_order() {
            println!("Please take your order");
            seat_at_table();
            let cust_pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza);
        }
        fn serve_customer(pizza: super::Pizza) {
            println!(
                "Here is your pizza: {} {} {}",
                pizza.dough, pizza.cheese, pizza.toppings
            );
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
