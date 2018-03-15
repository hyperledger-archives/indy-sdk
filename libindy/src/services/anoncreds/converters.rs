// This file contains conversion functions to convert old style objects to new style objects and vice versa
// This is temporary code.
extern crate indy_crypto;

use self::indy_crypto::cl::{CredentialSchema, CredentialPublicKey, CredentialPrivateKey,
                            CredentialPrimaryPublicKey, CredentialPrimaryPrivateKey,
                            BlindedCredentialSecrets, CredentialValue, CredentialValues,
                            PrimaryCredentialSignature, PrimaryBlindedCredentialSecretsFactors};
use self::indy_crypto::bn::BigNumber as NewBigNumber;
use self::indy_crypto::pair::{GroupOrderElement, PointG1, PointG2, Pair};
use self::indy_crypto::errors::IndyCryptoError;

use utils::crypto::bn::BigNumber as OldBigNumber;

use services::anoncreds::types::*;
use services::anoncreds::constants::{LINK_SECRET_NAME, POLICY_ADDRESS_NAME};
use errors::common::CommonError;

use std::collections::{BTreeMap, BTreeSet, HashSet, HashMap};


pub fn new_bn_to_old_bn(n: &NewBigNumber) -> Result<OldBigNumber, CommonError> {
    OldBigNumber::from_dec(&(n.to_dec()?))
}

pub fn old_bn_to_new_bn(n: &OldBigNumber) -> Result<NewBigNumber, CommonError> {
    match NewBigNumber::from_dec(&(n.to_dec()?)) {
        Ok(n) => Ok(n),
        Err(err) => Err(CommonError::InvalidStructure("Cannot convert to NewBigNumber".to_string())),
    }
}

pub fn new_bn_map_to_old_bn_map(m: &BTreeMap<String, NewBigNumber>) -> Result<HashMap<String, OldBigNumber>, CommonError> {
    let mut new_map: HashMap<String, OldBigNumber> = HashMap::new();
    for (key, ref val) in m.iter() {
        new_map.insert(key.to_string(), new_bn_to_old_bn(&val)?);
    }
    Ok(new_map)
}

pub fn old_bn_map_to_new_bn_map(m: &HashMap<String, OldBigNumber>) -> Result<BTreeMap<String, NewBigNumber>, CommonError> {
    let mut new_map: BTreeMap<String, NewBigNumber> = BTreeMap::new();
    for (key, ref val) in m.iter() {
        new_map.insert(key.to_string(), old_bn_to_new_bn(&val)?);
    }
    Ok(new_map)
}

pub fn Schema_to_CredentialSchema(schema: &Schema) -> CredentialSchema {
    // converts `Schema` to `CredentialSchema`
    let attrs: BTreeSet<String> = schema.data.attr_names.iter().cloned().collect();
    CredentialSchema {
        attrs
    }
}

pub fn CredentialPublicKey_to_PublicKey(pk: &mut CredentialPublicKey) -> Result<(PublicKey, Option<RevocationPublicKey>), CommonError> {
    let mut p_key = pk.get_primary_key()?;
    let prim = PublicKey {
        n: new_bn_to_old_bn(&p_key.n)?,
        s: new_bn_to_old_bn(&p_key.s)?,
        rms: new_bn_to_old_bn(&p_key.r.remove(LINK_SECRET_NAME).unwrap())?,
        rpa: new_bn_to_old_bn(&p_key.r.remove(POLICY_ADDRESS_NAME).unwrap())?,
        r: new_bn_map_to_old_bn_map(&p_key.r)?,
        rctxt: new_bn_to_old_bn(&p_key.rctxt)?,
        z: new_bn_to_old_bn(&p_key.z)?
    };
    let revoc = match pk.get_revocation_key()? {
        Some(r_key) => {
            Some(RevocationPublicKey {
                g: r_key.g,
                g_dash: r_key.g_dash,
                h: r_key.h,
                h0: r_key.h0,
                h1: r_key.h1,
                h2: r_key.h2,
                htilde: r_key.htilde,
                h_cap: r_key.h_cap,
                u: r_key.u,
                pk: r_key.pk,
                y: r_key.y,
            })
        }
        _ => None
    };
    Ok((prim, revoc))
}


pub fn CredentialPrivateKey_to_SecretKey(sk: &CredentialPrivateKey) -> Result<(SecretKey, Option<RevocationSecretKey>), CommonError> {
    let p_key = sk.get_primary_key()?;
    let prim = SecretKey {
        p: new_bn_to_old_bn(&p_key.p)?,
        q: new_bn_to_old_bn(&p_key.q)?,
    };
    let revoc = match sk.get_revocation_key()? {
        Some(r_key) => {
            Some(RevocationSecretKey {
                x: r_key.x,
                sk: r_key.sk,
            })
        }
        _ => None
    };
    Ok((prim, revoc))
}

pub fn PublicKey_to_CredentialPublicKey(pk: &PublicKey) -> Result<CredentialPublicKey, CommonError> {
    let mut r = old_bn_map_to_new_bn_map(&pk.r)?;
    r.insert(LINK_SECRET_NAME.to_string(), old_bn_to_new_bn(&pk.rms)?);
    r.insert(POLICY_ADDRESS_NAME.to_string(), old_bn_to_new_bn(&pk.rpa)?);
    let prim = CredentialPrimaryPublicKey {
        n: old_bn_to_new_bn(&pk.n)?,
        s: old_bn_to_new_bn(&pk.s)?,
        r: r,
        rctxt: old_bn_to_new_bn(&pk.rctxt)?,
        z: old_bn_to_new_bn(&pk.z)?
    };
    Ok(CredentialPublicKey::build_from_parts(&prim, None)?)
}

pub fn SecretKey_to_CredentialPrivateKey(sk: &SecretKey) -> Result<CredentialPrivateKey, CommonError> {
    let prim = CredentialPrimaryPrivateKey {
        p: old_bn_to_new_bn(&sk.p)?,
        q: old_bn_to_new_bn(&sk.q)?,
    };
    Ok(CredentialPrivateKey {p_key: prim, r_key: None})
}

pub fn PrimaryCredentialSignature_to_PrimaryClaim(cs: &PrimaryCredentialSignature) -> Result<PrimaryClaim, CommonError> {
    Ok(PrimaryClaim {
        m2: new_bn_to_old_bn(&cs.m_2)?,
        a: new_bn_to_old_bn(&cs.a)?,
        e: new_bn_to_old_bn(&cs.e)?,
        v: new_bn_to_old_bn(&cs.v)?,
    })
}

pub fn PrimaryBlindedCredentialSecretsFactors_to_ClaimInitData(binding_factors: PrimaryBlindedCredentialSecretsFactors) -> Result<ClaimInitData, CommonError> {
    Ok(ClaimInitData {
        u: new_bn_to_old_bn(&binding_factors.u)?,
        v_prime: new_bn_to_old_bn(&binding_factors.v_prime)?
    })
}

pub fn gen_hidden_attributes() -> BTreeSet<String>{
    let mut attrs: BTreeSet<String> = BTreeSet::new();
    attrs.insert(LINK_SECRET_NAME.to_string());
    attrs.insert(POLICY_ADDRESS_NAME.to_string());
    return attrs
}

pub fn gen_BlindedCredentialSecrets(u: &OldBigNumber, ur: &Option<PointG1>) -> Result<BlindedCredentialSecrets, CommonError> {
    Ok(BlindedCredentialSecrets {
        u: old_bn_to_new_bn(u)?,
        ur: ur.clone(),
        hidden_attributes: gen_hidden_attributes(),
        committed_attributes: BTreeMap::new()
    })
}

pub fn gen_known_CredentialValues(attributes: &HashMap<String, Vec<String>>) -> Result<CredentialValues, CommonError> {
    let mut attrs: BTreeMap<String, CredentialValue> = BTreeMap::new();
    for (key, ref val) in attributes.iter() {
        attrs.insert(key.to_string(), CredentialValue::Known { value: NewBigNumber::from_dec(val.get(1).unwrap())? });
    }
    Ok(CredentialValues {
        attrs_values: attrs
    })
}

pub fn gen_hidden_CredentialValues(ms: &OldBigNumber,
                                   policy_address: Option<OldBigNumber>) -> Result<CredentialValues, CommonError> {
    let mut attrs: BTreeMap<String, CredentialValue> = BTreeMap::new();
    attrs.insert(LINK_SECRET_NAME.to_string(), CredentialValue::Hidden { value: old_bn_to_new_bn(ms)? });
    if policy_address.is_some() {
        attrs.insert(POLICY_ADDRESS_NAME.to_string(), CredentialValue::Hidden { value: old_bn_to_new_bn(&policy_address.unwrap())? });
    }
    Ok(CredentialValues {
        attrs_values: attrs
    })
}