use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::{
    LARGE_MASTER_SECRET,
    LARGE_VPRIME,
    LARGE_MVECT,
    LARGE_E_START,
    LARGE_ETILDE,
    LARGE_VTILDE,
    LARGE_UTILDE,
    LARGE_RTILDE,
    ITERATION
};
use services::crypto::anoncreds::types::{
    PublicKey,
    ClaimInitData,
    PrimaryClaim,
    PrimaryEqualInitProof,
    PrimaryEqualProof,
    Predicate,
    PrimaryPrecicateGEInitProof,
    PrimaryPredicateGEProof,
    PrimaryInitProof,
    PrimaryProof,
    ClaimInitDataType
};
use services::crypto::wrappers::bn::BigNumber;
use std::collections::HashMap;
use services::crypto::anoncreds::verifier::Verifier;
use std::rc::Rc;

pub struct Prover {}

impl Prover {
    pub fn new() -> Prover {
        Prover {}
    }
    pub fn generate_master_secret(&self) -> Result<BigNumber, CryptoError> {
        let mut bn = try!(BigNumber::new());
        bn = try!(bn.rand(LARGE_MASTER_SECRET));
        Ok(bn)
    }

    fn gen_claim_init_data(public_key: &PublicKey, ms: &BigNumber) -> Result<ClaimInitData, CryptoError> {
        let bn = try!(BigNumber::new());
        let v_prime = try!(bn.rand(LARGE_VPRIME));

        let result_mul_one = try!(public_key.s.mod_exp(&v_prime, &public_key.n, None));

        let result_mul_two = try!(public_key.rms.mod_exp(&ms, &public_key.n, None));

        let mut u = try!(result_mul_one.mul(&result_mul_two, None));
        u = try!(u.modulus(&public_key.n, None));

        Ok(ClaimInitData {
            u: u,
            v_prime: v_prime
        })
    }

    fn prepare_primary_claim(claim_init_data: ClaimInitDataType, claim: &mut PrimaryClaim)
                             -> Result<&mut PrimaryClaim, CryptoError> {
        let new_v = try!(claim.v.add(&claim_init_data.v_prime));
        claim.v = new_v;
        Ok(claim)
    }

    fn init_proof(pk: &PublicKey, c1: Option<Rc<PrimaryClaim>>, revealed_attrs: &Vec<String>,
                  predicates: &Vec<Rc<Predicate>>, m1_t: &BigNumber, m2_t: Option<BigNumber>)
                  -> Result<Option<PrimaryInitProof>, CryptoError> {
        match c1 {
            Some(c1_val) => {
                let eq_proof = try!(Prover::init_eq_proof(pk, c1_val.clone(), revealed_attrs, m1_t, m2_t));
                let mut ge_proofs: Vec<Rc<PrimaryPrecicateGEInitProof>> = Vec::new();

                for predicate in predicates.iter() {
                    let ge_proof = try!(Prover::init_ge_proof(pk, &eq_proof, c1_val.clone(), predicate.clone()));
                    ge_proofs.push(Rc::new(ge_proof));
                }

                Ok(Some(PrimaryInitProof {
                    eq_proof: Rc::new(eq_proof),
                    ge_proofs: ge_proofs
                }))
            },
            None => Ok(None)
        }
    }

    fn finalize_proof(ms: &BigNumber, init_proof: Option<&PrimaryInitProof>, c_h: &BigNumber)
                      -> Result<Option<PrimaryProof>, CryptoError> {
        match init_proof {
            Some(init_pr) => {
                let eq_proof = try!(Prover::finalize_eq_proof(ms, &init_pr.eq_proof, c_h));
                let mut ge_proofs: Vec<Rc<PrimaryPredicateGEProof>> = Vec::new();

                for init_ge_proof in init_pr.ge_proofs.iter() {
                    let ge_proof = try!(Prover::finalize_ge_proof(c_h, init_ge_proof, &eq_proof));
                    ge_proofs.push(Rc::new(ge_proof));
                }

                Ok(Some(PrimaryProof {
                    eq_proof: Rc::new(eq_proof),
                    ge_proofs: ge_proofs
                }))
            },
            None => Ok(None)
        }
    }

    fn init_eq_proof(pk: &PublicKey, c1: Rc<PrimaryClaim>, revealed_attrs: &Vec<String>,
                     m1_tilde: &BigNumber, m2_t: Option<BigNumber>) -> Result<PrimaryEqualInitProof, CryptoError> {
        let mut ctx = try!(BigNumber::new_context());

        let m2_tilde = m2_t.unwrap_or(try!(BigNumber::new()?.rand(LARGE_MVECT)));

        let ra = try!(BigNumber::new()?.rand(LARGE_VPRIME));
        let etilde = try!(BigNumber::new()?.rand(LARGE_ETILDE));
        let vtilde = try!(BigNumber::new()?.rand(LARGE_VTILDE));

        let (_, unrevealed_attrs) = try!(Prover::split_revealed_attrs(&c1.encoded_attrs, &revealed_attrs));

        let mtilde = try!(Prover::get_mtilde(&unrevealed_attrs));

        let aprime = try!(
            pk.s
                .mod_exp(&ra, &pk.n, Some(&mut ctx))?
                .mul(&c1.a, Some(&mut ctx))?
                .modulus(&pk.n, Some(&mut ctx))
        );

        let e_mul_ra = try!(c1.e.mul(&ra, Some(&mut ctx)));
        let large_e_start = try!(BigNumber::from_dec(&LARGE_E_START.to_string()[..]));
        let two_pow_large_start: BigNumber = try!(
            BigNumber::from_dec("2")?
                .exp(&large_e_start, Some(&mut ctx))
        );

        let vprime = try!(c1.v.sub(&e_mul_ra));
        let eprime = try!(c1.e.sub(&two_pow_large_start));

        let unrevealed_attrs_keys: Vec<String> =
            unrevealed_attrs.keys().map(|k| k.to_owned()).collect::<Vec<String>>();

        let t = try!(
            Verifier::calc_teq(&pk, &aprime, &etilde, &vtilde, &mtilde, &m1_tilde, &m2_tilde, &unrevealed_attrs_keys)
        );

        Ok(
            PrimaryEqualInitProof {
                c1: c1,
                a_prime: aprime,
                t: t,
                etilde: etilde,
                eprime: eprime,
                vtilde: vtilde,
                vprime: vprime,
                mtilde: mtilde,
                m1_tilde: try!(m1_tilde.clone()),
                m2_tilde: m2_tilde,
                unrevealed_attrs: unrevealed_attrs_keys,
                revealed_attrs: revealed_attrs.to_owned()
            }
        )
    }


    fn finalize_eq_proof(ms: &BigNumber, init_proof: &PrimaryEqualInitProof, c_h: &BigNumber)
                         -> Result<PrimaryEqualProof, CryptoError> {
        let mut ctx = try!(BigNumber::new_context());

        let e = try!(
            c_h
                .mul(&init_proof.eprime, Some(&mut ctx))?
                .add(&init_proof.etilde)
        );

        let v = try!(
            c_h
                .mul(&init_proof.vprime, Some(&mut ctx))?
                .add(&init_proof.vtilde)
        );

        let mut m: HashMap<String, BigNumber> = HashMap::new();

        for k in init_proof.unrevealed_attrs.iter() {
            let cur_mtilde = try!(init_proof.mtilde.get(k)
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.mtilde", k))));
            let cur_val = try!(init_proof.c1.encoded_attrs.get(k)
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_prook.c1", k))));

            let val = try!(
                c_h
                    .mul(&cur_val, Some(&mut ctx))?
                    .add(&cur_mtilde)
            );

            m.insert(k.clone(), val);
        }

        let m1 = try!(
            c_h
                .mul(&ms, Some(&mut ctx))?
                .add(&init_proof.m1_tilde)
        );

        let m2 = try!(
            c_h
                .mul(&init_proof.c1.m2, Some(&mut ctx))?
                .add(&init_proof.m2_tilde)
        );

        Ok(PrimaryEqualProof {
            e: e,
            v: v,
            m: m,
            m1: m1,
            m2: m2,
            a_prime: try!(init_proof.a_prime.clone()),
            revealed_attr_names: init_proof.revealed_attrs.to_owned()
        })
    }

    fn init_ge_proof(pk: &PublicKey, eq_proof: &PrimaryEqualInitProof,
                     c1: Rc<PrimaryClaim>, predicate: Rc<Predicate>)
                     -> Result<PrimaryPrecicateGEInitProof, CryptoError> {
        let mut ctx = try!(BigNumber::new_context());
        let (k, value) = (&predicate.attr_name, predicate.value);

        let attr_value = try!(
            c1.encoded_attrs.get(&k[..])
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in c1.encoded_attrs", k)))?
                .to_dec()?
                .parse::<i64>());

        let delta: i64 = attr_value - value as i64;

        if delta < 0 {
            return Err(CryptoError::InvalidStructure("Predicate is not satisfied".to_string()))
        }

        let u = try!(Prover::four_squares(delta));

        let mut r: HashMap<String, BigNumber> = HashMap::new();
        let mut t: HashMap<String, BigNumber> = HashMap::new();
        let mut c_list: Vec<BigNumber> = Vec::new();

        for i in 0..ITERATION {
            let cur_u = try!(u.get(&i.to_string()[..])
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in u1", i))));

            let cur_r = try!(BigNumber::new()?.rand(LARGE_VPRIME));
            let pks_pow_r = try!(pk.s.mod_exp(&cur_r, &pk.n, Some(&mut ctx)));

            let cut_t = try!(
                pk.z
                    .mod_exp(&cur_u, &pk.n, Some(&mut ctx))?
                    .mul(&pks_pow_r, Some(&mut ctx))?
                    .modulus(&pk.n, Some(&mut ctx))
            );

            r.insert(i.to_string(), cur_r);
            t.insert(i.to_string(), try!(cut_t.clone()));
            c_list.push(cut_t)
        }

        let r_delta = try!(BigNumber::new()?.rand(LARGE_VPRIME));
        let pks_pow_r = try!(pk.s.mod_exp(&r_delta, &pk.n, Some(&mut ctx)));
        let big_delta = try!(BigNumber::from_dec(&delta.to_string()[..]));

        let t_delta = try!(
            pk.z
                .exp(&big_delta, Some(&mut ctx))?
                .mul(&pks_pow_r, Some(&mut ctx))?
                .modulus(&pk.n, Some(&mut ctx))
        );

        r.insert("DELTA".to_string(), r_delta);
        t.insert("DELTA".to_string(), try!(t_delta.clone()));
        c_list.push(t_delta);

        let mut utilde: HashMap<String, BigNumber> = HashMap::new();
        let mut rtilde: HashMap<String, BigNumber> = HashMap::new();

        for i in 0..ITERATION {
            utilde.insert(i.to_string(), try!(BigNumber::new()?.rand(LARGE_UTILDE)));
            rtilde.insert(i.to_string(), try!(BigNumber::new()?.rand(LARGE_RTILDE)));
        }

        rtilde.insert("DELTA".to_string(), try!(BigNumber::new()?.rand(LARGE_VPRIME)));
        let alphatilde = try!(BigNumber::new()?.rand(LARGE_VPRIME));

        let mj = try!(eq_proof.mtilde.get(&k[..])
            .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in eq_proof.mtilde", k))));

        let tau_list = try!(Verifier::calc_tge(&pk, &utilde, &rtilde, &mj, &alphatilde, &t));

        Ok(PrimaryPrecicateGEInitProof {
            c_list: c_list,
            tau_list: tau_list,
            u: u,
            u_tilde: utilde,
            r: r,
            r_tilde: rtilde,
            alpha_tilde: alphatilde,
            predicate: predicate.clone(),
            t: Rc::new(t)
        })
    }

    fn finalize_ge_proof(c_h: &BigNumber, init_proof: &PrimaryPrecicateGEInitProof,
                         eq_proof: &PrimaryEqualProof) -> Result<PrimaryPredicateGEProof, CryptoError> {
        let mut ctx = try!(BigNumber::new_context());
        let mut u: HashMap<String, BigNumber> = HashMap::new();
        let mut r: HashMap<String, BigNumber> = HashMap::new();
        let mut urproduct = try!(BigNumber::new());

        for i in 0..ITERATION {
            let cur_utilde = try!(init_proof.u_tilde.get(&i.to_string()[..])
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.u_tilde", i))));
            let cur_u = try!(init_proof.u.get(&i.to_string()[..])
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.u", i))));
            let cur_rtilde = try!(init_proof.r_tilde.get(&i.to_string()[..])
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r_tilde", i))));
            let cur_r = try!(init_proof.r.get(&i.to_string()[..])
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r", i))));

            let new_u: BigNumber = try!(
                c_h
                    .mul(&cur_u, Some(&mut ctx))?
                    .add(&cur_utilde)
            );
            let new_r: BigNumber = try!(
                c_h
                    .mul(&cur_r, Some(&mut ctx))?
                    .add(&cur_rtilde)
            );

            u.insert(i.to_string(), new_u);
            r.insert(i.to_string(), new_r);

            urproduct = try!(
                cur_u
                    .mul(&cur_r, Some(&mut ctx))?
                    .add(&urproduct)
            );

            let cur_rtilde_delta = try!(init_proof.r_tilde.get("DELTA")
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r_tilde", "DELTA"))));
            let cur_r_delta = try!(init_proof.r.get("DELTA")
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r", "DELTA"))));

            let new_delta = try!(
                c_h
                    .mul(&cur_r_delta, Some(&mut ctx))?
                    .add(&cur_rtilde_delta)
            );

            r.insert("DELTA".to_string(), new_delta);
        }

        let r_delta = try!(init_proof.r.get("DELTA")
            .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in init_proof.r", "DELTA"))));

        let alpha = try!(
            r_delta
                .sub(&urproduct)?
                .mul(&c_h, Some(&mut ctx))?
                .add(&init_proof.alpha_tilde)
        );

        let (k, value) = (&init_proof.predicate.attr_name, &init_proof.predicate.value);

        let mj = try!(eq_proof.m.get(&k[..])
            .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in eq_proof.m", k))));

        Ok(PrimaryPredicateGEProof {
            u: u,
            r: r,
            alpha: alpha,
            mj: try!(mj.clone()),
            t: init_proof.t.clone(),
            predicate: init_proof.predicate.clone()
        })
    }

    fn split_revealed_attrs(encoded_attrs: &HashMap<String, BigNumber>, revealed_ttrs: &Vec<String>)
                            -> Result<(HashMap<String, BigNumber>, HashMap<String, BigNumber>), CryptoError> {
        let mut ar: HashMap<String, BigNumber> = HashMap::new();
        let mut aur: HashMap<String, BigNumber> = HashMap::new();

        for (attr, value) in encoded_attrs.iter() {
            if revealed_ttrs.contains(&attr) {
                ar.insert(attr.clone(), try!(value.clone()));
            } else {
                aur.insert(attr.clone(), try!(value.clone()));
            }
        }
        Ok((ar, aur))
    }

    fn get_mtilde(unrevealed_attrs: &HashMap<String, BigNumber>)
                  -> Result<HashMap<String, BigNumber>, CryptoError> {
        let mut mtilde: HashMap<String, BigNumber> = HashMap::new();

        for (attr, _) in unrevealed_attrs.iter() {
            mtilde.insert(attr.clone(), try!(BigNumber::new()?.rand(LARGE_MVECT)));
        }
        Ok(mtilde)
    }

    fn largest_square_less_than(delta: i64) -> i64 {
        (delta as f64).sqrt().floor() as i64
    }

    fn four_squares(delta: i64) -> Result<HashMap<String, BigNumber>, CryptoError> {
        let u1 = Prover::largest_square_less_than(delta);
        let u2 = Prover::largest_square_less_than(delta - u1.pow(2));
        let u3 = Prover::largest_square_less_than(delta - u1.pow(2) - u2.pow(2));
        let u4 = Prover::largest_square_less_than(delta - u1.pow(2) - u2.pow(2) - u3.pow(2));

        if u1.pow(2) + u2.pow(2) + u3.pow(2) + u4.pow(2) == delta {
            let mut res: HashMap<String, BigNumber> = HashMap::new();
            res.insert("0".to_string(), try!(BigNumber::from_dec(&u1.to_string()[..])));
            res.insert("1".to_string(), try!(BigNumber::from_dec(&u2.to_string()[..])));
            res.insert("2".to_string(), try!(BigNumber::from_dec(&u3.to_string()[..])));
            res.insert("3".to_string(), try!(BigNumber::from_dec(&u4.to_string()[..])));

            Ok(res)
        } else {
            Err(CryptoError::InvalidStructure(format!("Cannot get the four squares for delta {} ", delta)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use services::crypto::anoncreds::verifier;

    #[test]
    fn four_squares_works() {
        let res = Prover::four_squares(10 as i64);

        assert!(res.is_ok());
        let res_data = res.unwrap();

        assert_eq!("3".to_string(), res_data.get("0").unwrap().to_dec().unwrap());
        assert_eq!("1".to_string(), res_data.get("1").unwrap().to_dec().unwrap());
        assert_eq!("0".to_string(), res_data.get("2").unwrap().to_dec().unwrap());
        assert_eq!("0".to_string(), res_data.get("3").unwrap().to_dec().unwrap());
    }

    #[test]
    fn split_revealed_attrs_works() {
        let mut encoded_attrs: HashMap<String, BigNumber> = HashMap::new();
        encoded_attrs.insert("name".to_string(), BigNumber::from_dec("1").unwrap());
        encoded_attrs.insert("age".to_string(), BigNumber::from_dec("1").unwrap());
        encoded_attrs.insert("sex".to_string(), BigNumber::from_dec("1").unwrap());

        let revealed_attrs = vec!["name".to_string()];

        let res = Prover::split_revealed_attrs(&encoded_attrs, &revealed_attrs);

        assert!(res.is_ok());

        let (revealed, unrevealed) = res.unwrap();

        assert_eq!(1, revealed.len());
        assert_eq!(2, unrevealed.len());
        assert!(revealed.contains_key("name"));
        assert!(unrevealed.contains_key("sex"));
        assert!(unrevealed.contains_key("age"));
    }

    #[test]
    fn finalize_eq_proof_works() {
        let ms = BigNumber::from_dec("12017662702207397635206788416861773342711375658894915181302218291088885004642").unwrap();
        let c_h = BigNumber::from_dec("65052515950080385170056404271846666093263620691254624189854445495335700076548").unwrap();
        let init_proof = mocks::get_primary_equal_init_proof().unwrap();

        let res = Prover::finalize_eq_proof(&ms, &init_proof, &c_h);

        assert!(res.is_ok());

        let proof = res.unwrap();

        assert_eq!("46977509037563772921188771357228696971286986611479769037400887043024357260824466323972528739266623662424083138906804114233154076462225260", proof.e.to_dec().unwrap());
        assert_eq!("555894869457553465718054497220703310113847971154321206264039643437256150021765032391630230094549373082683761872900289443108844758698210311744008775755841424663713495335913737925610645231143512448736634848872651673398623671421680147672048516992617074237823416006998805743252732623168072887558380980816786967972208697482105496476584623670241498051382948079749991653743122008317688039944886441991890739570646377897115078595023503848923611116244104325820581549132685254973230215813377280331818752749674933449141701081762918502111898869410069368198046357103361141404701610657859033620340201121860748524404546417655599945090144921881183922296151990310095095955070183524924902826674801457725425394553828477598974723668103655265677518938090134981829839176785641671819341783587890027487090232485080219343288188381028474008022615299819430842220715432262971141278304167669686965655751310509796666256987764202199558192225907485643584", proof.v.to_dec().unwrap());
        assert_eq!("17884736668674953594474879343533841182802514514784532835710262264561805009458126297222977824304362311586622997817594769134550513911169868072027461607531075593532027490623438201429184516874637111394210856531406371117724109267267829196540990374669452129657796333114585130056514558678918989249474063851032294543", proof.m1.to_dec().unwrap());
        assert_eq!("33970939655505026872690051065527896936826240486176548712174703648151652129591217103741946892383483806205993341432925544541557374346350172352729633028700080895510117255197249531019938518779850139061087723518395934746900289855498383299025412840993553136695018502936439397825288787933388062548604655707739594437", proof.m2.to_dec().unwrap());
        assert_eq!("5994573776370639993485756369006996958675983702501954416840497091005949903180887189862813008745473545649917725255704624891687505287597789027355671402389730792821549872949965312834", proof.m.get("age").unwrap().to_dec().unwrap());
    }


    #[test]
    fn init_ge_proof_works() {
        let pk = verifier::mocks::get_pk().unwrap();
        let eq_proof = mocks::get_primary_equal_init_proof().unwrap();
        let claim = mocks::get_primary_claim().unwrap();

        let predicate = Predicate { attr_name: "age".to_string(), p_type: "ge".to_string(), value: 18 };

        let res = Prover::init_ge_proof(&pk, &eq_proof, Rc::new(claim), Rc::new(predicate));
        assert!(res.is_ok());
    }

    #[test]
    fn finalize_ge_proof_works() {
        let c_h = BigNumber::from_dec("107686359310664445046126368677755391247164319345083587464043204013905993527834").unwrap();
        let ge_proof = mocks::get_primary_ge_init_proof().unwrap();
        let eq_proof = verifier::mocks::get_eq_proof().unwrap();

        let res = Prover::finalize_ge_proof(&c_h, &ge_proof, &eq_proof);

        assert!(res.is_ok());

        let proof = res.unwrap();

        assert_eq!("14530430712270780620115716831630456792731829285960002962064509786954277815652219734860240775632969505615425989813150680974232279981033881929825516835639704838509146807403579176456", proof.u.get("0").unwrap().to_dec().unwrap());
        assert_eq!("1415830066404575063558956955699897939417161777078791039926340455929989312103567388586750415279750275627689289774355989928259903201283164671369980334635402090593700202419576962251006803664979387881077329091553387025639738608978470326865096461988349436323051092921673039448207467310143157161249548690648317604663697956127142299857431279531067869166789113125108487447241380451860460435536386169606660126687136336515643267258245597749963499390882335368772524506108537160732974827392286571681871686360634706404457817326674394813236360450345475325164815205390904412548072443050097422540706146216417531228071209074620592598469883684966671309568705760392191743050877301212854432940753955279643358353605952631236345030655922045", proof.r.get("0").unwrap().to_dec().unwrap());
        assert_eq!("2909377521678119520977157959638852346549039931868195250658890196374980817755318676413066648981533034386605143040798380729872705956567376032225961933326117009011908374020093877002895162468521578763395678346621437225972600951965633549602979234732083149655058280123465723210167346545435946648092301500495871307611941306714133444462666462818882418100633983906555894992078138873969482714430788917034883079579778040749973092160959984323579215740942468398437958324399647532773947797685551797171537348210954088256282790659454179075257593928991997283548069103317735700818358235857780570873678690413979416837309542554490385517111819905278234351454124245103700468051202549165577210724696681231918320110736784038063606140146272860", proof.r.get("DELTA").unwrap().to_dec().unwrap());
        assert_eq!("44263308381149662900948673540609137605123483577985225626015193605421446490850432944403510911593807877995566074607735765400382861784877744789798777017960357051684400364048124004882741408393303775593487691064638002920853960645913535484864749193831701910596138125770720981871270085109534802728387292108961395671973015447681340852592012638839948998301809908713998541365956149792695654874324699264455657573099688614830144400409479952124271239106111005380360397720399778640177093636911827538708829123941248898780310301607124559838851222069991204870155414077086348071171421803569856093007812236846764361931252088960485440158830117131468627609450498244887243402854104282374544935516477360120294987311548247220633388905908551822949252630925854555366381978721601629564425954576926076828495554017163967076851067453147787769115012365426065129174495136", proof.alpha.to_dec().unwrap());
    }

    #[test]
    fn init_proof_works() {
        let pk = verifier::mocks::get_pk().unwrap();
        let claim = mocks::get_primary_claim().unwrap();
        let revealed_attrs = vec!["name".to_string()];
        let m1_t = BigNumber::from_dec("21544287380986891419162473617242441136231665555467324140952028776483657408525689082249184862870856267009773225408151321864247533184196094757877079561221602250888815228824796823045594410522810417051146366939126434027952941761214129885206419097498982142646746254256892181011609282364766769899756219988071473111").unwrap();
        let m2_t = BigNumber::from_dec("20019436401620609773538287054563349105448394091395718060076065683409192012223520437097245209626164187921545268202389347437258706857508181049451308664304690853807529189730523256422813648391821847776735976798445049082387614903898637627680273723153113532585372668244465374990535833762731556501213399533698173874").unwrap();

        let predicate = Predicate { attr_name: "age".to_string(), p_type: "ge".to_string(), value: 18 };
        let predicates = vec![Rc::new(predicate)];

        let res = Prover::init_proof(&pk, Some(Rc::new(claim)), &revealed_attrs, &predicates, &m1_t, Some(m2_t));

        assert!(res.is_ok());
    }

    #[test]
    fn finalize_proof_works() {
        let proof = mocks::get_primary_init_proof().unwrap();
        let ms = BigNumber::from_dec("12017662702207397635206788416861773342711375658894915181302218291088885004642").unwrap();
        let c_h = BigNumber::from_dec("107686359310664445046126368677755391247164319345083587464043204013905993527834").unwrap();

        let res = Prover::finalize_proof(&ms, Some(&proof), &c_h);

        assert!(res.is_ok());
    }
}

pub mod mocks {
    use super::*;
    use services::crypto::anoncreds::types::{PrimaryClaim, PrimaryEqualInitProof, PrimaryPrecicateGEInitProof, PrimaryInitProof};

    pub fn get_primary_claim() -> Result<PrimaryClaim, CryptoError> {
        let mut encoded_attrs = HashMap::new();
        encoded_attrs.insert("name".to_string(), try!(BigNumber::from_dec("1139481716457488690172217916278103335")));
        encoded_attrs.insert("height".to_string(), try!(BigNumber::from_dec("175")));
        encoded_attrs.insert("age".to_string(), try!(BigNumber::from_dec("28")));
        encoded_attrs.insert("sex".to_string(), try!(BigNumber::from_dec("5944657099558967239210949258394887428692050081607692519917050011144233115103")));

        Ok(PrimaryClaim {
            encoded_attrs: encoded_attrs,
            m2: try!(BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451")),
            a: try!(BigNumber::from_dec("9718041686050466417394454846401911338135485472714675418729730425836367006101286571902254065185334609278478268966285580036221254487921329959035516004179696181846182303481304972520273119065229082628152074260549403953056671718537655331440869269274745137172330211653292094784431599793709932507153005886317395811504324510211401461248180054115028194976434036098410711049411182121148080258018668634613727512389415141820208171799071602314334918435751431063443005717167277426824339725300642890836588704754116628420091486522215319582218755888011754179925774397148116144684399342679279867598851549078956970579995906560499116598")),
            e: try!(BigNumber::from_dec("259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742930098340478263817667896272954429430903")),
            v: try!(BigNumber::from_dec("6477858587997811893327035319417510316563341854132851390093281262022504586945336581881563055213337677056181844572991952555932751996898440671581814053127951224635658321050035511444952581661461627187910434460669459027627147456890732433603419064826350179660439920130024451053677588698924377810206573252996817104905392311087651297242904369072119405731178447311689527558852965235336515327317399731791386249101329130190016387606690470441587455323714369899646882695142389754346148949502193028268930628086102907423247334472635671986918166524901017034444368593822038576239079939991296769079454011618207560042821478623371046256253086080003123462245464426891261800415264830177943676315694882710793222167202116798132497210943950614123537502319388887451156451273696457920098972385375390906181570700610413812857561840771758041019799427"))
        })
    }

    pub fn get_primary_equal_init_proof() -> Result<PrimaryEqualInitProof, CryptoError> {
        let claim = try!(get_primary_claim());
        let a_prime = try!(BigNumber::from_dec("73257086769794587064099943967436413456606137933106600328493517494750494246990095654268081436982110418236942052043392353047210521286732387459211325220702233796797988644175700180272575648844736779152872382353777034795665067764357414889894540956741789789825768184852497440167487735512484852870071737572382353032530574683059753247452767913883743959993537969276743507336201600689240177338100796416706021606300904878397845520702439468069188914120053211111411367694831308267216395648656387187450864371933001748318901589141996368935626664855141812654806676458999719330682612787660793512632367212943940189704480718972567395396"));
        let t = try!(BigNumber::from_dec("44674566012490574873221338726897300898913972309497258940219569980165585727901128041268469063382008728753943624549705899352321456091543114868302412585283526922484825880307252509503073791126004302810210154078010540383153531873388989179579827245098862102426681204339454264314340301557268884832227229252811218295369187558339702047951827768806306420746905540597187171789203160885305546843423145986246941522359210926851598959302132486285824149905366986262860649723244924769182483122471613582108897710332837686070090582706144278719293684893116662729424191599602937927245245078018737281020133694291784582308345229012480867237"));
        let e_tilde = try!(BigNumber::from_dec("46977509037563772921188733388482759183920721890892331081711466073993908595985901848331080423617265862263799921846536660789175730995922544"));
        let e_prime = try!(BigNumber::from_dec("583662989559444524697883298067925567"));
        let v_tilde = try!(BigNumber::from_dec("555894869457553465718054081820422849162991995390494517944838822333270882977281784453553971006695084899118389412528359884323318041943325476169840344330169758975824153331145070636467566443788129175385227046128007984813738241967046976902336929862121881184186366859109984322069665187530843637401779413855818609802312254476734798474431968023612266562224855762384405063415925635256507132513009629860708092064413558502942291653812837032047514674344515747059720035918093216163460638260675950398390880830578559681142235013420891911126992440292399590994566860624336493535424361894744432273285682724123770355673224752107007429152867080154899799528690990463990548404671629807627523244386129350481398153531931938679507753616503159308561903414993607849227745071552258935672048341133052145284351204037153852982932148831702417091773975188604439616639047752092784493713122927003649804603056886698534968937477985617245235844137536420875188"));
        let v_prime = try!(BigNumber::from_dec("6385614367009544498316319864543758599368125535237154281129593935195304840005981562825197155593411953165678474906281926931734345545746305450155060321085033621943087275107403410421778410927175029299691621870014311758603481338163542127748609425153803125698411340444632405699004049116623822070114354834294417100495058580661465651621088982873513323615197209830002327017414747343279393904208898726365331869009344688921360397873074029215826510233949892379862093346250740392060647414939231278435894873270850369894735486668772618984555075698111243885998180015446535353880393300721921216798608648100651591884384998694753149400256499979477096295284464637015155612555162482909528968752278735282245702719302108328105954407143650479954196184276137753771191346680837180603858473130837072734570076818412628985088803641214956190551904227"));

        let mut mtilde = HashMap::new();
        mtilde.insert("height".to_string(), try!(BigNumber::from_dec("328239254444215359202413011039384400948619724582749893945860119567694733825817257718298661371570699153259493569073370961571347142344708295502263922952764273724563262628933150131")));
        mtilde.insert("age".to_string(), try!(BigNumber::from_dec("5994573776370639993485756369006996958675983702501954416840497091005949903180887189862813008745473543828447278653453840130108185675891138415974292047260253476897076003550363169490")));
        mtilde.insert("sex".to_string(), try!(BigNumber::from_dec("15676903660100599782892537672168595206472397407563160173801910200324852971115565726333098248477234261578382267257292128171302148146177177706738600849576738036322710108817954410150")));

        let m1_tilde = try!(BigNumber::from_dec("17884736668674953594474879343533841182802514514784532835710262264561805009458126297222977824304362311586622997817594769134550513911169868072027461607531074811752832872590561469149850932518336232675337827949722723740491540895259903956542158590123078908328645673377676179125379936830018221094043943562296958727"));
        let m2_tilde = try!(BigNumber::from_dec("33970939655505026872690051065527896936826240486176548712174703648151652129591217103741946892383483806205993341432925544541557374346350172352729633028700077053528659741067902223562294772771229606274461374185549251388524318740149589263256424345429891975622057372801133454251096604596597737126641279540347411289"));

        let unrevealed_attrs = vec!["height".to_string(), "age".to_string(), "sex".to_string()];
        let revealed_attrs = vec!["name".to_string()];

        Ok(PrimaryEqualInitProof {
            c1: Rc::new(claim),
            a_prime: a_prime,
            t: t,
            etilde: e_tilde,
            eprime: e_prime,
            vtilde: v_tilde,
            vprime: v_prime,
            mtilde: mtilde,
            m1_tilde: m1_tilde,
            m2_tilde: m2_tilde,
            unrevealed_attrs: unrevealed_attrs,
            revealed_attrs: revealed_attrs
        })
    }

    pub fn get_primary_ge_init_proof() -> Result<PrimaryPrecicateGEInitProof, CryptoError> {
        let mut c_list: Vec<BigNumber> = Vec::new();
        c_list.push(try!(BigNumber::from_dec("40419298688137869960380469261905532334637639358156591584198474730159922131845236332832025717302613443181736582484815352622543977612852994735900017491040605701377167257840237093127235154905233147231624795995550192527737607707481813233736307936765338317096333960487846640715651848248086837945953304627391859983207411514951469156988685936443758957189790705690990639460733132695525553505807698837031674923144499907591301228015553240722485660599743846214527228665753677346129919027033129697444096042970703607475089467398949054480185324997053077334850238886591657619835566943199882335077289734306701560214493298329372650208")));
        c_list.push(try!(BigNumber::from_dec("47324660473671124619766812292419966979218618321195442620378932643647808062884161914306007419982240044457291065692968166148732382413212489017818981907451810722427822947434701298426390923083851509190004176754308805544221591456757905034099563880547910682773230595375415855727922588298088826548392572988130537249508717978384646013947582546019729481146325021203427278860772516903057439582612008766763139310189576482839673644190743850755863703998143105224320265752122772813607076484126428361088197863213824404833756768819688779202461859342789097743829182212846809717194485567647846915198890325457736010590303357798473896700")));
        c_list.push(try!(BigNumber::from_dec("66450517869982062342267997954977032094273479808003128223349391866956221490486227999714708210796649990670474598595144373853545114810461129311488376523373030855652459048816291000188287472254577785187966494209478499264992271438571724964296278469527432908172064052750006541558566871906132838361892473377520708599782848821918665128705358243638618866198451401258608314504494676177177947997456537352832881339718141901132664969277082920274734598386059889447857289735878564021235996969965313779742103257439235693097049742098377325618673992118875810433536654414222034985875962188702260416140781008765351079345681492041353915517")));
        c_list.push(try!(BigNumber::from_dec("78070105827196661040600041337907173457854153272544487321115604386049561730740327194221314976259005306609156189248394958383576900423218823055146785779218825861357426069962919084354758074120740816717011931695486881373830741590805899909505141118332615581712873355033382526097135102214961582694467049685680521168599662570089045106588071095868679795860083477878392645086886419842393734377034091691861772354369870695105905981921915221671803577058964332747681671537519176296905411380141019477128072347200017918410813327520323098847715450370454307294123150568469231654825506721027060142669757561165103933103053528023034511606")));
        c_list.push(try!(BigNumber::from_dec("83200684536414956340494235687534491849084621311799273540992839950256544160417513543839780900524522144337818273323604172338904806642960330906344496013294511314421085013454657603118717753084155308020373268668810396333088299295804908264158817923391623116540755548965302906724851186886232431450985279429884730164260492598022651383336322153593491103199117187195782444754665111992163534318072330538584638714508386890137616826706777205862989966213285981526090164444190640439286605077153051456582398200856066916720632647408699812551248250054268483664698756596786352565981324521663234607300070180614929105425712839420242514321")));

        let mut tau_list: Vec<BigNumber> = Vec::new();
        tau_list.push(try!(BigNumber::from_dec("15140192132563983584011198891415484817238186596993071283607396936354194583335316868900705320271111009411714831320691337831872126628439138871262533224307544703281477371807698525452223425670200750605763418449125326560417154215882193420051788620324946208921285413124444012185102142014009066082073507405990774347752529726721364286432450040059237148949753473594808640751722631907871436041823113427561411327410265647850452755588149194739107401612541934957588751200713263042014153310254117194222238408605703075357183065968515077548856751608663405886764709143763920973999261289863795465373404979606051217224017793032766958811")));
        tau_list.push(try!(BigNumber::from_dec("22009325014877947630026527174200929317631472626208750791313439728894802205941501133457483305053287492055711395025700211096925855401324104745196675112371703883854338747182592204009840178348481147164357644090276358774264356146958774854024112737375489364695008508208970224155188285475467990251456534404860303212995739991780462885489625391318647267043983051823985749109827583921702054401295234951443763803867227290052184122075487663670525999631601499287795787258527407755075616126319202755499894030817914291589449384977544252255991849316879972035322419088010097341651222610917507166699253633464412656939604939686927779235")));
        tau_list.push(try!(BigNumber::from_dec("15627964533527004998432038389165000103816136005375029988964505427070988976134211606408535227344041158417145070028255238801455392103113521695579086689578896155932875705490340075005561484163012535940306402641682521571945553659305990483808164193225425501204573377669678891599593106986761315653866565476157194483433336149271900598697489496190572244872015009221591483425535935884303531919258635347941316161540221899064295767010090897562893601204666639265613355995553950307149582738593763092807462903005018385092974255197604160149549388615872030971412913398039602109611976167048531483220445501083739737215277412870810099396")));
        tau_list.push(try!(BigNumber::from_dec("69750460164463503832019239074179380223142417821933331668103242458939803887386159332871378045711353326082354712806990538579597154273250741009953395178245637905378411876747452614509288221818672025545306689963691675579404059572899417172145497523547047512889912370926674344888289753106210072610765364142940872887546059041780799075090797522396305865608421376284813869031711915938763531973096410258282809600437536302255350228578137679993463517124512267300176775839875875909783384538534171446077525775056641425609563775679897591880695823828105351526687827332736948255168213703139146311683168485769607106041873644234793657396")));
        tau_list.push(try!(BigNumber::from_dec("34132763173107445610560830841313898488394524485228364539925353006770404496634510086661879191043246497239583692381010279276417009418352322742486751048568992101518984018378013150772900354967187656947771069077786822194631197139777633372530138823901112650920148029338833974489530448873513107614207475925912746846289211981300599307572467810317763139839748754562514339971268176553099225860038153231205184249842168570757272245458275526022597007402749355980752036595066753740086758919247309876789184990621533422299096077633094437542715030347647138342894730223750339935127139185670656368946989949841411629192230558551287266526")));
        tau_list.push(try!(BigNumber::from_dec("76565683231858220413634970348355655533884193896594121193316140338326831295635725256889489793041881623418796770530744437643757750818290670869856629404442102804539779790470943920985982547126806372689451469829385561786853754658793672376584770590680698494872039421566522136994135799785364832139155336348898806149875050003083388070895449937350438703463774379388035785060136940608144835006837349223795491316522482304986804930841801932706957303647124712691616546214050336437883359026928636182057382080150720957312738870036121843132512663050961368923639527157611326078923388898194496216008348568701317636330495889266691635504")));

        let mut u: HashMap<String, BigNumber> = HashMap::new();
        u.insert("0".to_string(), try!(BigNumber::from_dec("3")));
        u.insert("1".to_string(), try!(BigNumber::from_dec("1")));
        u.insert("2".to_string(), try!(BigNumber::from_dec("0")));
        u.insert("3".to_string(), try!(BigNumber::from_dec("0")));

        let mut u_tilde = HashMap::new();
        u_tilde.insert("3".to_string(), try!(BigNumber::from_dec("16150358755672241012460695129321325864817061205875004033795225851087833314854821728249641937105666018799012422371351449632923847984317420011432438475930370578146646594276080296620")));
        u_tilde.insert("1".to_string(), try!(BigNumber::from_dec("919407332653360714123789350916436306282818598634846737462180851932618353714800404545973780338648396108988603165611273851585136854059054058096491382931469477309021233049221498113")));
        u_tilde.insert("2".to_string(), try!(BigNumber::from_dec("12947014011443544528806555912324837244278059715103522101625396652490441127598860132430239390604274414152958526164107676952222456505578632937449151556057867144023854768899064453215")));
        u_tilde.insert("0".to_string(), try!(BigNumber::from_dec("14530430712270780620115716831630456792731829285960002962064509786954277815652219734860240775632969505292366911881157345835853173947767708188332558800388942446379534765685598592954")));

        let mut r = HashMap::new();
        r.insert("3".to_string(), try!(BigNumber::from_dec("24132544754399620065431039572698185029324955788479147508951988368652141824169968012401631405197526596910936236200501256582143713616923547154109572725575025831049700191992467054494004142728014553921553626557686986621281917316088996263926122140046634865717430166998367117286676599143409419427119266152736056710053609203711125989405212726237071472139024673721365397939677743276201109255641130117429575054170206689862492630448098516389565571101329687068784027116494371890703259752175194377877183611963716122547113191413743333828140272547100543539245187448059851898592306246455570727209949211247659088241502448651714103679374008105070016373294139")));
        r.insert("1".to_string(), try!(BigNumber::from_dec("35594085114524945986639006224801730200805040269697932198069819550362676659001079845522469651677729918531683925947020457364678961154507874999789223287407843566694331672092132006386937192959717680231086106031364492343223860848813656183321276259834157693100328152560173336039125986710038567259388561327714033873384412441701350106617571828450963146214502461758094005490582378541947089847874178371413274096027707156703414573239039996352851800251963501114749923080129276591522903634133702734684169390809940285496300503809706037270335260091643596848671473612632965738250455900304403753944679890823052654248119197790585118329079277482895324313751745")));
        r.insert("2".to_string(), try!(BigNumber::from_dec("12416745370495785706540664461803499515274608347250522372751993828760489306351885826979329832840050558190176950831767527159950310255159121407314662120565985630054402779252658020076760721381778346175310011216646031116221826523234356681794951060518746570363532356465500405602755795374789457390143843942758354075220594989212432418989437209512300563151542879411125346015814671481005582531474362744461151940296407107019178307871514140216555328464170666072131235143570187183316375551189197788487022794256230528166132115181407432283810165812226326503815433275045997075793535640301266413926518752768461289738628490190972639107320352430895111692883956")));
        r.insert("0".to_string(), try!(BigNumber::from_dec("13147719687690861784642987903564117321119171978071399340721977775125245434410955945160797651733662914525457223263144997853255627605012387807755261713043301599172048697597969623088108659945671056128663376565520770635189017427518191119838455865079521045511096967890062994509991531319435529014076721106316221553877061138037619390089975320215668573127020159567603520558367466598464066051208531845265756516199237280615346212300039960390316563876371279899260670556125326105845359198862856997934813787872135942081650066851138525063820953011103923516149849171718879990909755711311066630273647571139362231496658023435123993551625990965120905367877028")));
        r.insert("DELTA".to_string(), try!(BigNumber::from_dec("27017140706603837321930128683239640314000768158256873249678565317492691240380026575901913931941056190702376634224147699776972092380298850972547700066333918991951816592945434946683483826563040675037562054977204619980251439268131171446694007072677802224789195666130332806561436046366163420230684036395638111654271698281134816476714689333767613969261806762069371304995020522349204504739989730038026877050861981423166431273260095284622132391212425440148029904651623110816052419900003918839190100781461896988942446779821380489281562762932476888984542881369286357081355126723729214222892496254014829234244943392135453620530526273515539280130914262")));

        let mut r_tilde = HashMap::new();
        r_tilde.insert("3".to_string(), try!(BigNumber::from_dec("1581310419623066984941512700585957369097463841185001482669660807480368207297113764053705737662920865913917179154960493364991851661497939487215481046202935838727534817426357413752818118478480001061422592")));
        r_tilde.insert("1".to_string(), try!(BigNumber::from_dec("12698175784092390914196064326251972665080818640176357824753635500206769181493592026455460352953871545194375704442227937145765550620924766094755145832764559452913248804386143791786806665433772526875435831")));
        r_tilde.insert("2".to_string(), try!(BigNumber::from_dec("17862530894611881146644634463381143206639453937332223200502790860790433041682100237129826201980749547269161308100519670647739748120710266271206949459654024958050006488529187007087901262025343947304658469")));
        r_tilde.insert("0".to_string(), try!(BigNumber::from_dec("2998707557005793821174408437474970579753005270493800573947732417828426843052636578438933523490696647169032669416867456683467729604860634400510897331774306232996333435200605615727332230536004853848724693")));
        r_tilde.insert("DELTA".to_string(), try!(BigNumber::from_dec("19088233876358835207419091970632588113690065223461360271820393633022806844306658668558786053764082234008649301641061865256819721316329021619475938398765638382289927962244874956969520735922406546981704352")));

        let alpha_tilde = try!(BigNumber::from_dec("44263308381149662900948673540609137605123483577985225626015193605421446490850432944403510911593807877995566074607735765405553971901390456606499786829482599516431010417531712251971394967321246775153919925111546818075969608334965840293178801177046634728971628794958354733739862829268202974391880631744795540398548558220556991011193251909350421018299683294728391990188211711336282937525988363919530945046525731631119770997772548393939963391123532107813552269482929793072647468150911792469305880140318793207179607757703958258825655827605820657411086482548357455342445528631707138831116535366105159771271994970748831148128639376843296223110470512276276476446567585975474806154081654470617634795717498851405124307682847795651436514926925739847629355175444715922870618554631909406889698383588133721911769288573078161344190971202698069599055089014"));
        let predicate = Predicate { attr_name: "age".to_string(), p_type: "ge".to_string(), value: 18 };

        let mut t = HashMap::new();
        t.insert("3".to_string(), try!(BigNumber::from_dec("78070105827196661040600041337907173457854153272544487321115604386049561730740327194221314976259005306609156189248394958383576900423218823055146785779218825861357426069962919084354758074120740816717011931695486881373830741590805899909505141118332615581712873355033382526097135102214961582694467049685680521168599662570089045106588071095868679795860083477878392645086886419842393734377034091691861772354369870695105905981921915221671803577058964332747681671537519176296905411380141019477128072347200017918410813327520323098847715450370454307294123150568469231654825506721027060142669757561165103933103053528023034511606")));
        t.insert("1".to_string(), try!(BigNumber::from_dec("47324660473671124619766812292419966979218618321195442620378932643647808062884161914306007419982240044457291065692968166148732382413212489017818981907451810722427822947434701298426390923083851509190004176754308805544221591456757905034099563880547910682773230595375415855727922588298088826548392572988130537249508717978384646013947582546019729481146325021203427278860772516903057439582612008766763139310189576482839673644190743850755863703998143105224320265752122772813607076484126428361088197863213824404833756768819688779202461859342789097743829182212846809717194485567647846915198890325457736010590303357798473896700")));
        t.insert("2".to_string(), try!(BigNumber::from_dec("66450517869982062342267997954977032094273479808003128223349391866956221490486227999714708210796649990670474598595144373853545114810461129311488376523373030855652459048816291000188287472254577785187966494209478499264992271438571724964296278469527432908172064052750006541558566871906132838361892473377520708599782848821918665128705358243638618866198451401258608314504494676177177947997456537352832881339718141901132664969277082920274734598386059889447857289735878564021235996969965313779742103257439235693097049742098377325618673992118875810433536654414222034985875962188702260416140781008765351079345681492041353915517")));
        t.insert("0".to_string(), try!(BigNumber::from_dec("40419298688137869960380469261905532334637639358156591584198474730159922131845236332832025717302613443181736582484815352622543977612852994735900017491040605701377167257840237093127235154905233147231624795995550192527737607707481813233736307936765338317096333960487846640715651848248086837945953304627391859983207411514951469156988685936443758957189790705690990639460733132695525553505807698837031674923144499907591301228015553240722485660599743846214527228665753677346129919027033129697444096042970703607475089467398949054480185324997053077334850238886591657619835566943199882335077289734306701560214493298329372650208")));
        t.insert("DELTA".to_string(), try!(BigNumber::from_dec("83200684536414956340494235687534491849084621311799273540992839950256544160417513543839780900524522144337818273323604172338904806642960330906344496013294511314421085013454657603118717753084155308020373268668810396333088299295804908264158817923391623116540755548965302906724851186886232431450985279429884730164260492598022651383336322153593491103199117187195782444754665111992163534318072330538584638714508386890137616826706777205862989966213285981526090164444190640439286605077153051456582398200856066916720632647408699812551248250054268483664698756596786352565981324521663234607300070180614929105425712839420242514321")));

        Ok(PrimaryPrecicateGEInitProof {
            c_list: c_list,
            tau_list: tau_list,
            u: u,
            u_tilde: u_tilde,
            r: r,
            r_tilde: r_tilde,
            alpha_tilde: alpha_tilde,
            predicate: Rc::new(predicate),
            t: Rc::new(t)
        })
    }

    pub fn get_primary_init_proof() -> Result<PrimaryInitProof, CryptoError> {
        let eq_proof = try!(mocks::get_primary_equal_init_proof());
        let ge_proof = try!(mocks::get_primary_ge_init_proof());

        Ok(PrimaryInitProof {
            eq_proof: Rc::new(eq_proof),
            ge_proofs: vec![Rc::new(ge_proof)]
        })
    }
}