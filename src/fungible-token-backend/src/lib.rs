// use candid::{CandidType, Deserialize, Principal};
// use ic_cdk::api::caller;

// use ic_cdk::{init, query, update};

// use std::cell::RefCell;
// use std::collections::HashMap;
// #[derive(CandidType, Deserialize, Clone)]
// struct TokenState {
//     name: String,
//     symbol: String,
//     total_supply: u64,
//     owner: Principal,
//     balances: HashMap<Principal, u64>,
//     holders: Vec<Principal>,
// }

// // Provide a Manual Default because `Principal` does not implement Default
// impl Default for TokenState {
//     fn default() -> Self {
//         Self {
//             name: "".to_string(),
//             symbol: "".to_string(),
//             total_supply: 0,
//             owner: Principal::anonymous(), // Default owner
//             balances: HashMap::new(),
//             holders: Vec::new(),
//         }
//     }
// }

// thread_local! {
//     static STATE: RefCell<TokenState> = RefCell::new(TokenState::default());
// }

// #[init]
// fn init(name: String, symbol: String, initial_supply: u64) {
//     let owner = caller();
//     STATE.with(|s| {
//         let mut st = s.borrow_mut();
//         st.name = name;
//         st.symbol = symbol;
//         st.total_supply = initial_supply;
//         st.owner = owner.clone();
//         st.balances.insert(owner.clone(), initial_supply);
//         st.holders.push(owner);
//     });
// }

// #[query]
// fn name() -> String {
//     STATE.with(|s| s.borrow().name.clone())
// }

// #[query]
// fn symbol() -> String {
//     STATE.with(|s| s.borrow().symbol.clone())
// }

// #[query]
// fn total_supply() -> u64 {
//     STATE.with(|s| s.borrow().total_supply)
// }

// #[query]
// fn owner() -> Principal {
//     // return an owned Principal (clone from state)
//     STATE.with(|s| s.borrow().owner.clone())
// }

// #[query]
// fn balance_of(p: Principal) -> u64 {
//     STATE.with(|s| *s.borrow().balances.get(&p).unwrap_or(&0))
// }

// #[query]
// fn holders() -> Vec<(Principal, u64)> {
//     STATE.with(|s| {
//         let st = s.borrow();
//         st.holders
//             .iter()
//             .map(|p| (p.clone(), *st.balances.get(p).unwrap_or(&0)))
//             .collect()
//     })
// }

// #[update]
// fn transfer(to: Principal, amount: u64) -> Result<bool, String> {
//     let from = caller();
//     STATE.with(|s| {
//         let mut st = s.borrow_mut();
//         let sender_balance = *st.balances.get(&from).unwrap_or(&0);
//         if sender_balance < amount {
//             return Err("Not enough tokens".to_string());
//         }
//         st.balances.insert(from.clone(), sender_balance - amount);
//         let recipient_balance = *st.balances.get(&to).unwrap_or(&0);
//         st.balances.insert(to.clone(), recipient_balance + amount);
//         if !st.holders.contains(&to) {
//             st.holders.push(to);
//         }
//         Ok(true)
//     })
// }

// #[update]
// fn mint(to: Principal, amount: u64) -> Result<bool, String> {
//     let caller_principal = caller();
//     STATE.with(|s| {
//         let mut st = s.borrow_mut();
//         if caller_principal != st.owner {
//             return Err("Only owner can mint".to_string());
//         }
//         let recipient_balance = *st.balances.get(&to).unwrap_or(&0);
//         st.balances.insert(to.clone(), recipient_balance + amount);
//         st.total_supply = st.total_supply.checked_add(amount).ok_or("overflow")?;
//         if !st.holders.contains(&to) {
//             st.holders.push(to);
//         }
//         Ok(true)
//     })
// }


use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::caller;
use ic_cdk::{init, query, update};
use std::cell::RefCell;
use std::collections::HashMap;


#[derive(CandidType, Deserialize, Clone)]
struct TokenState {
    name: String,
    symbol: String,
    total_supply: u64,
    owner: Principal,
    balances: HashMap<Principal, u64>,
    holders: Vec<Principal>,
}

// Manual Default because Principal doesn't implement Default
impl Default for TokenState {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            symbol: "".to_string(),
            total_supply: 0,
            owner: Principal::anonymous(),
            balances: HashMap::new(),
            holders: Vec::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<TokenState> = RefCell::new(TokenState::default());
}

// Init arguments struct matching candid record argument
#[derive(CandidType, Deserialize)]
struct InitArgs {
    name: String,
    symbol: String,
    initial_supply: u64,
}

#[init]
fn init(args: InitArgs) {
    let owner = caller();
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        st.name = args.name;
        st.symbol = args.symbol;
        st.total_supply = args.initial_supply;
        st.owner = owner.clone();
        st.balances.insert(owner.clone(), args.initial_supply);
        st.holders.push(owner);
    });
}

#[query]
fn name() -> String {
    STATE.with(|s| s.borrow().name.clone())
}

#[query]
fn symbol() -> String {
    STATE.with(|s| s.borrow().symbol.clone())
}

#[query]
fn total_supply() -> u64 {
    STATE.with(|s| s.borrow().total_supply)
}

#[query]
fn owner() -> Principal {
    STATE.with(|s| s.borrow().owner.clone())
}

#[query]
fn balance_of(p: Principal) -> u64 {
    STATE.with(|s| *s.borrow().balances.get(&p).unwrap_or(&0))
}

#[query]
fn holders() -> Vec<(Principal, u64)> {
    STATE.with(|s| {
        let st = s.borrow();
        st.holders
            .iter()
            .map(|p| (p.clone(), *st.balances.get(p).unwrap_or(&0)))
            .collect()
    })
}

#[update]
fn transfer(to: Principal, amount: u64) -> Result<bool, String> {
    let from = caller();
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        let sender_balance = *st.balances.get(&from).unwrap_or(&0);
        if sender_balance < amount {
            return Err("Not enough tokens".to_string());
        }
        st.balances.insert(from.clone(), sender_balance - amount);
        let recipient_balance = *st.balances.get(&to).unwrap_or(&0);
        st.balances.insert(to.clone(), recipient_balance + amount);
        if !st.holders.contains(&to) {
            st.holders.push(to);
        }
        Ok(true)
    })
}

#[update]
fn mint(to: Principal, amount: u64) -> Result<bool, String> {
    let caller_principal = caller();
    STATE.with(|s| {
        let mut st = s.borrow_mut();
        if caller_principal != st.owner {
            return Err("Only owner can mint".to_string());
        }
        let recipient_balance = *st.balances.get(&to).unwrap_or(&0);
        st.balances.insert(to.clone(), recipient_balance + amount);
        st.total_supply = st.total_supply.checked_add(amount).ok_or("overflow")?;
        if !st.holders.contains(&to) {
            st.holders.push(to);
        }
        Ok(true)
    })
}
