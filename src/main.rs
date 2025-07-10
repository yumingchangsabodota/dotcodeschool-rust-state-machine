mod balances;
mod proof_of_existence;
mod system;
mod support;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;

}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	// TODO: Not implemented yet.
    Balances(balances::Call<Runtime>),
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	fn new() -> Self {
        Self { system:  system::Pallet::new(), balances: balances::Pallet::new() }
	}
	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err("Incoming block number not match.");
        }
        
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            // do stuff with `caller` and `call`
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call)
                                            .map_err(|error|{
                                                    println!("Block Number:{0} Extrinsic Number:{1}",
                                                    block.header.block_number,i);
                                                    println!("Error:{:?}", error);
                                                });

        }
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
        match runtime_call{
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            },
        }
        Ok(())
	}
}


fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
    let charlie = "charlie".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);

    /*
		TODO: Replace the logic above with a new `Block`.
			- Set the block number to 1 in the `Header`.
			- Move your existing transactions into extrinsic format, using the
			  `Extrinsic` and `RuntimeCall`.
	*/
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer{ to: bob, amount: 66 }),
            },            
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(balances::Call::Transfer{ to: charlie, amount: 20 }),
            },
        ],
    };

	/*
		TODO:
		Use your `runtime` to call the `execute_block` function with your new block.
		If the `execute_block` function returns an error, you should panic!
		We `expect` that all the blocks being executed must be valid.
	*/
    runtime.execute_block(block_1).expect("invalid block");

	// Simply print the debug format of our runtime state.                
    println!("{:#?}", runtime);
}
