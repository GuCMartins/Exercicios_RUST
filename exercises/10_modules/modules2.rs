// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
    // TODO: Fix these use statements
    
    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }
    
    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
    pub use self::fruits::PEAR as fruit1;       //aplica outro nome para o item ja dentro do modulo fruits
    pub use self::veggies::CUCUMBER as veggie1; //aplica outro nome para o item ja dentro do modulo veggies

    pub use self::fruits::APPLE as fruit2;      //aplica outro nome para o item ja dentro do modulo fruits
    pub use self::veggies::CARROT as veggie2;   //aplica outro nome para o item ja dentro do modulo veggies
}

fn main() {
    println!(
        "favorite snacks: {} and {}, but I also like {} and {}",
        delicious_snacks::fruit1,//precisa ter o nome do modulo antes do item pois
        delicious_snacks::veggie1,//ele esta sendo especificado dentro do modulo
        delicious_snacks::fruit2,//e nao no escopo global
        delicious_snacks::veggie2,
    );
}
