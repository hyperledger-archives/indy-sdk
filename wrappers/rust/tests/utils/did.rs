use super::indy;

use indy::ErrorCode;

type DidAndVerKey = (String, String);

#[derive(Clone, Copy)]
pub enum NymRole
{
    Trustee,
    User,
}

impl NymRole
{
    pub fn prepare(&self) -> Option<&str>
    {
        match self {
            NymRole::Trustee => Some("TRUSTEE"),
            NymRole::User => None,
        }
    }
}

/**
Generate a did and send a nym request for it.
*/
pub fn create_nym(
    wallet_handle: i32,
    pool_handle: i32,
    did_trustee: &str,
    role: NymRole
) -> Result<DidAndVerKey, ErrorCode> {
    let (did, verkey) = indy::did::Did::new(wallet_handle, "{}").unwrap();

    let req_nym = indy::ledger::Ledger::build_nym_request(
        did_trustee,
        &did,
        Some(&verkey),
        None,
        role.prepare()
    )?;

    indy::ledger::Ledger::sign_and_submit_request(pool_handle, wallet_handle, &did_trustee, &req_nym)?;

    Ok((did, verkey))
}

/**
Creates multiple dids and corresponding nym requests.
*/
pub fn create_multiple_nym(
    wallet_handle: i32,
    pool_handle: i32,
    did_trustee: &str,
    n: u8,
    role: NymRole
) -> Result<Vec<DidAndVerKey>, ErrorCode> {
    let mut v: Vec<(String, String)> = Vec::new();
    for _ in 0..n {
        let new_did = create_nym(wallet_handle, pool_handle, did_trustee, role)?;
        v.push(new_did);
    }

    Ok(v)
}

/**
Create and store the initial dids of trustees.

Includes the initial trustee.
*/
pub fn initial_trustees(num_trustees: u8, wallet_handle: i32, pool_handle: i32) -> Result<Vec<DidAndVerKey>, ErrorCode> {
    let first = initial_trustee(wallet_handle);

    let mut trustees = create_multiple_nym(
        wallet_handle,
        pool_handle,
        &first.0,
        num_trustees - 1,
        NymRole::Trustee
    )?;
    trustees.insert(0, first);

    Ok(trustees)
}

/**
Store the did of the intial trustee
*/
pub fn initial_trustee(wallet_handle: i32) -> DidAndVerKey {
    let first_json_seed = json!({
        "seed":"000000000000000000000000Trustee1"
    }).to_string();

    indy::did::Did::new(wallet_handle, &first_json_seed).unwrap()
}

/**
Discard the verkey and return the did from a `Vec<DidAndVerKey`.
*/
pub fn did_str_from_trustees<'a>(trustees: &'a Vec<DidAndVerKey>) -> Vec<&'a str> {
    trustees
        .iter()
        .map(|(ref did, _)| did.as_str())
        .collect()
}
