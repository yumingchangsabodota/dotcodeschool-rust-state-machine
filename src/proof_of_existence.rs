use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	/* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self {claims: BTreeMap::new()}
	}
	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		/* TODO: `get` the `claim` */
		self.claims.get(claim)
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {

	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> crate::support::DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }
        self.claims.insert(claim, caller);

		Ok(())
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> crate::support::DispatchResult {
        let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if *owner != caller {
            return Err("caller does not own this content");
        }
        self.claims.remove(&claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {
    use std::fmt::Error;

	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		/*
			TODO:
			Create an end to end test verifying the basic functionality of this pallet.
				- Check the initial state is as you expect.
				- Check that all functions work successfully.
				- Check that all error conditions error as expected.
		*/
        let mut claims = super::Pallet::<TestConfig>::new();
        assert_eq!(claims.get_claim(&"testcontent"), None);
        let _res = claims.create_claim("ziv", "this is ziv's first claim");
        assert_eq!(claims.get_claim(&"this is ziv's first claim"), Some(&"ziv"));
        let _res = claims.revoke_claim("ziv", "this is not ziv's claim");
        assert_eq!(_res, Err("claim does not exist"));
        let _res = claims.create_claim("someone", "this is someone's first claim");
        let _res = claims.revoke_claim("ziv", "this is someone's first claim");
        assert_eq!(_res, Err("caller does not own this content"));
        let _res = claims.revoke_claim("ziv", "this is ziv's first claim");
        assert_eq!(_res, Ok(()));

	}
}
