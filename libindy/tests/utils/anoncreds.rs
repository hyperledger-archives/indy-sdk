extern crate serde_json;

use indy::api::ErrorCode;
use indy::api::anoncreds::*;

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::types::{ClaimsForProofRequest, ClaimInfo, Schema, SchemaData, SchemaKey};

use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use utils::constants::*;

use std::collections::HashSet;
use std::iter::FromIterator;

pub struct AnoncredsUtils {}

pub static mut WALLET_HANDLE: i32 = 0;
pub static mut CLAIM_DEF_JSON: &'static str = "";
pub static mut CLAIM_OFFER_JSON: &'static str = "";
pub static mut CLAIM_REQUEST_JSON: &'static str = "";
pub static mut CLAIM_JSON: &'static str = "";
pub const COMMON_MASTER_SECRET: &'static str = "common_master_secret_name";
pub const GVT_SEQ_NO: i32 = 1;
pub const XYZ_SEQ_NO: i32 = 2;

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

    pub fn issuer_create_claim_offer(wallet_handle: i32, schema: &str, issuer_did: &str, prover_did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_def_json| {
            sender.send((err, claim_def_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_offer_cb(cb);

        let schema = CString::new(schema).unwrap();
        let issuer_did = CString::new(issuer_did).unwrap();
        let prover_did = CString::new(prover_did).unwrap();

        let err =
            indy_issuer_create_claim_offer(command_handle,
                                           wallet_handle,
                                           schema.as_ptr(),
                                           issuer_did.as_ptr(),
                                           prover_did.as_ptr(),
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_offer_json) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_offer_json)
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
                                             claim_def_json: &str, master_secret_name: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_req_json| {
            sender.send((err, claim_req_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_claim_req_cb(cb);

        let prover_did = CString::new(prover_did).unwrap();
        let claim_offer_json = CString::new(claim_offer_json).unwrap();
        let claim_def_json = CString::new(claim_def_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();

        let err = indy_prover_create_and_store_claim_req(command_handle,
                                                         wallet_handle,
                                                         prover_did.as_ptr(),
                                                         claim_offer_json.as_ptr(),
                                                         claim_def_json.as_ptr(),
                                                         master_secret_name.as_ptr(),
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

    pub fn issuer_create_claim(wallet_handle: i32, claim_req_json: &str, claim_values_json: &str, user_revoc_index: Option<i32>) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
            sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_cb(cb);

        let claim_req_json = CString::new(claim_req_json).unwrap();
        let claim_values_json = CString::new(claim_values_json).unwrap();

        let err = indy_issuer_create_claim(command_handle,
                                           wallet_handle,
                                           claim_req_json.as_ptr(),
                                           claim_values_json.as_ptr(),
                                           user_revoc_index.unwrap_or(-1),
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, revoc_reg_update_json, claim_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((revoc_reg_update_json, claim_json))
    }

    pub fn prover_store_claim(wallet_handle: i32, claim_json: &str, rev_reg_json: Option<&str>) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_store_claim_cb(cb);

        let claim_json = CString::new(claim_json).unwrap();
        let rev_reg_json_str = rev_reg_json.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = indy_prover_store_claim(command_handle,
                                          wallet_handle,
                                          claim_json.as_ptr(),
                                          if rev_reg_json.is_some() { rev_reg_json_str.as_ptr() } else { null() },
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
                               schemas_json: &str, master_secret_name: &str, claim_defs_json: &str,
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
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = indy_prover_create_proof(command_handle,
                                           wallet_handle,
                                           proof_req_json.as_ptr(),
                                           requested_claims_json.as_ptr(),
                                           schemas_json.as_ptr(),
                                           master_secret_name.as_ptr(),
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

    pub fn indy_issuer_create_and_store_revoc_reg(wallet_handle: i32, issuer_did: &str, schema_json: &str, max_claim_num: u32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_json| {
            sender.send((err, revoc_reg_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_and_store_revoc_reg_cb(cb);

        let issuer_did = CString::new(issuer_did).unwrap();
        let schema_json = CString::new(schema_json).unwrap();

        let err = indy_issuer_create_and_store_revoc_reg(command_handle,
                                                         wallet_handle,
                                                         issuer_did.as_ptr(),
                                                         schema_json.as_ptr(),
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

    pub fn issuer_revoke_claim(wallet_handle: i32, issuer_did: &str, schema_json: &str, user_revoc_index: u32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_update_json| {
            sender.send((err, revoc_reg_update_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_revoke_claim_cb(cb);

        let issuer_did = CString::new(issuer_did).unwrap();
        let schema_json = CString::new(schema_json).unwrap();

        let err = indy_issuer_revoke_claim(command_handle,
                                           wallet_handle,
                                           issuer_did.as_ptr(),
                                           schema_json.as_ptr(),
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

    pub fn get_composite_id(issuer_did: &str, schema_key: &SchemaKey) -> String {
        format!("{}:{}:{}:{}", issuer_did, schema_key.name, schema_key.version, schema_key.did)
    }

    pub fn gvt_schema_key() -> SchemaKey {
        SchemaKey {
            name: "gvt".to_string(),
            version: "1.0".to_string(),
            did: DID_TRUSTEE.to_string()
        }
    }

    pub fn xyz_schema_key() -> SchemaKey {
        SchemaKey {
            name: "xyz".to_string(),
            version: "1.0".to_string(),
            did: ISSUER_DID.to_string()
        }
    }

    pub fn gvt_schema_key_json() -> String {
        serde_json::to_string(&AnoncredsUtils::gvt_schema_key()).unwrap()
    }

    pub fn xyz_schema_key_json() -> String {
        serde_json::to_string(&AnoncredsUtils::xyz_schema_key()).unwrap()
    }

    pub fn gvt_schema() -> Schema {
        Schema {
            seq_no: 1,
            dest: DID_TRUSTEE.to_string(),
            data: SchemaData {
                name: "gvt".to_string(),
                version: "1.0".to_string(),
                attr_names: HashSet::from_iter(vec!["age".to_string(), "sex".to_string(), "height".to_string(), "name".to_string()].iter().cloned())
            }
        }
    }

    pub fn gvt_schema_json() -> String {
        serde_json::to_string(&AnoncredsUtils::gvt_schema()).unwrap()
    }

    pub fn xyz_schema() -> Schema {
        Schema {
            seq_no: 1,
            dest: ISSUER_DID.to_string(),
            data: SchemaData {
                name: "xyz".to_string(),
                version: "1.0".to_string(),
                attr_names: HashSet::from_iter(vec!["status".to_string(), "period".to_string()].iter().cloned())
            }
        }
    }

    pub fn xyz_schema_json() -> String {
        serde_json::to_string(&AnoncredsUtils::xyz_schema()).unwrap()
    }

    pub fn custom_schema(name: &str) -> String {
        serde_json::to_string(&Schema {
            seq_no: 1,
            dest: DID_TRUSTEE.to_string(),
            data: SchemaData {
                name: name.to_string(),
                version: "1.0".to_string(),
                attr_names: HashSet::from_iter(vec!["age".to_string(), "sex".to_string(), "height".to_string(), "name".to_string()].iter().cloned())
            }
        }).unwrap()
    }

    pub fn get_claim_offer(issuer_did: &str, schema_key: &SchemaKey) -> String {
        format!(r#"{{
                    "issuer_did":"{}",
                    "schema_key":{{"name":"{}", "version":"{}", "did":"{}"}},
                    "nonce": "123456789",
                    "key_correctness_proof": {{
                        "c": "40983841062403114696351105468714473190092945361781922980284036284848255102181",
                        "xz_cap": "213464720484089744362618540118202909431724596227070046572799595772146912256777238162172299903278411669692103853805864530643873553457214761106883317761027120427539370942370962393243000202519433219346910021513926418330149853541417254168868250711855156634731074143905917765057402152157911116556639972831731251935718700937677764992720913700241472793061399051130783380568753128719906986229399266611916274676591429895036623790763995576609783339973743504299970516925347817430071017432740932267531399950884977365109931994767334355518355993866340663897276958741231492087880026463329993917787126111107043213858416969200146216919724481899757743335320111464498989394895406342200088098092363803780335427461",
                        "xr_cap": {{
                            "age":"428551817403717345496666225339792093375807052545681523267375890344316772830711607454203456132869878139513849454441680693213618371219989695825485734499740269394536720808195352535836241683894046154422601982139187282874298237872016126349886775459552952772019552508536658271791921689339332976876098011786609958781975486656330572741194023204426484952261795227021910523213710060753808292174119734371160619117183552237714301799404966964550022274275304553920529452381773256570920044318126774874528737572897410668952113510485617038975261423255802900402119311766709164727308287389090038205944899002702399329914812218958604251060872288309671952489910694000990814697006984351987506978581894973799401592211",
                            "height":"411657369769012290217285704283327594441623457932786578784814099069982464122634859546109989278195207865751652851745795318226450710002590664034182094394469565119510105316695655956892227633500179147318075067585551834678079812461536830790528252558459232505987954022333485123452283103020768033499524916346016600527748463901810773406682862302906632677327131603416116045070433377354243916235831969703006718595171020843309342047669615896623593427073236313132690348520294916012881797187163597866196204765064323603066770657621979137899593499157032831120638301825327588467780000638198687916279993936278677557249905181200340769464532921226462449219936857163316761986408035441733035901688059567989300117882",
                            "name":"869322975653258356083915983990526979728408630010817458571291042713509811788475578121058954003892631131467356704604351238331780894204614591041662184716582274656810743747383953498817535302551304877321807454020020152874312640585570851593902460677745364557958108957714916300524302083561141490749493731078047092029485764829854763907822331747337420362381448975375124969403844387156269077805007874513313426920627145892677170274354768722781701010279364942880411045002631531693007422568259696565436694533169879230288912084861052431355880089929921941631003274141039364415665970063262534617675591334457554707139889594182371941548502441982219614399925468254660740292400093419154694726435630358592702798293",
                            "sex":"80391464088175985479491145491149691676821702211894975540979533937774408491785219834122762944971811095537317848654416410580026667952335862665033546961195841179049138780634877378888139872391903804566992942049889566118414459535461354834916790111149556147862372720479995171424595620702416860508557772658191427975040372006893431243929350584258325646184152369207604974849840003307909256680303811690743921237117427932325288396536300357224457903672928805464748280413883820982138162562660615091490216949908906589977916965927522227509078411025411863914347809289131586019476288990589861921562466467956967324009607175203666778312423056471533641756179235960697838324279027572094105302470967687825859737087"
                        }}
                    }}
                   }}"#,
                issuer_did, schema_key.name, schema_key.version, schema_key.did)
    }

    pub fn gvt_claim_offer() -> String {
        AnoncredsUtils::get_claim_offer(ISSUER_DID, &AnoncredsUtils::gvt_schema_key())
    }

    pub fn xyz_claim_offer() -> String {
        AnoncredsUtils::get_claim_offer(ISSUER_DID, &AnoncredsUtils::xyz_schema_key())
    }

    pub fn gvt_claim_values_json() -> &'static str {
        r#"{
               "sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],
               "name":["Alex","1139481716457488690172217916278103335"],
               "height":["175","175"],
               "age":["28","28"]
        }"#
    }

    pub fn xyz_claim_values_json() -> &'static str {
        r#"{
               "status":["partial","51792877103171595686471452153480627530895"],
               "period":["8","8"]
        }"#
    }

    pub fn gvt2_claim_values_json() -> &'static str {
        r#"{
               "sex":["male","2142657394558967239210949258394838228692050081607692519917028371144233115103"],
               "name":["Alexander","21332817548165488690172217217278169335"],
               "height":["170","170"],
               "age":["28","28"]
        }"#
    }

    pub fn gvt_claim_def_data_json() -> &'static str {
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
        }"#
    }

    pub fn claim_def_json() -> &'static str {
        r#"{
            "ref":1,
            "origin":"NcYxiDXkpYi6ov5FcYDi1e",
            "signature_type":"CL",
            "data":{
                "primary":{
                    "n":"83700833261954142840883490294895166161595301731578998022262502712066776442890514325744286884197144798326414368014405751886855622105389299968962016434932215929671719982377213566105569947388216644079909281643650041788187098746961840411450942272990950663881633634695967313771568709791719832415691196006613061872217580015159326668870707429727718052538670621252863240380009721914996037137097764074847327414994867897029862524099326400599230496517275748726576865044646547575948006832245888394496484637725112875227352388732906515206524134299973521640981808602607767797461838791125103163246880592602246349912477596300221925281",
                    "s":"51577844597428165224950247213739713017697817263052505822008472141885375360124708713237015426238558907393646642217464396827882029587872798688393901386020233337851425716622744208800923988402063042331810328754576977594738201111077898698670357826113122576540034863965148704561050678789353363396002680043829322189597404890414046290678587698475252039104265662355991953364086657697478701976860866979237315295373127740932599887679536942342018253925518322194136878729798111869095518966543456247951590663867805357123124475913654374213374751041539590433961525088634170724107188131376949258026545290354317749832311415250990164780",
                    "rms":"62708414794538422026943562355906571554881830752849062463063917641676613902479491229637145851771930128327808147064140503158100571036282116638397969353349228925020048012395800852264983947986479849714387058861061034242192022705759403875689605612734501984741158738056172714426231433129979726629653681375665701705028679069521733305781044343001519335391197811473052518083306713323493336766202332476417248027975403719396975857246682971312037605116774850666238204315047288111325043386411766566477681558576375469258169775053419929433271605886306241119655198512577145876062966065581871314672888861154295655754837094932956805426",
                    "r":{
                        "height":"7494188622493963296371917167403489481700273059782054999455659466386532648382611786914343237242844878773430234638910045295358478625399697391585449023556522219075858680740645546795758181628981868954184260313152164031653595874294283262885339798434731903876494249293850867986870399677814154148567535559651088297572610075535852514290667435536599602759182312599231526717957528420962353399555892560660069225530124896146119913556820301264467039816331287604702401879088610932532894129594204847093247332825201633482082600376522831908067045247351373719763805226525727696839451033356886434970153609023330012153231016667329777696",
                        "age":"43046580493164449821961705026387530718400962423879727006013946580835545832101569331369498984037749519211158406754939208296104507300631668137258362994203612534116672604355742579715019955935409355636621688964776800628565598346203942840267656899349137712767748817368845735656201367242542534635279763131516901403181429708581998366028577775710901657876749334400673065486555707081600694875642698628626665153188555931913999679166028466417167006140881133170951984403242763148060394279316818497553647532981619051273875000348303344274886985296929891179020792044187882266662869725597159101701220942643032293399612230392957570581",
                        "sex":"31391934749268777097046095921329371256192556560798569606151655494000334218671922453509535334425317042318307374504839955690976647333546341369834768688635784140862983291552330278860624226449188642575498831752386208941406613814321749480509109201900035329797459779229058581915450415577005732788045738483099035786100628640371978086263122452921356849544792769452654842833600056471373685447335223378705910906125957737766421419437315127439980793505777939033365211586384773464903151776643617589982755373937461256077657573950063876991303871444299245075401364895496285926085382510741543391601676959655452384503068011979934904299",
                        "name":"64840191664146986014724852820703243030122885784900045259945800604982240780213882839075029527730237693303411568455775358176681800981202303514798201517723103843389755391177416142616408575673840594667007246267969400671516741051469569038254235920709685371937127215998733852043413680284395500100531343570463969226739712267441866700485712180044264216527103402675699198099678041853150035796984466247681379666040861693728820386624059966279843155445343462554727993823292930187774999030025912062750634785781247559879913255618927306902136363693793213719920011348477522844420605936701667553189824313123043674515752876373195871501"
                    },
                    "rctxt":"13920125979496359383664089416368046657681178381336442748179982248090587205285324324319385460031681344719966280342706146885080211497557646733775315068928877946117771740999746266941852936734002809096478340345265332354968435653841555658979717252259856489574519747752076399872768043883082679544989654069519821636373428202935450859526735558087445491143414940123774990508370867355492079422429892097841461957589279524217790035579627150412018826222685692001964707919705792614905631165408310732388384665325591503572546353748867294759755431259001387311984646674572904572661231923735604585456892245402733390935721768635135049503",
                    "z":"50109296960333342288026367833057996290823387533856893347356441132719853478925901265330098686202447936439713166809373460542432372819663794205473392135238719646136491620149885056265034742223048897220959566730659845455539891685421917703834066412587767428625819805714800636503521917315498708118955336538986979915466389840766558674135553950710428562937188174376705150160959711400066104198147552458983394499781679896880103474557745812410257278134246578495915433917231140731774952957708221646162686869495299299488019344103269536547263643347547484711709240083083547828111748533176817401632721994861304680045936924478972441786"
                },
                "revocation":null
            }
        }"#
    }

    pub fn proof_request_attr_and_predicate() -> &'static str {
        r#"{
              "nonce":"123432421212",
              "name":"proof_req_1",
              "version":"0.1",
              "requested_attrs":{
                  "attr1_referent":{
                      "name":"name"
                  }
              },
              "requested_predicates":{
                  "predicate1_referent":{
                      "attr_name":"age","p_type":">=","value":18
                  }
              }
         }"#
    }

    pub fn proof_request_attr() -> &'static str {
        r#"{
              "nonce":"123432421212",
              "name":"proof_req_1",
              "version":"0.1",
              "requested_attrs":{
                  "attr1_referent":{
                      "name":"name"
                  }
              },
              "requested_predicates":{}
         }"#
    }

    pub fn proof_json() -> &'static str {
        r#"{
            "proof":{
                "proofs":{
                    "claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{
                        "primary_proof":{
                            "eq_proof":{"revealed_attrs":{"name":"1139481716457488690172217916278103335"},"a_prime":"80401564260558483983794628158664845806393125691167675024527906210615204776868092566789307767601325086260531777605457298059939671624755239928848057947875953445797869574854365751051663611984607735255307096920094357120779812375573500489773454634756645206823074153240319316758529163584251907107473703779754778699279153037094140428648169418133281187947677937472972061954089873405836249023133445286756991574802740614183730141450546881449500189789970102133738133443822618072337620343825908790734460412932921199267304555521397418007577171242880211812703320270140386219809818196744216958369397014610013338422295772654405475023","e":"31151798717381512709903464053695613005379725796031086912986270617392167764097422442809244590980303622977555221812111085160553241592792901","v":"524407431684833626723631303096063196973911986967748096669183384949467719053669910411426601230736351335262754473490498825342793551112426427823428399937548938048089615644972537564428344526295733169691240937176356626523864731701111189536269488496019586818879697981955044502664124964896796783428945944075084807859935155837238670987272778459356531608865162828109489758902085206073584532002909678902616210042778963974064479140826712481297584040209095459963718975102750913306565864485279810056629704077428898739021040190774575868853629858297299392839284660771662690107106553362040805152261505268111067408422298806905178826507224233050991301274817252924123120887017757639206512015559321675322509820081151404696713509158685022511201565062671933414307463988209696457343022378430051265752251403461414881325357657438328740471164157220698425309006894962942640219890219594168419276308074677144722217081026358892787770650248878952483621","m":{"age":"10477979077744818183854012231360633424177093192344587159214818537659704987539982653663361680650769087122324965941845552897155693994859927792964720675888893623940580527766661802170","sex":"15368219775809326116045200104269422566086585069798988383076685221700842794654771075432385446820819836777771517356551059931242867733879324915651894894695726945279462946826404864068","height":"268172143999991481637372321419290603042446269013750825098514042757459298040087626745653681785038933035820421862976371452111736537699176931068992453946771945552540798204580069806"},"m1":"119095745403940293668103184388411799541118279558928018597628509118163496000813590825371995586347826189221837428823000332905316924389185590810015031744029496470545254805993327676570037596326743185389101389800942263689809725968264069601565478411709555274081560719927118853299543998608664701485475703881376151770","m2":"3166313665375815600922385342096456465402430622944571045536207479553790085339726549928012930073803465171492637049498407367742103524723152099973753540483894420905314750248333232361"},
                            "ge_proofs":[{"u":{"2":"6494171529848192644197417834173236605253723188808961394289041396341136802965710957759175642924978223517091081898946519122412445399638640485278379079647638538597635045303985779767","0":"7739508859260491061487569748588091139318989278758566530899756574128579312557203413565436003310787878172471425996601979342157451689172171025305431595131816910273398879776841751855","3":"9424758820140378077609053635383940574362083113571024891496206162696034958494400871955445981458978146571146602763357500412840538526390475379772903513687358736287298159312524034159","1":"9011979414559555265454106061917684716953356440811838475257096756618761731111646531136628099710567973381801256908067529269805992222342928842825929421929485785888403149296320711642"},"r":{"DELTA":"2119857977629302693157808821351328058251440215802746362450951329352726877165815663955490999790457576333458830301801261754696823614762123890412904169206391143688952648566814660498520188221060505840151491403269696751525874990487604723445355651918681212361562384420233903265612599812725766212744963540390806334870022328290970137051148373040320927100063898502086531019924715927190306801273252711777648467224661735618842887006436195147540705753550974655689586750013569294343535843195025962867299786380033532422131203367401906988124836294104501525520053613392691214421562815044433237816093079784307397782961917892254668290115653012265908717124278607660504580036193346698672079435538219972121355893074219968755049500875222141","2":"879097501989202140886939888802566536179834329508897124489020677433754766947767937608431979796722207676629625451150104784909666168153917345813160237337412296010679353735699663083287427507870244565918756969618964144516025526404618052053542009438548457492400344119561349471929199757453154204191407620539220514897529346602664135146454509169680801061111878075145734123580343470361019624175036825631373890661124315134340427076598351080893567995392248394683875116715114577054906406649006122102488431184007790011073389768061904597267545895265921673106871142463561948479668876241841045522543174660428236658891636170119227855493059358614089146415798861053408542832475696099851160385105386001523305465829676723036394820593263477","0":"1724016272047416140958096373304304971004826284109046259544344355102178044512441391364907122486655755929044720001281832600729467778103556397960700809066582436321515744527550472324028227472294258045699756170293405547851344921626775854114063087070898499913846456795761213291925373770081490280103876827479351849800210782799381740073719081199000612284788683993320623339686128531187019125095700122135094060470612862911102824801065698176788174959069186600426519872015152034176356923049531650418553748519941342115963599848111324793380438600664408464987023646615003553912544410140730587797458882329021327455905737414352355326238028222782957735440607899424838572541602600159016542488644761584240884783618700311735467659132540546","3":"2317535203964314926167241523636020444600002667629517624482931328850422196008281300859516069440995466415138723103558631951648519232327284208990029010060986032518946759289078833125920310350676484457972303378558158127406345804560689086460633931717939234025886786468170219981598030245042011840614339386724945679531091642132820284896626191109974537171662283750959028046143650291367908660204201563611944187723824430780626387525165408619587771059635528553832034409311888615502905143628507219523591091412192645348525327725381323865648645828460581593542176351568614465903523790649219812666979685223535464526901006270478687017672202058914176692964406859722580270696925877498058525086810338471380117323227744481903228027847825795","1":"1119193929864813751243160041764170298897380522230946444206167281178657213260394833843687899872857393015947283159245092452814155776571829885921814072299525859857844030379558685168895306445277750249341844789101670896570226707650318347992386244538723699686941887792682779028216548922683313576597384354842537728667739985216662699631842296096507821667149950956179957306177525178260912379909156360834120816956949271530622510333943914411903103069247646327625753995178999023427645468623522280255892736633780185163496867644317005801241786702434621502492159672660131289312665511793827552317714835658019088880972220344126692027952749318018900669839090109361161616086319604439015851316798257015063653414161203599184730094765941653"},"mj":"10477979077744818183854012231360633424177093192344587159214818537659704987539982653663361680650769087122324965941845552897155693994859927792964720675888893623940580527766661802170","alpha":"46280660038407959140964701167450659223532556136388451390393713283900546119670373626221864441898929302821705811144923685080534692512705456699843367809872982836890616398604933641265111106644805368974824737276965928297120628041257593166650593538539384316563258781595629888673792430276007730792093088812056156937735120078929629310611907731935101448992312370312134173482115524436767558802102266208152808607480693236511858269018733175523724309089010048330044458187371675333889670055578652283806685440133357512406700879353713629795062705271430695988191782837658895477702634883214188598350625843489120361660836956958750828038278027538830855628653513539929730230905015331221220017847248793929813230252015802389329428995718799619565984669228143200627972926117282688854152516298117476837960100343260648687249027349308513966440386556698667484082658689","t":{"DELTA":"46814992964714978733007076702016837564951956529003697497847838781899848384824991374342901164708655443686022921583406187082133141084994843502230809550055933825660668160300304112671478218513259983054489597176651737200716259733573469298437873515151377206364940530308167934399245072298875358347931404742292788785586833114480704138718996633638362933821933388459210678374952072108333767698704767907612549860590824123780096225591372365712106060039646448181221691765233478768574198237963457485496438076793333937013217675591500849193742006533651525421426481898699626618796271544860105422331629265388419155909716261466161258430","2":"59423006413504086085782234600502410213751379553855471973440165009200961757474676407242673622935614782362911290590560535490636029324125251850583605745046201217673654522625983661578962623803698461459190578519097656221453474955879823750445359506290522280566225253310030053812918275525607874059407284653434046369835156477189219911810464401689041140506062300317020407969423270374033482533711564673658146930272487464489365713112043565257807490520178903336328210031106311280471651300486164966423437275272281777742004535722142265580037959473078313965482591454009972765788975683031385823798895914265841131145707278751512534120","0":"56510878078818710798555570103060159621941668074271797077206591818472978018558098567975838757566260370093327989369045722406190165972775356924844244889146946158949660988214890388299203816110339909687790860564719380865809705044646711632599987968183128514431910561478715003212633874423067294596323864121737000450543142072142652163818450299889830999149821558252183477517484127000480272695698860647674027831262149565273068850774090998356019534296579838685977022988536930596918054160990243868372150609770079720240227817149126735182138479851227052696211125454858584118346950878092387488482897777914362341820607560926173967363","3":"63511079416489489495396586813126304469185174450150717746314545118902972011091412254834718868134635251731510764117528579641756327883640004345178347120290107941107152421856942264968771810665927914509411385404403747487862696526824127219640807008235054362138760656969613951620938020257273816713908815343872804442748694361381399025862438391456307852482826748664499083370705834755863016895566228300904018909174673301643617543662527772400085378252706897979609427451977654028887889811453690146157824251379525221390697200211891556653698308665831075787991412401737090471273439878635073797691350863566834141222438011402987450926","1":"30348838247529448929141877305241172943867610065951047292188826263950046630912426030349276970628525991007036685038199133783991618544554063310358191845473212966131475853690378885426974792306638181168558731807811629973716711132134244797541560013139884391800841941607502149630914097258613821336239993125960064136287579351403225717114920758719152701696123905042695943045383536065833292374624566478931465135875411483860059753175449604448434619593495399051968638830805689355610877075130302742512428461286121237297212174164897833936610857614962734658136750299346971377383141235020438750748045568800723867413392427848651081274"},"predicate":{"attr_name":"age","p_type":"GE","value":18}}]
                        },
                        "non_revoc_proof":null
                    }
                },
                "aggregated_proof":{
                    "c_hash":"81135772044295974649282368084258333955993271555081206390568996949836231116301",
                    "c_list":[[2,124,231,47,189,36,247,160,61,220,165,35,97,165,203,185,133,253,81,239,67,127,156,49,189,16,140,30,177,161,221,54,154,0,127,143,98,212,114,193,188,85,206,171,198,140,9,192,10,254,218,120,201,182,40,141,80,35,81,148,204,192,41,5,186,33,50,77,211,163,124,130,32,219,193,167,79,43,181,76,19,249,53,79,70,221,205,36,180,50,120,255,161,227,196,204,71,106,221,131,220,7,73,86,128,208,48,58,123,63,82,24,170,141,143,56,221,96,151,108,105,38,185,243,224,112,177,101,195,87,208,201,39,123,165,125,92,104,234,188,54,92,31,158,178,152,52,205,26,156,237,241,23,15,76,220,168,32,175,230,157,197,225,70,57,237,8,81,13,17,95,70,143,56,162,223,203,8,48,153,51,51,118,116,32,139,187,222,146,86,165,111,125,107,203,18,212,28,168,22,62,69,204,207,122,148,25,30,92,120,83,214,116,221,204,120,230,70,128,139,181,110,69,93,253,240,69,16,113,224,246,41,142,0,83,237,186,4,50,156,206,199,89,74,96,168,249,240,101,16,103,234,162,219,52,218,207],[1,191,167,2,151,36,61,136,184,172,120,86,127,88,109,119,56,21,167,171,217,221,24,64,246,237,255,152,81,183,201,191,59,234,213,101,254,91,33,205,120,71,215,144,160,243,145,109,19,151,241,46,135,132,50,143,219,207,197,35,89,103,83,212,96,83,222,101,55,57,220,161,252,115,39,62,46,160,30,138,221,89,125,66,114,150,5,95,63,10,55,107,102,73,40,69,41,6,57,0,64,226,152,66,181,149,251,50,28,53,18,26,221,5,188,67,125,184,190,200,56,92,132,201,242,211,37,2,43,6,146,88,228,120,204,190,4,118,134,106,118,110,249,145,175,165,116,197,200,183,207,215,197,79,207,203,29,182,231,151,248,233,107,41,79,234,250,27,33,33,107,102,240,47,37,230,243,185,93,192,52,31,73,211,11,173,150,92,194,154,172,247,221,206,129,85,193,105,172,140,201,40,240,200,28,94,1,96,204,175,113,170,46,134,229,111,215,208,237,252,84,50,249,41,214,79,38,194,23,212,7,164,153,217,23,252,32,114,145,58,189,118,104,131,84,184,115,175,199,227,219,117,23,113,113,180,3],[240,104,187,71,84,144,129,123,12,181,215,233,27,55,56,54,94,57,17,42,111,42,112,234,192,23,226,103,118,198,189,175,175,1,102,64,128,100,221,201,134,106,83,239,69,43,150,172,95,206,145,224,207,239,39,193,30,200,90,125,175,125,59,47,250,224,193,21,64,112,101,131,128,249,96,165,73,33,174,64,69,252,209,158,130,53,23,158,217,173,69,51,12,145,70,174,15,206,13,181,50,246,50,110,223,65,250,44,39,33,8,47,169,242,147,3,190,164,110,20,68,5,142,133,38,198,151,161,167,0,219,128,126,120,190,23,153,22,250,78,114,241,252,181,74,142,65,123,225,153,75,159,78,84,28,110,203,105,231,238,75,138,121,233,75,163,221,69,106,143,1,217,251,43,147,252,189,122,19,124,189,180,206,91,165,199,41,172,233,102,14,91,162,254,16,142,60,230,39,200,208,236,101,69,101,152,233,217,100,206,31,120,211,191,90,56,205,40,180,120,47,210,224,86,153,34,86,237,204,11,183,227,0,224,15,201,32,228,4,210,43,156,68,246,137,150,103,197,191,150,155,181,78,5,134,58],[1,214,184,139,205,251,132,131,8,186,140,58,211,242,134,120,121,253,128,192,10,252,172,101,44,26,119,56,212,8,248,71,19,96,59,12,233,191,63,187,217,35,191,160,127,247,189,247,229,111,252,101,126,10,142,252,238,215,211,137,137,164,114,186,255,199,183,50,103,9,158,63,134,140,162,154,188,109,52,31,92,78,38,228,0,60,225,100,239,88,114,95,48,71,7,117,168,45,45,177,178,62,87,197,98,174,123,249,26,237,179,12,63,182,46,218,183,148,163,222,179,159,146,56,142,190,122,100,211,6,86,237,10,7,111,186,27,66,95,252,108,247,203,1,111,60,13,218,104,63,128,125,197,11,201,138,33,122,37,31,163,123,120,132,65,122,208,60,80,87,113,183,28,31,74,106,18,79,52,245,113,184,94,202,72,223,8,128,209,43,77,237,119,208,255,144,26,76,223,77,177,131,237,49,150,251,53,150,115,33,254,237,185,15,140,234,205,99,248,252,171,245,192,104,151,194,190,186,249,180,246,9,169,165,0,221,7,107,39,67,58,178,176,99,212,40,247,49,127,7,94,5,170,65,154,28,104],[1,247,26,202,244,120,131,95,151,52,56,38,141,232,178,50,61,45,235,61,12,68,11,180,174,222,110,211,141,253,198,204,248,192,40,99,237,1,45,170,79,208,3,13,135,89,195,65,3,228,224,146,181,198,14,79,78,237,168,81,108,151,68,12,88,242,120,200,120,193,253,51,167,140,43,175,59,18,160,190,233,21,213,135,162,76,38,48,163,110,155,197,97,93,211,183,95,42,172,249,98,59,161,136,70,39,142,48,242,44,154,103,186,161,214,215,0,254,166,150,111,71,242,102,209,125,25,65,144,223,211,137,223,239,50,96,185,171,120,155,171,98,204,23,102,253,68,141,91,240,127,170,199,249,217,165,164,37,174,212,159,232,140,196,216,140,205,102,84,104,220,223,9,249,75,245,78,157,245,203,235,154,73,34,77,12,227,138,93,105,178,114,255,210,88,216,202,64,69,128,220,211,113,51,15,185,103,236,52,187,49,29,162,20,35,21,65,188,33,46,11,172,59,15,221,36,33,213,14,121,36,218,76,80,97,197,83,64,145,73,194,43,233,144,251,86,112,209,230,67,234,116,172,219,123,50,46],[1,114,216,159,37,214,198,117,230,153,15,176,95,20,29,134,179,207,209,35,101,193,47,54,130,141,78,213,54,167,31,73,105,177,129,135,6,135,45,107,103,16,133,187,74,217,42,40,1,214,60,70,78,245,86,82,150,75,91,235,181,249,129,147,202,15,86,250,222,240,203,236,102,39,53,147,79,178,124,184,97,73,65,136,74,29,219,182,83,167,221,203,32,200,243,130,65,234,133,181,203,35,86,21,123,170,74,174,5,132,1,149,77,141,158,193,249,130,37,53,253,234,228,144,66,152,232,246,26,193,6,53,139,45,231,173,115,87,89,61,197,9,96,73,229,189,49,44,203,214,156,139,58,153,77,13,90,35,157,130,184,150,161,69,145,157,4,206,52,216,227,233,113,202,54,154,153,100,83,97,135,88,197,227,42,52,28,221,91,117,56,183,198,102,231,37,232,226,136,142,115,218,175,45,221,143,130,215,184,39,102,172,126,253,152,108,254,241,17,98,70,223,191,138,251,227,243,32,180,190,223,69,135,0,97,105,115,189,221,134,26,159,32,210,172,233,7,65,238,77,203,159,181,188,203,159,190]]
                }
            },
            "requested_proof":{
                "revealed_attrs":{"attr1_referent":["claim::58479554-187f-40d9-b0a5-a95cfb0338c3","Alex","1139481716457488690172217916278103335"]},
                "unrevealed_attrs":{},
                "self_attested_attrs":{},
                "predicates":{"predicate1_referent":"claim::58479554-187f-40d9-b0a5-a95cfb0338c3"}
            },
            "identifiers":{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}}}
        }"#
    }

    pub fn get_unique_claims(proof_claims: &ClaimsForProofRequest) -> Vec<ClaimInfo> {
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

    pub fn get_claim_for_attr_referent(claims_json: &str, referent: &str) -> ClaimInfo {
        let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
        let claims_for_referent = claims.attrs.get(referent).unwrap();
        claims_for_referent[0].clone()
    }

    pub fn get_claim_for_predicate_referent(claims_json: &str, referent: &str) -> ClaimInfo {
        let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
        let claims_for_referent = claims.predicates.get(referent).unwrap();
        claims_for_referent[0].clone()
    }

    pub fn init_common_wallet() -> (i32, &'static str, &'static str, &'static str, &'static str) {
        lazy_static! {
            static ref COMMON_WALLET_INIT: Once = ONCE_INIT;

        }

        unsafe {
            COMMON_WALLET_INIT.call_once(|| {
                //TODO Need clean after tests but not exists After function in Cargo
                TestUtils::cleanup_storage();

                //1. Create and Open wallet
                WALLET_HANDLE = WalletUtils::create_and_open_wallet("pool1", None).unwrap();

                //2. Issuer1 Create GVT ClaimDefinition
                //TODO Fix it.....Convert String to &'static str
                let issuer1_gvt_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(WALLET_HANDLE,
                                                                                                ISSUER_DID,
                                                                                                &AnoncredsUtils::gvt_schema_json(), None, false).unwrap();

                //3. Issuer1 Create XYZ ClaimDefinition
                let issuer1_xyz_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(WALLET_HANDLE,
                                                                                                ISSUER_DID,
                                                                                                &AnoncredsUtils::xyz_schema_json(), None, false).unwrap();

                //4. Issuer2 Create GVT ClaimDefinition
                let issuer2_gvt_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(WALLET_HANDLE,
                                                                                                DID,
                                                                                                &AnoncredsUtils::gvt_schema_json(), None, false).unwrap();

                //5. Issuer1 Create GVT ClaimOffer
                let issuer1_gvt_claim_offer = AnoncredsUtils::issuer_create_claim_offer(WALLET_HANDLE,
                                                                                        &AnoncredsUtils::gvt_schema_json(),
                                                                                        ISSUER_DID, DID_MY1).unwrap();

                //6. Prover store Issuer1 GVT ClaimOffer
                AnoncredsUtils::prover_store_claim_offer(WALLET_HANDLE, &issuer1_gvt_claim_offer).unwrap();

                //7. Issuer1 Create XYZ ClaimOffer
                let issuer1_xyz_claim_offer = AnoncredsUtils::issuer_create_claim_offer(WALLET_HANDLE,
                                                                                        &AnoncredsUtils::xyz_schema_json(),
                                                                                        ISSUER_DID, DID_MY1).unwrap();

                //8. Prover store Issuer1 XYZ ClaimOffer
                AnoncredsUtils::prover_store_claim_offer(WALLET_HANDLE, &issuer1_xyz_claim_offer).unwrap();

                //9. Issuer2 Create GVT ClaimOffer
                let issuer2_gvt_claim_offer = AnoncredsUtils::issuer_create_claim_offer(WALLET_HANDLE,
                                                                                        &AnoncredsUtils::gvt_schema_json(),
                                                                                        DID, DID_MY1).unwrap();

                //10. Prover store Issuer2 GVT ClaimOffer
                AnoncredsUtils::prover_store_claim_offer(WALLET_HANDLE, &issuer2_gvt_claim_offer).unwrap();

                //11. Create MasterSecret
                AnoncredsUtils::prover_create_master_secret(WALLET_HANDLE, COMMON_MASTER_SECRET).unwrap();

                // Issuer1 issue GVT Claim
                //12. Create and Store Claim Request
                let issuer1_gvt_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(WALLET_HANDLE,
                                                                                              DID_MY1,
                                                                                              &issuer1_gvt_claim_offer,
                                                                                              &issuer1_gvt_claim_def_json,
                                                                                              COMMON_MASTER_SECRET).unwrap();
                let claim_values_json = AnoncredsUtils::gvt_claim_values_json();

                //13. Issuer1 creates GVT Claim
                let (_, claim_json) = AnoncredsUtils::issuer_create_claim(WALLET_HANDLE, &issuer1_gvt_claim_req, &claim_values_json, None).unwrap();

                //14. Store Claim
                AnoncredsUtils::prover_store_claim(WALLET_HANDLE, &claim_json, None).unwrap();

                // Issuer1 issue XYZ Claim
                //15. Create and Store Claim Request
                let issuer1_xyz_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(WALLET_HANDLE,
                                                                                  DID_MY1,
                                                                                  &issuer1_xyz_claim_offer,
                                                                                  &issuer1_xyz_claim_def_json,
                                                                                  COMMON_MASTER_SECRET).unwrap();
                let claim_values_json = AnoncredsUtils::xyz_claim_values_json();

                //16. Create XYZ Claim
                let (_, claim_2_json) = AnoncredsUtils::issuer_create_claim(WALLET_HANDLE, &issuer1_xyz_claim_req, &claim_values_json, None).unwrap();

                //17. Store Claim
                AnoncredsUtils::prover_store_claim(WALLET_HANDLE, &claim_2_json, None).unwrap();

                // Issuer2 issue GVT Claim
                //18. Create and Store Claim Request
                let issuer2_gvt_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(WALLET_HANDLE,
                                                                                  DID_MY1,
                                                                                  &issuer2_gvt_claim_offer,
                                                                                  &issuer2_gvt_claim_def_json,
                                                                                  COMMON_MASTER_SECRET).unwrap();
                let claim_values_json = AnoncredsUtils::gvt2_claim_values_json();

                //19. Create XYZ Claim
                let (_, claim_3_json) = AnoncredsUtils::issuer_create_claim(WALLET_HANDLE, &issuer2_gvt_claim_req, &claim_values_json, None).unwrap();

                //20. Store Claim
                AnoncredsUtils::prover_store_claim(WALLET_HANDLE, &claim_3_json, None).unwrap();

                let res = mem::transmute(&issuer1_gvt_claim_def_json as &str);
                mem::forget(issuer1_gvt_claim_def_json);
                CLAIM_DEF_JSON = res;

                let res = mem::transmute(&issuer1_gvt_claim_offer as &str);
                mem::forget(issuer1_gvt_claim_offer);
                CLAIM_OFFER_JSON = res;

                let res = mem::transmute(&issuer1_gvt_claim_req as &str);
                mem::forget(issuer1_gvt_claim_req);
                CLAIM_REQUEST_JSON = res;

                let res = mem::transmute(&claim_json as &str);
                mem::forget(claim_json);
                CLAIM_JSON = res;
            });

            (WALLET_HANDLE, CLAIM_DEF_JSON, CLAIM_OFFER_JSON, CLAIM_REQUEST_JSON, CLAIM_JSON)
        }
    }
}