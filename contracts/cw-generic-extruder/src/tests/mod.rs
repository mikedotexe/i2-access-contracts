use cosmwasm_std::{coins, Addr, VerificationError, Api, StdResult, CanonicalAddr, StdError, RecoverPubkeyError, Storage};
use cosmwasm_std::testing::{digit_sum, mock_env, MockApi, MockStorage, riffle_shuffle};
use cw_multi_test::{App, AppBuilder, BankKeeper, Router};

mod contracts;
mod core_tests;
const SHUFFLES_ENCODE: usize = 10;
const SHUFFLES_DECODE: usize = 2;

/// We set this to "TOKEN" to match the denom here:
/// https://github.com/CosmWasm/cosmwasm/blob/32f308a1a56ae5b8278947891306f7a374c3df94/packages/vm/src/environment.rs#L383
pub const DENOM: &str = "TOKEN";

pub fn alice() -> Addr {
    Addr::unchecked("juno18smcd7hmzj4pt04vmt693u6lcvtxr44e20kz3w")
}
pub fn bob() -> Addr {
    Addr::unchecked("juno1hwpalfeqt7h892qrfepg5ypklf7yvas5tz6nhj")
}
pub fn charlie() -> Addr {
    Addr::unchecked("juno15md2qvgma8lnvqv67w0umu2paqkqkheg0c6h0r")
}

fn no_init<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT>(
    _: &mut Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT>,
    _: &dyn Api,
    _: &mut dyn Storage,
) {
}

pub fn default_app() -> App<BankKeeper, MyMockApi> {
    let my_api = MyMockApi {
        canonical_length: 43,
    };
    let app = AppBuilder::new_custom()
      .with_api(my_api)
      .build(|router, _, storage| {
          let accounts: Vec<(u128, Addr)> =
            vec![(6_000_000, alice()), (660_000, bob()), (66_600, charlie())];
          for (amt, address) in accounts {
              router
                .bank
                .init_balance(storage, &address, coins(amt, DENOM))
                .unwrap();
          }
      });

    app
}

#[derive(Copy, Clone)]
pub struct MyMockApi {
    /// Length of canonical addresses created with this API. Contracts should not make any assumptions
    /// what this value is.
    canonical_length: usize,
}

impl Default for MyMockApi {
    fn default() -> Self {
        MyMockApi {
            canonical_length: 43, // fuckin' a
        }
    }
}

impl Api for MyMockApi {
    fn addr_validate(&self, input: &str) -> StdResult<Addr> {
        println!("aloha mebbe0");
        // because cw-multi-test returns stuff like "contract0"
        // we skip all validation

        // let canonical = self.addr_canonicalize(input)?;
        // let normalized = self.addr_humanize(&canonical)?;
        // if input != normalized {
        //     return Err(StdError::generic_err(
        //         "Invalid input: address not normalized",
        //     ));
        // }

        Ok(Addr::unchecked(input))
    }

    fn addr_canonicalize(&self, input: &str) -> StdResult<CanonicalAddr> {
        // Dummy input validation. This is more sophisticated for formats like bech32, where format and checksum are validated.
        let min_length = 3;
        let max_length = self.canonical_length;
        if input.len() < min_length {
            return Err(StdError::generic_err(
                format!("Invalid input: human address too short for this mock implementation (must be >= {min_length})."),
            ));
        }
        println!("aloha input.len() {:?}", input.len());
        println!("aloha max_length {:?}", max_length);
        if input.len() > max_length {
            return Err(StdError::generic_err(
                format!("Invalid input: human address too long for this mock implementation (must be <= {max_length})."),
            ));
        }

        // mimicks formats like hex or bech32 where different casings are valid for one address
        let normalized = input.to_lowercase();

        let mut out = Vec::from(normalized);

        // pad to canonical length with NULL bytes
        out.resize(self.canonical_length, 0x00);
        // content-dependent rotate followed by shuffle to destroy
        // the most obvious structure (https://github.com/CosmWasm/cosmwasm/issues/552)
        let rotate_by = digit_sum(&out) % self.canonical_length;
        out.rotate_left(rotate_by);
        for _ in 0..SHUFFLES_ENCODE {
            out = riffle_shuffle(&out);
        }
        Ok(out.into())
    }

    fn addr_humanize(&self, canonical: &CanonicalAddr) -> StdResult<Addr> {
        println!("aloha top of addr_humanize");
        // fuck you
        if canonical.len() != self.canonical_length {
            return Err(StdError::generic_err(
                "Invalid input: canonical address length not correct",
            ));
        }

        let mut tmp: Vec<u8> = canonical.clone().into();
        // Shuffle two more times which restored the original value (24 elements are back to original after 20 rounds)
        for _ in 0..SHUFFLES_DECODE {
            tmp = riffle_shuffle(&tmp);
        }
        // Rotate back
        let rotate_by = digit_sum(&tmp) % self.canonical_length;
        tmp.rotate_right(rotate_by);
        // Remove NULL bytes (i.e. the padding)
        let trimmed = tmp.into_iter().filter(|&x| x != 0x00).collect();
        // decode UTF-8 bytes into string
        let human = String::from_utf8(trimmed)?;
        Ok(Addr::unchecked(human))
    }

    fn secp256k1_verify(
        &self,
        message_hash: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        Ok(cosmwasm_crypto::secp256k1_verify(
            message_hash,
            signature,
            public_key,
        )?)
    }

    fn secp256k1_recover_pubkey(
        &self,
        message_hash: &[u8],
        signature: &[u8],
        recovery_param: u8,
    ) -> Result<Vec<u8>, RecoverPubkeyError> {
        let pubkey =
          cosmwasm_crypto::secp256k1_recover_pubkey(message_hash, signature, recovery_param)?;
        Ok(pubkey.to_vec())
    }

    fn ed25519_verify(
        &self,
        message: &[u8],
        signature: &[u8],
        public_key: &[u8],
    ) -> Result<bool, VerificationError> {
        Ok(cosmwasm_crypto::ed25519_verify(
            message, signature, public_key,
        )?)
    }

    fn ed25519_batch_verify(
        &self,
        messages: &[&[u8]],
        signatures: &[&[u8]],
        public_keys: &[&[u8]],
    ) -> Result<bool, VerificationError> {
        Ok(cosmwasm_crypto::ed25519_batch_verify(
            messages,
            signatures,
            public_keys,
        )?)
    }

    fn debug(&self, message: &str) {
        println!("{}", message);
    }
}