use errors::crypto::CryptoError;
use services::crypto::anoncreds::constants::{
    LARGE_E_START,
    LARGE_E_END_RANGE,
    LARGE_MASTER_SECRET,
    LARGE_PRIME,
    LARGE_VPRIME_PRIME
};
use services::crypto::anoncreds::types::{
    Accumulator,
    AccumulatorPublicKey,
    AccumulatorSecretKey,
    Attribute,
    ByteOrder,
    ClaimRequest,
    Claims,
    NonRevocationClaim,
    PrimaryClaim,
    PublicKey,
    RevocationPublicKey,
    RevocationSecretKey,
    Schema,
    SecretKey,
    Witness
};
use services::crypto::anoncreds::helpers::{
    random_qr,
    bitwise_or_big_int,
    get_hash_as_int,
    transform_u32_to_array_of_u8
};
use services::crypto::wrappers::bn::BigNumber;
use services::crypto::wrappers::pair::{GroupOrderElement, PointG1, Pair};
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;

extern crate time;

pub struct Issuer {}

impl Issuer {
    pub fn new() -> Issuer {
        Issuer {}
    }
    pub fn generate_keys(&self, schema: Schema) -> Result<((PublicKey, SecretKey), (RevocationPublicKey, RevocationSecretKey)), CryptoError> {
        Ok((Issuer::_generate_keys(&schema)?, Issuer::_generate_revocation_keys()?))
    }

    pub fn create_claim() {
        //Generate context attribute here
    }

    fn _generate_keys(schema: &Schema) -> Result<(PublicKey, SecretKey), CryptoError> {
        let p = BigNumber::new()?.generate_safe_prime(LARGE_PRIME)?;
        let q = BigNumber::new()?.generate_safe_prime(LARGE_PRIME)?;

        let mut p_prime = p.sub(&BigNumber::from_u32(1)?)?;
        p_prime.div_word(2)?;

        let mut q_prime = q.sub(&BigNumber::from_u32(1)?)?;
        q_prime.div_word(2)?;

        let n = p.mul(&q, None)?;
        let s = random_qr(&n)?;
        let xz = Issuer::_gen_x(&p_prime, &q_prime)?;
        let mut r: HashMap<String, BigNumber> = HashMap::new();

        for attribute in &schema.attribute_names {
            let random = Issuer::_gen_x(&p_prime, &q_prime)?;
            r.insert(attribute.to_string(), s.mod_exp(&random, &n, None)?);
        }

        let z = s.mod_exp(&xz, &n, None)?;

        let rms = s.mod_exp(&Issuer::_gen_x(&p_prime, &q_prime)?, &n, None)?;
        let rctxt = s.mod_exp(&Issuer::_gen_x(&p_prime, &q_prime)?, &n, None)?;
        Ok((
            PublicKey {
                n: n,
                rms: rms,
                rctxt: rctxt,
                r: r,
                s: s,
                z: z
            },
            SecretKey {
                p: p_prime,
                q: q_prime
            }
        ))
    }

    fn _generate_revocation_keys() -> Result<(RevocationPublicKey, RevocationSecretKey), CryptoError> {
        let h = PointG1::new()?;
        let h0 = PointG1::new()?;
        let h1 = PointG1::new()?;
        let h2 = PointG1::new()?;
        let g = PointG1::new()?;
        let htilde = PointG1::new()?;
        let u = PointG1::new()?;
        let x = GroupOrderElement::new()?;
        let sk = GroupOrderElement::new()?;
        let pk = g.mul(&sk)?;
        let y = h.mul(&x)?;
        Ok((
            RevocationPublicKey {
                g: g,
                h: h,
                h0: h0,
                h1: h1,
                h2: h2,
                htilde: htilde,
                u: u,
                pk: pk,
                y: y,
                x: x
            },
            RevocationSecretKey {
                x: x,
                sk: sk
            }
        ))
    }

    pub fn issue_accumulator(pk_r: &RevocationPublicKey, accumulator_id: i32, max_claim_num: i32)
                             -> Result<(Accumulator, HashMap<i32, PointG1>, AccumulatorPublicKey, AccumulatorSecretKey), CryptoError> {
        let gamma = GroupOrderElement::new()?;
        let mut g: HashMap<i32, PointG1> = HashMap::new();
        let g_count = 2 * max_claim_num;

        for i in 0..g_count {
            if i != max_claim_num + 1 {
                let i_bytes = transform_u32_to_array_of_u8(i as u32);
                let mut pow = GroupOrderElement::from_bytes(&i_bytes)?;
                pow = gamma.pow_mod(&pow)?;
                g.insert(i, pk_r.g.mul(&pow)?);
            }
        }

        let mut z = Pair::pair(&pk_r.g, &pk_r.g)?;
        let mut pow = GroupOrderElement::from_bytes(&transform_u32_to_array_of_u8((max_claim_num + 1) as u32))?;
        pow = gamma.pow_mod(&pow)?;
        z = z.pow(&pow)?;
        let acc = PointG1::new_inf()?;
        let v: HashSet<i32> = HashSet::new();
        Ok((
            Accumulator {
                accumulator_id: accumulator_id,
                acc: acc,
                v: v,
                max_claim_num: max_claim_num,
                current_i: 1
            },
            g,
            AccumulatorPublicKey {
                z: z
            },
            AccumulatorSecretKey {
                gamma: gamma
            }
            ))
    }

    fn _issue_non_revocation_claim(accumulator: RefCell<Accumulator>, pk_r: &RevocationPublicKey,
                                   sk_r: &RevocationSecretKey, g: &HashMap<i32, PointG1>,
                                   sk_accum: &AccumulatorSecretKey, context_attribute: &BigNumber,
                                   claim_request: &ClaimRequest, seq_number: Option<i32>) -> Result<(NonRevocationClaim, RefCell<Accumulator>, i64), CryptoError> {
        if accumulator.borrow().is_full() {
            return Err(CryptoError::InvalidStructure("Accumulator is full. New one must be issued.".to_string()))
        }

        let mut i;

        if let Some(x) = seq_number {
            i = x;
        } else {
            i = accumulator.borrow().current_i;
        }

        accumulator.borrow_mut().current_i += 1;

        let vr_prime_prime = GroupOrderElement::new()?;
        let c = GroupOrderElement::new()?;
        let m2 = GroupOrderElement::from_bytes(&context_attribute.to_bytes()?)?;

        let g_i = g.get(&i)
            .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in g", i)))?;

        let sigma = pk_r.h0
            .add(&pk_r.h1.mul(&m2)?)?
            .add(&claim_request.ur)?
            .add(g_i)?
            .add(&pk_r.h2.mul(&vr_prime_prime)?)?
            .mul(&sk_r.x.add_mod(&c)?.inverse()?)?;

        let mut omega = PointG1::new_inf()?;

        for j in &accumulator.borrow().v {
            let index = accumulator.borrow().max_claim_num + 1 - j + i;
            omega = omega.add(g.get(&index)
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in g", index)))?)?;
        }

        let sigma_i = pk_r.g
            .mul(&sk_r.sk
                .add_mod(&sk_accum.gamma
                    .pow_mod(&GroupOrderElement::from_bytes(&transform_u32_to_array_of_u8(i as u32))?)?)?
                .inverse()?)?;
        let u_i = pk_r.u
            .mul(&sk_accum.gamma
                .pow_mod(&GroupOrderElement::from_bytes(&transform_u32_to_array_of_u8(i as u32))?)?)?;

        let index = accumulator.borrow().max_claim_num + 1 - i;
        accumulator.borrow_mut().acc = accumulator.borrow().acc.add(g.get(&index)
            .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in g", index)))?)?;
        accumulator.borrow_mut().v.insert(i);

        let witness = Witness {
            sigma_i: sigma_i,
            u_i: u_i,
            g_i: g_i.clone(),
            omega: omega,
            v: accumulator.borrow().v.clone()
        };

        let timestamp = time::now_utc().to_timespec().sec;
        let acc_id = accumulator.borrow().accumulator_id;

        Ok(
            (
                NonRevocationClaim {
                    accumulator_id: acc_id,
                    sigma: sigma,
                    c: c,
                    vr_prime_prime: vr_prime_prime,
                    witness: witness,
                    g_i: g_i.clone(),
                    i: i,
                    m2: m2
                },
                accumulator,
                timestamp
            )
        )
    }

    pub fn revoke(accumulator: RefCell<Accumulator>, g: &HashMap<i32, PointG1>, i: i32) -> Result<(RefCell<Accumulator>, i64), CryptoError> {
        accumulator.borrow_mut().v.remove(&i);
        let index: i32 = accumulator.borrow().max_claim_num + 1 - i;
        let element = g.get(&index)
            .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in g", index)))?;
        accumulator.borrow_mut().acc = accumulator.borrow().acc.sub(element)?;
        let timestamp = time::now_utc().to_timespec().sec;
        Ok((accumulator, timestamp))
    }

    fn _issue_primary_claim(public_key: &PublicKey, secret_key: &SecretKey, u: &BigNumber, context_attribute: &BigNumber,
                            attributes: &Vec<Attribute>) -> Result<PrimaryClaim, CryptoError> {
        let v_prime_prime = Issuer::_generate_v_prime_prime()?;
        let e_start = BigNumber::from_u32(2)?.exp(&BigNumber::from_u32(LARGE_E_START)?, None)?;
        let e_end = BigNumber::from_u32(2)?
            .exp(&BigNumber::from_u32(LARGE_E_END_RANGE)?, None)?
            .add(&e_start)?;

        let e = e_start.generate_prime_in_range(&e_start, &e_end)?;

        let encoded_attributes = Issuer::_encode_attributes(&attributes)?;
        let a = Issuer::_sign(public_key, secret_key, context_attribute, &encoded_attributes, &v_prime_prime, u, &e)?;

        Ok(PrimaryClaim {
            encoded_attributes: encoded_attributes,
            m2: context_attribute.clone()?,
            a: a,
            e: e,
            v_prime: v_prime_prime
        })
    }

    fn _generate_context_attribute(accumulator_id: &String, user_id: &String) -> Result<BigNumber, CryptoError> {
        let accumulator_id_encoded = Issuer::_encode_attribute(&accumulator_id, ByteOrder::Little)?;
        let user_id_encoded = Issuer::_encode_attribute(&user_id, ByteOrder::Little)?;
        let mut s = vec![
            bitwise_or_big_int(&accumulator_id_encoded, &user_id_encoded)?.to_bytes()?
        ];
        let pow_2 = BigNumber::from_u32(2)?.exp(&BigNumber::from_u32(LARGE_MASTER_SECRET)?, None)?;
        let h = get_hash_as_int(&mut s)?
            .modulus(&pow_2, None)?;
        Ok(h)
    }

    fn _sign(public_key: &PublicKey, secret_key: &SecretKey, context_attribute: &BigNumber,
             attributes: &HashMap<String, BigNumber>, v: &BigNumber, u: &BigNumber, e: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut rx = BigNumber::from_u32(1)?;
        let mut context = BigNumber::new_context()?;

        for (key, value) in attributes {
            let pk_r = public_key.r.get(key)
                .ok_or(CryptoError::InvalidStructure(format!("Value by key '{}' not found in pk.r", key)))?;
            rx = rx.mul(
                &pk_r.mod_exp(&value, &public_key.n, Some(&mut context))?,
                Some(&mut context)
            )?;
        }

        let pk_rctxt_pow = public_key.rctxt.mod_exp(&context_attribute, &public_key.n, Some(&mut context))?;
        rx = rx.mul(&pk_rctxt_pow, Some(&mut context))?;

        if u != &BigNumber::from_u32(0)? {
            let module = u.modulus(&public_key.n, Some(&mut context))?;
            rx = rx.mul(&module, Some(&mut context))?;
        }

        let n = secret_key.p.mul(&secret_key.q, Some(&mut context))?;
        let mut e_inverse = e.modulus(&n, Some(&mut context))?;

        let mut a = public_key.s
            .mod_exp(&v, &public_key.n, Some(&mut context))?
            .mul(&rx, Some(&mut context))?;
        a = public_key.z.mod_div(&a, &public_key.n)?;

        e_inverse = e_inverse.inverse(&n, Some(&mut context))?;
        a = a.mod_exp(&e_inverse, &public_key.n, Some(&mut context))?;

        Ok(a)
    }

    fn _encode_attribute(attribute: &str, byte_order: ByteOrder) -> Result<BigNumber, CryptoError> {
        let mut result = BigNumber::hash(&attribute.as_bytes())?;
        let index = result.iter().position(|&value| value == 0);
        if let Some(position) = index {
            result.truncate(position);
        }
        if let ByteOrder::Little = byte_order {
            result.reverse();
        }
        Ok(BigNumber::from_bytes(&result)?)
    }

    fn _encode_attributes(attributes: &Vec<Attribute>) -> Result<HashMap<String, BigNumber>, CryptoError> {
        let mut encoded_attributes: HashMap<String, BigNumber> = HashMap::new();
        for i in attributes {
            if i.encode {
                encoded_attributes.insert(i.name.clone(), Issuer::_encode_attribute(&i.value, ByteOrder::Big)?);
            } else {
                encoded_attributes.insert(i.name.clone(), BigNumber::from_dec(&i.value.to_string())?);
            }
        }
        Ok(encoded_attributes)
    }

    fn _gen_x(p: &BigNumber, q: &BigNumber) -> Result<BigNumber, CryptoError> {
        let mut result = p
            .mul(&q, None)?
            .sub_word(3)?
            .rand_range()?;
        result.add_word(2)?;

        Ok(result)
    }

    fn _generate_v_prime_prime() -> Result<BigNumber, CryptoError> {
        let a = BigNumber::rand(LARGE_VPRIME_PRIME)?;
        let b = BigNumber::from_u32(2)?
            .exp(&BigNumber::from_u32(LARGE_VPRIME_PRIME - 1)?, None)?;
        let v_prime_prime = bitwise_or_big_int(&a, &b)?;
        Ok(v_prime_prime)
    }

    pub fn issue_claim(&self, pk: PublicKey, sk: SecretKey, accumulator_id: String, user_id: String,
                       claim_request: ClaimRequest, attributes: Vec<Attribute>) -> Result<Claims, CryptoError> {
        let context = Issuer::_generate_context_attribute(&accumulator_id, &user_id)?;
        let c1 = Issuer::_issue_primary_claim(&pk, &sk, &claim_request.u, &context, &attributes)?;

        Ok(Claims {
            primary_claim: c1,
            non_revocation_claim: None
        })
    }

    //        fn issue_claims(&self, data: Vec<(&PublicKey, &SecretKey, &String, &String, Rc<ClaimRequest>, Rc<Vec<Rc<Attribute>>>)>)
    //                        -> Result<HashMap<Rc<Schema>, Claims>, CryptoError> {
    //            let mut res: Vec<Claims> = Vec::new();
    //
    //            for d in data {
    //                let (pk, sk, accumulator_id, user_id, claim_req, attributes) = d;
    //                let context = Issuer::_generate_context_attribute(accumulator_id, user_id)?;
    //
    //                res.push(
    //                    Issuer::issue_claim(&self, pk, sk, accumulator_id, user_id, claim_req.clone(), attributes.clone())?
    //                );
    //            }
    //            Ok(res)
    //        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_attribute_works_short_hash() {
        let test_str = "Alexer5435";
        let test_answer = "62794";
        assert_eq!(test_answer, Issuer::_encode_attribute(test_str, ByteOrder::Big).unwrap().to_dec().unwrap());
    }

    #[test]
    fn encode_attribute_works_long_hash() {
        let test_str = "Alexer";
        let test_answer = "93838255634171043313693932530283701522875554780708470423762684802192372035729";
        assert_eq!(test_answer, Issuer::_encode_attribute(test_str, ByteOrder::Big).unwrap().to_dec().unwrap());
    }

    #[test]
    fn encode_attributes_works() {
        assert_eq!(mocks::get_encoded_attributes().unwrap(), Issuer::_encode_attributes(&mocks::get_attributes()).unwrap());
    }

    #[test]
    fn generate_context_attribute_works() {
        let accumulator_id = "110".to_string();
        let user_id = "111".to_string();
        let answer = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();
        let result = Issuer::_generate_context_attribute(&accumulator_id, &user_id).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn sign_works() {
        let public_key = mocks::get_pk().unwrap();
        let secret_key = mocks::get_secret_key();
        let context_attribute = BigNumber::from_dec("59059690488564137142247698318091397258460906844819605876079330034815387295451").unwrap();
        let attributes = mocks::get_encoded_attributes().unwrap();
        let v = BigNumber::from_dec("5237513942984418438429595379849430501110274945835879531523435677101657022026899212054747703201026332785243221088006425007944260107143086435227014329174143861116260506019310628220538205630726081406862023584806749693647480787838708606386447727482772997839699379017499630402117304253212246286800412454159444495341428975660445641214047184934669036997173182682771745932646179140449435510447104436243207291913322964918630514148730337977117021619857409406144166574010735577540583316493841348453073326447018376163876048624924380855323953529434806898415857681702157369526801730845990252958130662749564283838280707026676243727830151176995470125042111348846500489265328810592848939081739036589553697928683006514398844827534478669492201064874941684905413964973517155382540340695991536826170371552446768460042588981089470261358687308").unwrap();
        let u = BigNumber::from_dec("72637991796589957272144423539998982864769854130438387485781642285237707120228376409769221961371420625002149758076600738245408098270501483395353213773728601101770725294535792756351646443825391806535296461087756781710547778467803194521965309091287301376623972321639262276779134586366620773325502044026364814032821517244814909708610356590687571152567177116075706850536899272749781370266769562695357044719529245223811232258752001942940813585440938291877640445002571323841625932424781535818087233087621479695522263178206089952437764196471098717335358765920438275944490561172307673744212256272352897964947435086824617146019").unwrap();
        let e = BigNumber::from_dec("259344723055062059907025491480697571938277889515152306249728583105665800713306759149981690559193987143012367913206299323899696942213235956742930214202955935602153431795703076242907").unwrap();
        let result = BigNumber::from_dec("18970881790876593286488783486386867538450674270137197011105008151201183300028283403854725282778638150217936721942434319741164063687946275930536223863520768657672755664180955901543160149915323325151339912941454195063854083578091043058101001054089316795088554097754632405106453701959655043761308676687984722831097067744306280339099944309055300662730322057853217855619342132319369757252485139011180518031078822262681093763592682724354563150664662385847044702450408149239372444565988153918412684418519832197112374827438788434448252992414094101094582772269873015514685057917124494501480003311040042093731740782916169155664").unwrap();
        assert_eq!(result, Issuer::_sign(&public_key, &secret_key, &context_attribute, &attributes, &v, &u, &e).unwrap());
    }
}

pub mod mocks {
    use super::*;

    pub fn get_attributes() -> Vec<Attribute> {
        let attributes: Vec<Attribute> = vec![
            Attribute {
                name: "name".to_string(),
                value: "Alex".to_string(),
                encode: true
            },
            Attribute {
                name: "age".to_string(),
                value: "28".to_string(),
                encode: false
            },
            Attribute {
                name: "sex".to_string(),
                value: "male".to_string(),
                encode: true
            },
            Attribute {
                name: "height".to_string(),
                value: "175".to_string(),
                encode: false
            }
        ];
        attributes
    }

    pub fn get_encoded_attributes() -> Result<HashMap<String, BigNumber>, CryptoError> {
        let mut encoded_attributes: HashMap<String, BigNumber> = HashMap::new();
        encoded_attributes.insert("name".to_string(), BigNumber::from_dec("1139481716457488690172217916278103335")?);
        encoded_attributes.insert("age".to_string(), BigNumber::from_dec("28")?);
        encoded_attributes.insert("sex".to_string(), BigNumber::from_dec("5944657099558967239210949258394887428692050081607692519917050011144233115103")?);
        encoded_attributes.insert("height".to_string(), BigNumber::from_dec("175")?);
        Ok(encoded_attributes)
    }

    pub fn get_secret_key() -> SecretKey {
        SecretKey {
            p: BigNumber::from_dec("157329491389375793912190594961134932804032426403110797476730107804356484516061051345332763141806005838436304922612495876180233509449197495032194146432047460167589034147716097417880503952139805241591622353828629383332869425029086898452227895418829799945650973848983901459733426212735979668835984691928193677469").unwrap(),
            q: BigNumber::from_dec("151323892648373196579515752826519683836764873607632072057591837216698622729557534035138587276594156320800768525825023728398410073692081011811496168877166664537052088207068061172594879398773872352920912390983199416927388688319207946493810449203702100559271439586753256728900713990097168484829574000438573295723").unwrap()
        }
    }

    pub fn get_pk() -> Result<PublicKey, CryptoError> {
        let mut r = HashMap::new();
        r.insert("name".to_string(), BigNumber::from_dec("55636937636844819812189791288187243913404055721058334520072574568680438360936320682628189506248931475232504868784141809162526982794777886937554791279646171992316154768489491205932973020390955775825994246509354890417980543491344959419958264200222321573290332068573840656874584148318471805081070819330139498643368112616125508016850665039138240007045133711819182960399913468566074586611076818097815310939823561848962949647054263397457358507697316036204724311688330058092618087260011626918624130336633163118234963001890740389604366796070789463043007475519162863457847133916866147682877703700016314519649272629853810342756")?);
        r.insert("height".to_string(), BigNumber::from_dec("32014206266070285395118493698246684536543402308857326229844369749153998025988120078148833919040926762489849787174726278317154939222455553684674979640533728771798727404529140716275948809394914126446467274094766630776034154814466245563241594664595503357965283703581353868787640425189228669159837529621065262578472511140258233443082035493432067002995028424708181638248338655901732889892559561796172833245307347288440850886016760883963087954594369665160758244185860669353304463245326602784567519981372129418674907732019485821481470791951576038671383506105840172336020165255666872489673679749492975692222529386986002548508")?);
        r.insert("age".to_string(), BigNumber::from_dec("5573886601587513393941805393558438475134278869721908377896820376573868172897985632537697650826768061917733566546691785934393119648542993289296693181509209448802827620254572500988956963540401872482092959068516484681223765164669694589952326903719257213107559712016680752042520470482095682948519795635218252370953948099226141669796718651544648226881826585169101432801215379161624527044414118535373924688074790569833168081423701512430033511620744395776217769497965549575153091462845485986562792539143519413414753164756782101386489471333391388468474082175228293592033872018644198196278046021752128670441648674265160079365")?);
        r.insert("sex".to_string(), BigNumber::from_dec("44319112097252841415305877008967513656231862316131581238409828513703699212059952418622049664178569730633939544882861264006945675755509881864438312327074402062963599178195087536260752294006450133601248863198870283839961116512248865885787100775903023034879852152846002669257161013317472827548494571935048240800817870893700771269978535707078640961353407573194897812343272563394036737677668293122931520603798620428922052839619195929427039933665104815440476791376703125056734891504425929510493567119107731184250744646520780647416583157402277832961026300695141515177928171182043898138863324570665593349095177082259229019129")?);

        let n = BigNumber::from_dec("95230844261716231334966278654105782744493078250034916428724307571481648650972254096365233503303500776910009532385733941342231244809050180342216701303297309484964627111488667613567243812137828734726055835536190375874228378361894062875040911721595668269426387378524841651770329520854646198182993599992246846197622806018586940960824812499707703407200235006250330376435395757240807360245145895448238973940748414130249165698642798758094515234629492123379833360060582377815656998861873479266942101526163937107816424422201955494796734174781894506437514751553369884508767256335322189421050651814494097369702888544056010606733")?;
        let s = BigNumber::from_dec("83608735581956052060766602122241456047092927591272898317077507857903324472083195301035502442829713523495655160192410742120440247481077060649728889735943333622709039987090137325037494001551239812739256925595650405403616377574225590614582056226657979932825031688262428848508620618206304014287232713708048427099425348438343473342088258502098208531627321778163620061043269821806176268690486341352405206188888371253713940995260309747672937693391957731544958179245054768704977202091642139481745073141174316305851938990898215928942632876267309335084279137046749673230694376359278715909536580114502953378593787412958122696491")?;
        let rms = BigNumber::from_dec("12002410972675035848706631786298987049295298281772467607461994087192649160666347028767622091944497528304565759377490497287538655369597530498218287879384450121974605678051982553150980093839175365101087722528582689341030912237571526676430070213849160857477430406424356131111577547636360346507596843363617776545054084329725294982409132506989181200852351104199115448152798956456818387289142907618956667090125913885442746763678284193811934837479547315881192351556311788630337391374089308234091189363160599574268958752271955343795665269131980077642259235693653829664040302092446308732796745472579352704501330580826351662240")?;
        let rctxt = BigNumber::from_dec("77129119521935975385795386930301402827628026853991528755303486255023263353142617098662225360498227999564663438861313570702364984107826653399214544314002820732458443871729599318191904265844432709910182014204478532265518566229953111318413830009256162339443077098917698777223763712267731802804425167444165048596271025553618253855465562660530445682078873631967934956107222619891473818051441942768338388425312823594456990243766677728754477201176089151138798586336262283249409402074987943625960454785501038059209634637204497573094989557296328178873844804605590768348774565136642366470996059740224170274762372312531963184654")?;
        let z = BigNumber::from_dec("55164544925922114758373643773121488212903100773688663772257168750760838562077540114734459902014369305346806516101767509487128278169584105585138623374643674838487232408713159693511105298301789373764578281065365292802332455328842835614608027129883137292324033168485729810074426971615144489078436563295402449746541981155232849178606822309310700682675942602404109375598809372735287212196379089816519481644996930522775604565458855945697714216633192192613598668941671920105596720544264146532180330974698466182799108850159851058132630467033919618658033816306014912309279430724013987717126519405488323062369100827358874261055")?;

        Ok(PublicKey { n: n, r: r, s: s, rms: rms, rctxt: rctxt, z: z })
    }
}