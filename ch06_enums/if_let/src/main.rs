// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {max}"),
//         _ => (),
//     }
// }

// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {max}");
//     }
// }

#[derive(Debug)]
#[allow(dead_code)]
enum USState {
    Alabama,
    Alaska,
    Arkansas,
    Missouri,
    Michigan,
}

impl USState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            USState::Alabama => year >= 1819,
            USState::Alaska => year >= 1959,
            USState::Arkansas => year >= 1836,
            USState::Missouri => year >= 1821,
            USState::Michigan => year >= 1837,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main() {
    let coin = Coin::Quarter(USState::Alaska);
    if let Some(output) = describe_state_quarter(&coin) {
        println!("{output}");
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
