/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    non_snake_case
)]

/*- Imports -*/
use bevy::prelude::*;

/*- Constants -*/
/*- Structs, enums & unions -*/
#[derive( Debug )]
struct Human {
    name: &'static str,
    address: &'static str,
    age: u8,
    abilities: Vec<&'static str>
}

/*- Initialize -*/
fn main() {
    let human = Human::new("Aaron", "rumpvägen 4", 12, vec!["Kan kasta xbox kontroller i hastigheter över ljusets hastighet"]);
    println!("{} after birthday: {:?}", &human.name.clone(), &human.birthday());
    // Aaron {
    //     name: "Aaron",
    //     address: "Pooproad 23",
    //     age: 5,
    //     abilities: vec!["Shit powerfully", "Piss steam can break bones"]
    // };

    // App::new().run();
}

/*- implement methods for strutcs -*/
impl Human {
    fn new(name:&'static str, address: &'static str, age:u8, abilities: Vec<&'static str>) -> Self {
        Human { name, address, age, abilities }
    }

    fn new_with_age(age:u8) -> Self {
        Human { name: "", address: "", age, abilities: vec![] }
    }

    pub fn birthday(self) -> u8 {
        self.age+1
    }
}


