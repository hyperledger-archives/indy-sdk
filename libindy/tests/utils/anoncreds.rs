use indy::api::ErrorCode;
use indy::api::anoncreds::*;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;
use utils::types::{
    ProofClaimsJson,
    ClaimInfo
};
use utils::test::TestUtils;

use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;
use std::collections::HashSet;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use utils::constants::*;


pub struct AnoncredsUtils {}

static mut WALLET_HANDLE: i32 = 0;
static mut CLAIM_DEF_JSON: &'static str = "";
pub const COMMON_MASTER_SECRET: &'static str = "common_master_secret_name";
pub const COMMON_POLICY_ADDRESS: &'static str = "114356529703218070977209757375038280327138143043673463359702632758673448977019";

impl AnoncredsUtils {
    pub fn issuer_create_claim_definition(wallet_handle: i32, issuer_did: &str, schema: &str, signature_type: Option<&str>, create_non_revoc: bool) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_def_json| {
            sender.send((err, claim_def_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_definition_cb(cb);

        let schema = CString::new(schema).unwrap();
        let signature_type_str = signature_type.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let issuer_did = CString::new(issuer_did).unwrap();

        let err =
            indy_issuer_create_and_store_claim_def(command_handle,
                                                   wallet_handle,
                                                   issuer_did.as_ptr(),
                                                   schema.as_ptr(),
                                                   if signature_type.is_some() { signature_type_str.as_ptr() } else { null() },
                                                   create_non_revoc,
                                                   cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_def_json) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_def_json)
    }

    pub fn prover_create_master_secret(wallet_handle: i32, master_secret_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_master_secret_cb(cb);

        let master_secret_name = CString::new(master_secret_name).unwrap();

        let err = indy_prover_create_master_secret(command_handle,
                                                   wallet_handle,
                                                   master_secret_name.as_ptr(),
                                                   cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_store_claim_offer(wallet_handle: i32, claim_offer_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_claim_offer_json_cb(cb);

        let claim_offer_json = CString::new(claim_offer_json).unwrap();

        let err = indy_prover_store_claim_offer(command_handle,
                                                wallet_handle,
                                                claim_offer_json.as_ptr(),
                                                cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_get_claim_offers(wallet_handle: i32, filter_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_offers_json| {
            sender.send((err, claim_offers_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claim_offers_cb(cb);

        let filter_json = CString::new(filter_json).unwrap();

        let err = indy_prover_get_claim_offers(command_handle,
                                               wallet_handle,
                                               filter_json.as_ptr(),
                                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_offers_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_offers_json)
    }

    pub fn prover_create_and_store_claim_req(wallet_handle: i32, prover_did: &str, claim_offer_json: &str,
                                             claim_def_json: &str, master_secret_name: &str, policy_address: Option<&str>) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_req_json| {
            sender.send((err, claim_req_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_claim_req_cb(cb);

        let prover_did = CString::new(prover_did).unwrap();
        let claim_offer_json = CString::new(claim_offer_json).unwrap();
        let claim_def_json = CString::new(claim_def_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();
        let policy_address_str = policy_address.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_prover_create_and_store_claim_req(command_handle,
                                                         wallet_handle,
                                                         prover_did.as_ptr(),
                                                         claim_offer_json.as_ptr(),
                                                         claim_def_json.as_ptr(),
                                                         master_secret_name.as_ptr(),
                                                         if policy_address.is_some() { policy_address_str.as_ptr() } else { null() },
                                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_req_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_req_json)
    }

    pub fn issuer_create_claim(wallet_handle: i32, claim_req_json: &str, claim_json: &str, user_revoc_index: Option<i32>) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
            sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_cb(cb);

        let claim_req_json = CString::new(claim_req_json).unwrap();
        let claim_json = CString::new(claim_json).unwrap();

        let err = indy_issuer_create_claim(command_handle,
                                           wallet_handle,
                                           claim_req_json.as_ptr(),
                                           claim_json.as_ptr(),
                                           user_revoc_index.unwrap_or(-1),
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, revoc_reg_update_json, xclaim_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((revoc_reg_update_json, xclaim_json))
    }

    pub fn prover_store_claim(wallet_handle: i32, claims_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_store_claim_cb(cb);

        let claims_json = CString::new(claims_json).unwrap();

        let err = indy_prover_store_claim(command_handle,
                                          wallet_handle,
                                          claims_json.as_ptr(),
                                          cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_get_claims(wallet_handle: i32, filter_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claims_json| {
            sender.send((err, claims_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims(cb);

        let filter_json = CString::new(filter_json).unwrap();

        let err = indy_prover_get_claims(command_handle,
                                         wallet_handle,
                                         filter_json.as_ptr(),
                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claims_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claims_json)
    }

    pub fn prover_get_claims_for_proof_req(wallet_handle: i32, proof_request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claims_json| {
            sender.send((err, claims_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(cb);

        let proof_request_json = CString::new(proof_request_json).unwrap();

        let err = indy_prover_get_claims_for_proof_req(command_handle,
                                                       wallet_handle,
                                                       proof_request_json.as_ptr(),
                                                       cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claims_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claims_json)
    }

    pub fn prover_create_proof(wallet_handle: i32, proof_req_json: &str, requested_claims_json: &str,
                               schemas_json: &str, master_secret_name: &str, policy_address: &str, claim_defs_json: &str,
                               revoc_regs_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, proof_json| {
            sender.send((err, proof_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(cb);

        let proof_req_json = CString::new(proof_req_json).unwrap();
        let requested_claims_json = CString::new(requested_claims_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();
        let policy_address = CString::new(policy_address).unwrap();
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = indy_prover_create_proof(command_handle,
                                           wallet_handle,
                                           proof_req_json.as_ptr(),
                                           requested_claims_json.as_ptr(),
                                           schemas_json.as_ptr(),
                                           master_secret_name.as_ptr(),
                                           policy_address.as_ptr(),
                                           claim_defs_json.as_ptr(),
                                           revoc_regs_json.as_ptr(),
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, proof_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(proof_json)
    }

    pub fn verifier_verify_proof(proof_request_json: &str, proof_json: &str,
                                 schemas_json: &str, claim_defs_json: &str, revoc_regs_json: &str) -> Result<bool, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, valid| {
            sender.send((err, valid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_verifier_verify_proof_cb(cb);

        let proof_request_json = CString::new(proof_request_json).unwrap();
        let proof_json = CString::new(proof_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = indy_verifier_verify_proof(command_handle,
                                             proof_request_json.as_ptr(),
                                             proof_json.as_ptr(),
                                             schemas_json.as_ptr(),
                                             claim_defs_json.as_ptr(),
                                             revoc_regs_json.as_ptr(),
                                             cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, valid) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(valid)
    }

    pub fn indy_issuer_create_and_store_revoc_reg(wallet_handle: i32, issuer_did: &str, schema_seq_no: i32, max_claim_num: i32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_json| {
            sender.send((err, revoc_reg_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_and_store_revoc_reg_cb(cb);

        let issuer_did = CString::new(issuer_did).unwrap();

        let err = indy_issuer_create_and_store_revoc_reg(command_handle,
                                                         wallet_handle,
                                                         issuer_did.as_ptr(),
                                                         schema_seq_no,
                                                         max_claim_num,
                                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, revoc_reg_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(revoc_reg_json)
    }

    pub fn issuer_revoke_claim(wallet_handle: i32, issuer_did: &str, schema_seq_no: i32, user_revoc_index: i32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_update_json| {
            sender.send((err, revoc_reg_update_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_revoke_claim_cb(cb);

        let issuer_did = CString::new(issuer_did).unwrap();

        let err = indy_issuer_revoke_claim(command_handle,
                                           wallet_handle,
                                           issuer_did.as_ptr(),
                                           schema_seq_no,
                                           user_revoc_index,
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, revoc_reg_update_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(revoc_reg_update_json)
    }

    pub fn get_composite_id(issuer_did: &str, schema_seq_no: i32) -> String {
        issuer_did.to_string() + ":" + &schema_seq_no.to_string()
    }

    pub fn get_gvt_schema_json(schema_seq_no: i32) -> String {
        format!(r#"{{
                    "seqNo":{},
                    "data": {{
                        "name":"gvt",
                        "version":"1.0",
                        "attr_names":["age","sex","height","name"]
                    }}
                 }}"#, schema_seq_no)
    }

    pub fn get_xyz_schema_json(schema_seq_no: i32) -> String {
        format!(r#"{{
                    "seqNo":{},
                    "data":{{
                        "name":"xyz",
                        "version":"1.0",
                        "attr_names":["status","period"]
                    }}
                 }}"#, schema_seq_no)
    }

    pub fn get_claim_offer(issuer_did: &str, schema_seq_no: i32) -> String {
        format!(r#"{{"issuer_did":"{}","schema_seq_no":{} }}"#,
                issuer_did, schema_seq_no)
    }

    pub fn get_gvt_claim_json() -> String {
        r#"{
               "sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],
               "name":["Alex","1139481716457488690172217916278103335"],
               "height":["175","175"],
               "age":["28","28"]
        }"#.to_string()
    }

    pub fn get_xyz_claim_json() -> String {
        r#"{
               "status":["partial","51792877103171595686471452153480627530895"],
               "period":["8","8"]
        }"#.to_string()
    }

    pub fn get_gvt_claim_def_data_json() -> String {
        r#"{
            "primary":{
                "n":"83469852984476956871633111285697420678256060723156580163068122759469567425381600849138438902552107548539766861666590365174848381535291010418041757276710240953030842046122202402016906205924972182252295487319094577329593677544393592632224714613427822130473474379696616183721440743475053734247824037725487533789856061706740833324717788602268746116297029721621398888459529131593826880823126900285858832457134377949183677639585442886904844793608783831753240185678448312284269486845497720949217396146132958861735347072722092449280372574205841746312833280031873247525372459800132930201998084029506922484661426185450002143461",
                "s":"36598527821865478336201608644402887021319976830281254144922838415193047189326184120876650311572665920640111967758238425066565864958446810892401358531545590342401290056836149703549220109981509774843525259400920352082531560361277411808530872594109525982462491998670199872903823657869742599086495624835178373073050767142081484206776345277546531080450529061958937980460303537107061046725579009809137197541389237618812289642185603461102513124991949835948586623327143696280240600789928383168220919049794681181861776889681393339729944540218566460627715413465709316412838042632482652979005394086058441511591756153781159121227",
                "rms":"23836382972046033209463023456985914927629254782286444334728987813724281765327660893337383450653748691133061782449580026414785334582859397732571499366000805280757877601804441568263743400086744823885203441870748890135445454347495577599234477813361254101857848089907496868136222777024967328411073984887059985103475100012787049491016895625234124538894645853939428009610771524580099452739392988318945585946758355611531582519514003714424216836334706370901576611410508113637778751976890941210538254418937285168453791223070083264852447713991114719905171445881819334587600693321106919667204512182250084258986170095774914769107",
                "r":{
                    "age":"15428480888651268593621235736458685943389726269437020388313417035842991073151072061010468945249435098482625869236498750525662874597991333642865524104221652457788998109101749530884821300954337535472137069380551054204373136155978715752232238326100335828797868667735730830741789880726890058203015780792583471770404478023662994191588489722949795849990796063953164194432710764145637578113087142419634074378464118254848566088943085760634805903735300398689750649630456000759025366784986694635635510206166144055869823907120081668956271923743188342071093889666111639924270726451727101864752767708690529389259470017692442002767",
                    "name":"74008461204977705404956807338714891429397387365673402608947856456696416827848931951447004905350314563364442667940680669672331872875260077257474781261367591510351742401708951175978700805098470304211391452678992053755170054677498844656517106987419550598382601263743442309896367374279461481792679346472671426558385003925845431219156475505665973289508268634194964491418902859845301351562575713510002838692825728016254605821829245646855474149449192539144107522081712005891593405826343897070114168186645885993480245755494685105636168333649181939830898090651120926156152753918086493335382129839850609934233944397884745134858",
                    "sex":"40646934914193532541511585946883243600955533193734077080129022860038019728021796610599139377923881754708640252789475144625086755150150612623804964347576907276705600241268266301487516824189453869193926251791711672689649199860304727280764676403810510047397326018392955950249975529169980045664874433828266623495515931483137318724210483205730962036282580749206735450480976847188040030165278917936054139926609849181885654646269083459580415131952938813839182742590617440550773580790446467896469090164142755499827316294406540664375065617280568303837662431668218593808092907551353093044984225946834165309588512359379032847125",
                    "height":"60077310006190228182950362501472785016827894350517184186566050586806482282196022414888288252599211919680339352529750982779980002923071031258837648242708410943080288964834346858544959217874890558006056551949543916094446891954292824146212277550956558692224016968153138097595802008943263818064605343108607131298420107448075252583276684858815371561492996587478784667827675142382692061950832554910690663724101360454494298013020706080246986445114235542283015624510836206522238238728405826712730615187235709554561445384409566940622412591208469650855059870671771721035756205878334354791247751663679130847366069215369484993653"
                },
                "rctxt":"36378575722516953828830668112614685244571602424420162720244033008706985740860180373728219883172046821464173434592331868657297711725743060654773725561634332269874655603697872022630999786617840856366807034806938874090561725454026277048301648000835861491030368037108847175790943895107305383779598585532854170748970999977490244057635358075906501562932970296830906796719844887269636297064678777638050708353254894155336111384638276041851818109156194135995350001255928374102089400812445206030019103440866343736761861049159446083221399575945128480681798837648578166327164995640582517916904912672875184928940552983440410245037",
                "z":"65210811645114955910002482704691499297899796787292244564644467629838455625296674951468505972574512639263601600908664306008863647466643899294681985964775001929521624341158696866597713112430928402519124073453804861185882073381514901830347278653016300430179820703804228663001232136885036042101307423527913402600370741689559698469878576457899715687929448757963179899698951620405467976414716277505767979494596626867505828267832223147104774684678295400387894506425769550011471322118172087199519094477785389750318762521728398390891214426117908390767403438354009767291938975195751081032073309083309746656253788033721103313036"
            }
        }"#.to_string()
    }

    pub fn get_gvt_claim_req() -> String {
        format!(r#"{{
            "blinded_ms":{{
                "prover_did":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                "u":"72052674960029442327236458752017934128206007798774128392572211954456711136771871346204637748253860917837147111221378456345006764308173447177933384497678611527908801900335623480700015849806575534757455484512742315652166882850683721692964547448843598104385874050447011820051099399087175505815748958014671544911179795524159951193233504921329404534187047046492036161628814022862661479869322137573048331473599346645871295570237032991261433025344456232326409789544299441933427561947291495434188942844516539974096858281005872862193803356400358925349350554630231733687344283622639185011395343616612151755685912869590344206893",
                "ur":null
            }},
            "issuer_did":"{}",
            "schema_seq_no":1
        }}"#, ISSUER_DID)
    }

    pub fn get_unique_claims(proof_claims: &ProofClaimsJson) -> Vec<ClaimInfo> {
        let attrs_claims =
            proof_claims.attrs
                .values()
                .flat_map(|claims| claims)
                .map(|claim| claim.clone())
                .collect::<Vec<ClaimInfo>>();

        let predicates_claims =
            proof_claims.predicates
                .values()
                .flat_map(|claims| claims)
                .map(|claim| claim.clone())
                .collect::<Vec<ClaimInfo>>();

        attrs_claims.into_iter().collect::<HashSet<ClaimInfo>>()
            .union(&predicates_claims.into_iter().collect::<HashSet<ClaimInfo>>())
            .map(|v| v.clone()).collect::<Vec<ClaimInfo>>()
    }

    pub fn init_common_wallet() -> (i32, &'static str) {
        lazy_static! {
            static ref COMMON_WALLET_INIT: Once = ONCE_INIT;

        }

        unsafe {
            COMMON_WALLET_INIT.call_once(|| {
                //TODO Need clean after tests but not exists After function in Cargo
                TestUtils::cleanup_storage();

                //1. Create and Open wallet
                WALLET_HANDLE = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

                //2. Create GVT ClaimDefinition
                let schema = AnoncredsUtils::get_gvt_schema_json(SEQ_NO);
                //TODO Fix it.....Convert String to &'static str
                let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(WALLET_HANDLE, ISSUER_DID, &schema, None, false).unwrap();
                let res = mem::transmute(&claim_def_json as &str);
                mem::forget(claim_def_json);
                CLAIM_DEF_JSON = res;

                //3. Store three claim offers
                let claim_offer_json_1 = AnoncredsUtils::get_claim_offer(ISSUER_DID, SEQ_NO);
                let claim_offer_json_2 = AnoncredsUtils::get_claim_offer(ISSUER_DID, 2);
                let claim_offer_json_3 = AnoncredsUtils::get_claim_offer(DID, 2);

                AnoncredsUtils::prover_store_claim_offer(WALLET_HANDLE, &claim_offer_json_1).unwrap();
                AnoncredsUtils::prover_store_claim_offer(WALLET_HANDLE, &claim_offer_json_2).unwrap();
                AnoncredsUtils::prover_store_claim_offer(WALLET_HANDLE, &claim_offer_json_3).unwrap();

                //4. Create MasterSecret
                AnoncredsUtils::prover_create_master_secret(WALLET_HANDLE, COMMON_MASTER_SECRET).unwrap();

                //5. Create and Store Claim Request
                let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(WALLET_HANDLE,
                                                                                  "HEJ9gvWX64wW7UD",
                                                                                  &claim_offer_json_1,
                                                                                  CLAIM_DEF_JSON,
                                                                                  COMMON_MASTER_SECRET, None).unwrap();
                let claim_json = AnoncredsUtils::get_gvt_claim_json();

                //6. Create Claim
                let (_, xclaim_json) = AnoncredsUtils::issuer_create_claim(WALLET_HANDLE, &claim_req, &claim_json, None).unwrap();

                //7. Store Claim
                AnoncredsUtils::prover_store_claim(WALLET_HANDLE, &xclaim_json).unwrap();
            });

            (WALLET_HANDLE, CLAIM_DEF_JSON)
        }
    }
}