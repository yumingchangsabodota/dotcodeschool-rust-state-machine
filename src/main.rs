mod balances;
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
    BalancesTransfer { to: types::AccountId, amount: types::Balance },
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
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(caller, to, amount)?;
            },
        }
        Ok(())
	}
}


fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(&"alice".to_string(), 100);

	// start emulating a block
	/* TODO: Increment the block number in system. */
	/* TODO: Assert the block number is what we expect. */
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	/* TODO: Increment the nonce of `alice`. */
    runtime.system.inc_nonce(&"alice".to_string());

	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/
    let _res = runtime
                            .balances
                            .transfer("alice".to_string(), "bob".to_string(), 30)
                            .map_err(|error|{
                                println!("{:?}",error);
                            });


	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
    runtime.system.inc_nonce(&"alice".to_string());
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _res = runtime
                            .balances
                            .transfer("alice".to_string(), "charlie".to_string(), 30)
                            .map_err(|error|{
                                println!("{:?}",error);
                            });
    println!("{:#?}", runtime);
}
