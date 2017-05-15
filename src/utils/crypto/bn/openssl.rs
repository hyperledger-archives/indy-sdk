use errors::crypto::CryptoError;

extern crate openssl;
extern crate int_traits;
extern crate serde;

use self::int_traits::IntTraits;

use self::openssl::bn::{BigNum, BigNumRef, BigNumContext};
use self::openssl::hash::{hash2, MessageDigest, Hasher};
use self::serde::ser::{Serialize, Serializer, Error as SError};
use self::serde::de::{Deserialize, Deserializer, Visitor, Error as DError};
use std::fmt;
use std::cmp::Ord;
use std::cmp::Ordering;
use services::anoncreds::helpers::BytesView;

#[cfg(test)]
use services::anoncreds::constants::{
    LARGE_ALPHATILDE,
    LARGE_NONCE,
    LARGE_ETILDE,
    LARGE_UTILDE,
    LARGE_RTILDE,
    LARGE_MASTER_SECRET,
    LARGE_PRIME,
    LARGE_VPRIME,
    LARGE_VPRIME_PRIME,
    LARGE_VTILDE
};

pub struct BigNumberContext {
    openssl_bn_context: BigNumContext
}

#[derive(Debug)]
pub struct BigNumber {
    openssl_bn: BigNum
}

impl BigNumber {
    pub fn new_context() -> Result<BigNumberContext, CryptoError> {
        let ctx = BigNumContext::new()?;
        Ok(BigNumberContext {
            openssl_bn_context: ctx
        })
    }

    pub fn new() -> Result<BigNumber, CryptoError> {
        let bn = BigNum::new()?;
        Ok(BigNumber {
            openssl_bn: bn
        })
    }

    pub fn generate_prime(size: usize) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        BigNumRef::generate_prime(&mut bn.openssl_bn, size as i32, false, None, None)?;
        Ok(bn)
    }

    #[cfg(not(test))]
    pub fn generate_safe_prime(size: usize) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        BigNumRef::generate_prime(&mut bn.openssl_bn, (size + 1) as i32, true, None, None)?;
        Ok(bn)
    }

    #[cfg(test)]
    pub fn generate_safe_prime(size: usize) -> Result<BigNumber, CryptoError> {
        match size {
            LARGE_PRIME => Ok(BigNumber::from_dec("298425477551432359319017298068281828134535746771300905126443720735756534287270383542467183175737460443806952398210045827718115111810885752229119677470711305345901926067944629292942471551423868488963517954094239606951758940767987427212463600313901180668176172283994206392965011112962119159458674722785709556623")?),
            _ => {
                debug!("Uncovered case: {}", size);
                Ok(BigNumber::new()?)
            }
        }
    }

    pub fn generate_prime_in_range(start: &BigNumber, end: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut prime;
        let mut iteration = 0;
        let mut bn_ctx = BigNumber::new_context()?;
        let sub = end.sub(start)?;

        loop {
            prime = sub.rand_range()?;
            prime = prime.add(start)?;

            if prime.is_prime(Some(&mut bn_ctx))? {
                debug!("Found prime in {} iteration", iteration);
                break;
            }
            iteration += 1;
        }

        Ok(prime)
    }

    pub fn is_prime(&self, ctx: Option<&mut BigNumberContext>) -> Result<bool, CryptoError> {
        let prime_len = self.to_dec()?.len();
        let checks = prime_len.log2() as i32;
        match ctx {
            Some(context) => Ok(self.openssl_bn.is_prime(checks, &mut context.openssl_bn_context)?),
            None => {
                let mut ctx = BigNumber::new_context()?;
                Ok(self.openssl_bn.is_prime(checks, &mut ctx.openssl_bn_context)?)
            }
        }
    }

    #[cfg(not(test))]
    pub fn rand(size: usize) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        BigNumRef::rand(&mut bn.openssl_bn, size as i32, openssl::bn::MSB_MAYBE_ZERO, false)?;
        Ok(bn)
    }

    #[cfg(test)]
    pub fn rand(size: usize) -> Result<BigNumber, CryptoError> {
        match size {
            LARGE_NONCE => Ok(BigNumber::from_dec("526193306511429638192053")?),
            LARGE_MASTER_SECRET => Ok(BigNumber::from_dec("21578029250517794450984707538122537192839006240802068037273983354680998203845")?),
            LARGE_ETILDE => Ok(BigNumber::from_dec("162083298053730499878539835193560156486733663622707027216327685550780519347628838870322946818623352681120371349972731968874009673965057322")?),
            LARGE_UTILDE => Ok(BigNumber::from_dec("6461691768834933403326572830814516653957231030793837560544354737855803497655300429843454445497126567767486684087006218691084619904526729989680526652503377438786587511370042964338")?),
            LARGE_RTILDE => Ok(BigNumber::from_dec("7575191721496255329790454166600075461811327744716122725414003704363002865687003988444075479817517968742651133011723131465916075452356777073568785406106174349810313776328792235352103470770562831584011847")?),
            LARGE_PRIME => Ok(BigNumber::from_dec("67940925789970108743024738273926421512152745397724199848594503731042154269417576665420030681245389493783225644817826683796657351721363490290016166310023506339911751676800452438014771736117676826911321621579680668201191205819012441197794443970687648330757835198888257781967404396196813475280544039772512800509")?),
            LARGE_VPRIME => Ok(BigNumber::from_dec("1921424195886158938744777125021406748763985122590553448255822306242766229793715475428833504725487921105078008192433858897449555181018215580757557939320974389877538474522876366787859030586130885280724299566241892352485632499791646228580480458657305087762181033556428779333220803819945703716249441372790689501824842594015722727389764537806761583087605402039968357991056253519683582539703803574767702877615632257021995763302779502949501243649740921598491994352181379637769188829653918416991301420900374928589100515793950374255826572066003334385555085983157359122061582085202490537551988700484875690854200826784921400257387622318582276996322436")?),
            LARGE_VPRIME_PRIME => Ok(BigNumber::from_dec("6620937836014079781509458870800001917950459774302786434315639456568768602266735503527631640833663968617512880802104566048179854406925811731340920442625764155409951969854303612644121780700879432308016935250101960876405664503219252820761501606507817390189252221968804450207070282033815280889897882643560437257171838117793768660731379360330750300543760457608638753190279419951706206819943151918535286779337023708838891906829360439545064730288538139152367417882097349210427894031568623898916625312124319876670702064561291393993815290033742478045530118808274555627855247830659187691067893683525651333064738899779446324124393932782261375663033826174482213348732912255948009062641783238846143256448824091556005023241191311617076266099622843011796402959351074671886795391490945230966123230485475995208322766090290573654498779155")?),
            LARGE_VTILDE => Ok(BigNumber::from_dec("241132863422049783305938184561371219250127488499746090592218003869595412171810997360214885239402274273939963489505434726467041932541499422544431299362364797699330176612923593931231233163363211565697860685967381420219969754969010598350387336530924879073366177641099382257720898488467175132844984811431059686249020737675861448309521855120928434488546976081485578773933300425198911646071284164884533755653094354378714645351464093907890440922615599556866061098147921890790915215227463991346847803620736586839786386846961213073783437136210912924729098636427160258710930323242639624389905049896225019051952864864612421360643655700799102439682797806477476049234033513929028472955119936073490401848509891547105031112859155855833089675654686301183778056755431562224990888545742379494795601542482680006851305864539769704029428620446639445284011289708313620219638324467338840766574612783533920114892847440641473989502440960354573501")?),
            LARGE_ALPHATILDE => Ok(BigNumber::from_dec("15019832071918025992746443764672619814038193111378331515587108416842661492145380306078894142589602719572721868876278167686578705125701790763532708415180504799241968357487349133908918935916667492626745934151420791943681376124817051308074507483664691464171654649868050938558535412658082031636255658721308264295197092495486870266555635348911182100181878388728256154149188718706253259396012667950509304959158288841789791483411208523521415447630365867367726300467842829858413745535144815825801952910447948288047749122728907853947789264574578039991615261320141035427325207080621563365816477359968627596441227854436137047681372373555472236147836722255880181214889123172703767379416198854131024048095499109158532300492176958443747616386425935907770015072924926418668194296922541290395990933578000312885508514814484100785527174742772860178035596639")?),
            _ => {
                debug!("Uncovered case: {}", size);
                Ok(BigNumber::new()?)
            }
        }
    }

    pub fn rand_range(&self) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        BigNumRef::rand_range(&self.openssl_bn, &mut bn.openssl_bn)?;
        Ok(bn)
    }

    pub fn num_bits(&self) -> Result<i32, CryptoError> {
        Ok(self.openssl_bn.num_bits())
    }

    pub fn is_bit_set(&self, n: i32) -> Result<bool, CryptoError> {
        Ok(self.openssl_bn.is_bit_set(n))
    }

    pub fn set_bit(&mut self, n: i32) -> Result<&mut BigNumber, CryptoError> {
        BigNumRef::set_bit(&mut self.openssl_bn, n)?;
        Ok(self)
    }

    pub fn from_u32(n: usize) -> Result<BigNumber, CryptoError> {
        let bn = BigNum::from_u32(n as u32)?;
        Ok(BigNumber {
            openssl_bn: bn
        })
    }

    pub fn from_dec(dec: &str) -> Result<BigNumber, CryptoError> {
        let bn = BigNum::from_dec_str(dec)?;
        Ok(BigNumber {
            openssl_bn: bn
        })
    }

    pub fn from_hex(hex: &str) -> Result<BigNumber, CryptoError> {
        let bn = BigNum::from_hex_str(hex)?;
        Ok(BigNumber {
            openssl_bn: bn
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<BigNumber, CryptoError> {
        let bn = BigNum::from_slice(bytes)?;
        Ok(BigNumber {
            openssl_bn: bn
        })
    }

    pub fn to_dec(&self) -> Result<String, CryptoError> {
        let result = self.openssl_bn.to_dec_str()?;
        Ok(result.to_string())
    }

    pub fn to_hex(&self) -> Result<String, CryptoError> {
        let result = self.openssl_bn.to_hex_str()?;
        Ok(result.to_string())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.openssl_bn.to_vec())
    }

    pub fn hash(data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Ok(hash2(MessageDigest::sha256(), data)?.to_vec())
    }

    pub fn add(&self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        BigNumRef::checked_add(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn)?;
        Ok(bn)
    }

    pub fn sub(&self, a: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        BigNumRef::checked_sub(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn)?;
        Ok(bn)
    }

    pub fn sqr(&self, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::sqr(&mut bn.openssl_bn, &self.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::sqr(&mut bn.openssl_bn, &self.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn mul(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::checked_mul(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::checked_mul(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn div(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::checked_div(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::checked_div(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn add_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
        BigNumRef::add_word(&mut self.openssl_bn, w)?;
        Ok(self)
    }

    pub fn sub_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
        BigNumRef::sub_word(&mut self.openssl_bn, w)?;
        Ok(self)
    }

    pub fn mul_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
        BigNumRef::mul_word(&mut self.openssl_bn, w)?;
        Ok(self)
    }

    pub fn div_word(&mut self, w: u32) -> Result<&mut BigNumber, CryptoError> {
        BigNumRef::div_word(&mut self.openssl_bn, w)?;
        Ok(self)
    }

    pub fn mod_exp(&self, a: &BigNumber, b: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::mod_exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &b.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::mod_exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &b.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn modulus(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::nnmod(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::nnmod(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn exp(&self, a: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::exp(&mut bn.openssl_bn, &self.openssl_bn, &a.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn inverse(&self, n: &BigNumber, ctx: Option<&mut BigNumberContext>) -> Result<BigNumber, CryptoError> {
        let mut bn = BigNumber::new()?;
        match ctx {
            Some(context) => BigNumRef::mod_inverse(&mut bn.openssl_bn, &self.openssl_bn, &n.openssl_bn, &mut context.openssl_bn_context)?,
            None => {
                let mut ctx = BigNumber::new_context()?;
                BigNumRef::mod_inverse(&mut bn.openssl_bn, &self.openssl_bn, &n.openssl_bn, &mut ctx.openssl_bn_context)?;
            }
        }
        Ok(bn)
    }

    pub fn mod_div(&self, b: &BigNumber, p: &BigNumber) -> Result<BigNumber, CryptoError> {
        //(a*  (1/b mod p) mod p)

        let mut context = BigNumber::new_context()?;

        let res = b
            .inverse(p, Some(&mut context))?
            .mul(&self, Some(&mut context))?
            .modulus(&p, Some(&mut context))?;
        Ok(res)
    }

    pub fn clone(&self) -> Result<BigNumber, CryptoError> {
        Ok(BigNumber {
            openssl_bn: BigNum::from_slice(&self.openssl_bn.to_vec()[..])?
        })
    }

    pub fn hash_array(nums: &Vec<Vec<u8>>) -> Result<Vec<u8>, CryptoError> {
        let mut sha256 = Hasher::new(MessageDigest::sha256())?;

        for num in nums.iter() {
            let index =
                num.iter()
                    .position(|&value| value != 0)
                    .unwrap_or(num.len());

            sha256.update(&num[index..])?;
        }

        Ok(sha256.finish2()?.to_vec())
    }
}

impl Ord for BigNumber {
    fn cmp(&self, other: &BigNumber) -> Ordering {
        self.openssl_bn.ucmp(&other.openssl_bn)
    }
}

impl Eq for BigNumber {}

impl PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &BigNumber) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigNumber {
    fn eq(&self, other: &BigNumber) -> bool {
        self.openssl_bn == other.openssl_bn
    }
}

impl BytesView for BigNumber {
    fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        Ok(self.to_bytes()?)
    }
}

impl Serialize for BigNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("BigNumber", &self.to_dec().map_err(SError::custom)?)
    }
}

impl<'a> Deserialize<'a> for BigNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'a> {
        struct BigNumberVisitor;

        impl<'a> Visitor<'a> for BigNumberVisitor {
            type Value = BigNumber;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expected BigNumber")
            }

            fn visit_str<E>(self, value: &str) -> Result<BigNumber, E>
                where E: DError
            {
                Ok(BigNumber::from_dec(value).map_err(DError::custom)?)
            }
        }

        deserializer.deserialize_str(BigNumberVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::logger::LoggerUtils;
    use services::anoncreds::constants::{
        LARGE_UTILDE,
        LARGE_RTILDE,
    };

    extern crate serde_json;

    #[test]
    fn generate_prime_in_range_works() {
        LoggerUtils::init();

        let start = BigNumber::rand(LARGE_UTILDE).unwrap();
        let end = BigNumber::rand(LARGE_RTILDE).unwrap();
        let random_prime = BigNumber::generate_prime_in_range(&start, &end).unwrap();
        assert!(start < random_prime);
        assert!(end > random_prime);
    }

    #[derive(Serialize, Deserialize)]
    struct Test {
        field: BigNumber
    }

    #[test]
    fn serialize_works() {
        let s = Test { field: BigNumber::from_dec("1").unwrap() };
        let serialized = serde_json::to_string(&s);

        assert!(serialized.is_ok());
        assert_eq!("{\"field\":\"1\"}", serialized.unwrap());
    }

    #[test]
    fn deserialize_works() {
        let s = "{\"field\":\"1\"}";
        let bn: Result<Test, _> = serde_json::from_str(&s);

        assert!(bn.is_ok());
        assert_eq!("1", bn.unwrap().field.to_dec().unwrap());
    }
}
