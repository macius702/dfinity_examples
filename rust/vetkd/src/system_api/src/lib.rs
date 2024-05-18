use ic_cdk::{println, update};




#[update]
async fn vetkd_encrypted_key() -> () {
    println!("Derivation Path");
    ic_cdk::println!("Starting with_rng function");
        let (raw_rand,): (Vec<u8>,) = ic_cdk::api::management_canister::main::raw_rand()
            .await
            .unwrap_or_else(|_e| ic_cdk::trap("call to raw_rand failed"));
        let raw_rand_32_bytes: [u8; 32] = raw_rand
            .try_into()
            .unwrap_or_else(|_e| panic!("raw_rand not 32 bytes"));
        ic_cdk::println!("raw_rand: {:?}", raw_rand_32_bytes);

}

