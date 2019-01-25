extern crate futures;

use indy::IndyError;
use indy::anoncreds;
use self::futures::Future;

use serde_json;

use utils::{environment, wallet, blob_storage, test, pool};
use utils::types::CredentialOfferInfo;

use std::sync::{Once, ONCE_INIT};
use std::mem;
use utils::constants::*;

use std::collections::{HashSet, HashMap};

use utils::domain::anoncreds::schema::{Schema, SchemaV1};
use utils::domain::anoncreds::credential_definition::{CredentialDefinition, CredentialDefinitionConfig};
use utils::domain::anoncreds::revocation_registry_definition::RevocationRegistryConfig;
use utils::domain::anoncreds::credential::{AttributeValues, CredentialInfo};
use utils::domain::anoncreds::credential_for_proof_request::CredentialsForProofRequest;

pub static mut WALLET_HANDLE: i32 = 0;
pub static mut CREDENTIAL_DEF_JSON: &'static str = "";
pub static mut CREDENTIAL_OFFER_JSON: &'static str = "";
pub static mut CREDENTIAL_REQUEST_JSON: &'static str = "";
pub static mut CREDENTIAL_JSON: &'static str = "";
pub const ANONCREDS_WALLET_CONFIG: &'static str = r#"{"id": "anoncreds_wallet"}"#;
pub const COMMON_MASTER_SECRET: &'static str = "common_master_secret_name";
pub const CREDENTIAL1_ID: &'static str = "credential1_id";
pub const CREDENTIAL2_ID: &'static str = "credential2_id";
pub const CREDENTIAL3_ID: &'static str = "credential3_id";
pub const DELIMITER: &'static str = ":";
pub const CRED_DEF_MARKER: &'static str = "3";

macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub fn issuer_create_schema(issuer_did: &str, name: &str, version: &str, attr_names: &str) -> Result<(String, String), IndyError> {
    anoncreds::issuer_create_schema(issuer_did, name, version, attr_names).wait()
}

pub fn issuer_create_credential_definition(wallet_handle: i32, issuer_did: &str, schema: &str, tag: &str,
                                           signature_type: Option<&str>, config: Option<&str>) -> Result<(String, String), IndyError> {
    anoncreds::issuer_create_and_store_credential_def(wallet_handle, issuer_did, schema, tag, signature_type, config.unwrap_or("{}")).wait() // TODO: FIXME OPTIONAL CONFIG
}

pub fn issuer_create_and_store_revoc_reg(wallet_handle: i32, issuer_did: &str, type_: Option<&str>, tag: &str,
                                         cred_def_id: &str, config_json: &str, tails_writer_handle: i32)
                                         -> Result<(String, String, String), IndyError> {
    anoncreds::issuer_create_and_store_revoc_reg(wallet_handle, issuer_did, type_, tag, cred_def_id, config_json, tails_writer_handle).wait()
}

pub fn issuer_create_credential_offer(wallet_handle: i32, cred_def_id: &str) -> Result<String, IndyError> {
    anoncreds::issuer_create_credential_offer(wallet_handle, cred_def_id).wait()
}

pub fn issuer_create_credential(wallet_handle: i32, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str,
                                rev_reg_id: Option<&str>, blob_storage_reader_handle: Option<i32>) -> Result<(String, Option<String>, Option<String>), IndyError> {
    anoncreds::issuer_create_credential(wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle.unwrap_or(-1)).wait() // TODO OPTIONAL blob_storage_reader_handle
}

pub fn issuer_revoke_credential(wallet_handle: i32, blob_storage_reader_handle: i32, rev_reg_id: &str, cred_revoc_id: &str) -> Result<String, IndyError> {
    anoncreds::issuer_revoke_credential(wallet_handle, blob_storage_reader_handle, rev_reg_id, cred_revoc_id).wait()
}

pub fn issuer_merge_revocation_registry_deltas(rev_reg_delta: &str, other_rev_reg_delta: &str) -> Result<String, IndyError> {
    anoncreds::issuer_merge_revocation_registry_deltas(rev_reg_delta, other_rev_reg_delta).wait()
}

pub fn prover_create_master_secret(wallet_handle: i32, master_secret_id: &str) -> Result<String, IndyError> {
    anoncreds::prover_create_master_secret(wallet_handle, Some(master_secret_id)).wait()
}

pub fn prover_create_credential_req(wallet_handle: i32, prover_did: &str, cred_offer_json: &str,
                                    cred_def_json: &str, master_secret_id: &str) -> Result<(String, String), IndyError> {
    anoncreds::prover_create_credential_req(wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id).wait()
}

pub fn prover_store_credential(wallet_handle: i32, cred_id: &str, cred_req_metadata_json: &str, cred_json: &str,
                               cred_def_json: &str, rev_reg_def_json: Option<&str>) -> Result<String, IndyError> {
    anoncreds::prover_store_credential(wallet_handle, Some(cred_id), cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json).wait()
}

//TODO mark as depricated and use only in target tests
pub fn prover_get_credentials(wallet_handle: i32, filter_json: &str) -> Result<String, IndyError> {
    anoncreds::prover_get_credentials(wallet_handle, Some(filter_json)).wait()
}

pub fn prover_get_credential(wallet_handle: i32, cred_id: &str) -> Result<String, IndyError> {
    anoncreds::prover_get_credential(wallet_handle, cred_id).wait()
}

pub fn prover_search_credentials(wallet_handle: i32, filter_json: &str) -> Result<(i32, usize), IndyError> {
    anoncreds::prover_search_credentials(wallet_handle, Some(filter_json)).wait()
}

pub fn prover_fetch_credentials(search_handle: i32, count: usize) -> Result<String, IndyError> {
    anoncreds::prover_fetch_credentials(search_handle, count).wait()
}

pub fn prover_close_credentials_search(search_handle: i32) -> Result<(), IndyError> {
    anoncreds::prover_close_credentials_search(search_handle).wait()
}

//TODO mark as depricated and use only in target tests
pub fn prover_get_credentials_for_proof_req(wallet_handle: i32, proof_request_json: &str) -> Result<String, IndyError> {
    anoncreds::prover_get_credentials_for_proof_req(wallet_handle, proof_request_json).wait()
}

pub fn prover_search_credentials_for_proof_req(wallet_handle: i32, proof_request_json: &str, extra_query_json: Option<&str>) -> Result<i32, IndyError> {
    anoncreds::prover_search_credentials_for_proof_req(wallet_handle, proof_request_json, extra_query_json).wait()
}

pub fn prover_fetch_next_credentials_for_proof_req(search_handle: i32, item_ref: &str, count: usize) -> Result<String, IndyError> {
    anoncreds::prover_fetch_credentials_for_proof_req(search_handle, item_ref, count).wait()
}

pub fn prover_close_credentials_search_for_proof_req(search_handle: i32) -> Result<(), IndyError> {
    anoncreds::prover_close_credentials_search_for_proof_req(search_handle).wait()
}

pub fn prover_create_proof(wallet_handle: i32, proof_req_json: &str, requested_credentials_json: &str,
                           master_secret_name: &str, schemas_json: &str, cred_defs_json: &str,
                           rev_states_json: &str) -> Result<String, IndyError> {
    anoncreds::prover_create_proof(wallet_handle, proof_req_json, requested_credentials_json,
                         master_secret_name, schemas_json, cred_defs_json, rev_states_json).wait()
}

pub fn verifier_verify_proof(proof_request_json: &str, proof_json: &str, schemas_json: &str,
                             cred_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str) -> Result<bool, IndyError> {
    anoncreds::verifier_verify_proof(proof_request_json, proof_json, schemas_json, cred_defs_json, rev_reg_defs_json, rev_regs_json).wait()
}

pub fn create_revocation_state(blob_storage_reader_handle: i32, rev_reg_def_json: &str,
                               rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Result<String, IndyError> {
    anoncreds::create_revocation_state(blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id).wait()
}

pub fn update_revocation_state(tails_reader_handle: i32, rev_state_json: &str, rev_reg_def_json: &str,
                               rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Result<String, IndyError> {
    anoncreds::update_revocation_state(tails_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id).wait()
}

pub fn default_cred_def_config() -> String {
    serde_json::to_string(&CredentialDefinitionConfig { support_revocation: false }).unwrap()
}

pub fn revocation_cred_def_config() -> String {
    serde_json::to_string(&CredentialDefinitionConfig { support_revocation: true }).unwrap()
}

pub fn issuance_on_demand_rev_reg_config() -> String {
    serde_json::to_string(&RevocationRegistryConfig { max_cred_num: Some(5), issuance_type: None }).unwrap()
}

pub fn issuance_by_default_rev_reg_config() -> String {
    serde_json::to_string(&RevocationRegistryConfig { max_cred_num: Some(5), issuance_type: Some("ISSUANCE_BY_DEFAULT".to_string()) }).unwrap()
}

pub fn gvt_schema_id() -> String {
    Schema::schema_id(ISSUER_DID, GVT_SCHEMA_NAME, SCHEMA_VERSION)
}

pub fn gvt_schema() -> SchemaV1 {
    SchemaV1 {
        id: gvt_schema_id().to_string(),
        version: SCHEMA_VERSION.to_string(),
        name: GVT_SCHEMA_NAME.to_string(),
        attr_names: serde_json::from_str::<HashSet<String>>(GVT_SCHEMA_ATTRIBUTES).unwrap(),
        seq_no: None
    }
}

pub fn gvt_schema_json() -> String {
    serde_json::to_string(&Schema::SchemaV1(gvt_schema())).unwrap()
}

pub fn xyz_schema_id() -> String {
    Schema::schema_id(ISSUER_DID, XYZ_SCHEMA_NAME, SCHEMA_VERSION)
}

pub fn xyz_schema() -> SchemaV1 {
    SchemaV1 {
        id: xyz_schema_id().to_string(),
        version: SCHEMA_VERSION.to_string(),
        name: XYZ_SCHEMA_NAME.to_string(),
        attr_names: serde_json::from_str::<HashSet<String>>(XYZ_SCHEMA_ATTRIBUTES).unwrap(),
        seq_no: None
    }
}

pub fn xyz_schema_json() -> String {
    serde_json::to_string(&Schema::SchemaV1(xyz_schema())).unwrap()
}

pub fn cred_def_id(did: &str, schema_id: &str, signature_type: &str, tag: &str) -> String {
    format!("{}{}{}{}{}{}{}{}{}", did, DELIMITER, CRED_DEF_MARKER, DELIMITER, signature_type, DELIMITER, schema_id, DELIMITER, tag)
}

pub fn issuer_1_gvt_cred_def_id() -> String {
    cred_def_id(ISSUER_DID, &gvt_schema_id(), SIGNATURE_TYPE, TAG_1)
}

pub fn issuer_2_gvt_cred_def_id() -> String {
    cred_def_id(ISSUER_DID_2, &gvt_schema_id(), SIGNATURE_TYPE, TAG_1)
}

pub fn issuer_1_xyz_cred_def_id() -> String {
    cred_def_id(ISSUER_DID, &xyz_schema_id(), SIGNATURE_TYPE, TAG_1)
}

pub fn issuer_1_gvt_cred_offer_info() -> CredentialOfferInfo {
    CredentialOfferInfo { cred_def_id: issuer_1_gvt_cred_def_id() }
}

pub fn issuer_1_xyz_cred_offer_info() -> CredentialOfferInfo {
    CredentialOfferInfo { cred_def_id: issuer_1_xyz_cred_def_id() }
}

pub fn issuer_2_gvt_cred_offer_info() -> CredentialOfferInfo {
    CredentialOfferInfo { cred_def_id: issuer_2_gvt_cred_def_id() }
}

// note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
pub fn gvt_credential_values() -> HashMap<String, AttributeValues> {
    map! {
            "sex".to_string() => AttributeValues {raw: "male".to_string(), encoded: "5944657099558967239210949258394887428692050081607692519917050011144233115103".to_string()},
            "name".to_string() => AttributeValues {raw: "Alex".to_string(), encoded: "1139481716457488690172217916278103335".to_string()},
            "height".to_string() => AttributeValues {raw: "175".to_string(), encoded: "175".to_string()},
            "age".to_string() => AttributeValues {raw: "28".to_string(), encoded: "28".to_string()}
          }
}

pub fn gvt_credential_values_json() -> String {
    serde_json::to_string(&gvt_credential_values()).unwrap()
}

pub fn xyz_credential_values() -> HashMap<String, AttributeValues> {
    map! {
            "status".to_string() => AttributeValues {raw: "partial".to_string(), encoded: "51792877103171595686471452153480627530895".to_string()},
            "period".to_string() => AttributeValues {raw: "8".to_string(), encoded: "8".to_string()}
          }
}

pub fn xyz_credential_values_json() -> String {
    serde_json::to_string(&xyz_credential_values()).unwrap()
}

pub fn gvt2_credential_values() -> HashMap<String, AttributeValues> {
    map! {
            "sex".to_string() => AttributeValues {raw: "male".to_string(), encoded: "2142657394558967239210949258394838228692050081607692519917028371144233115103".to_string()},
            "name".to_string() => AttributeValues {raw: "Alexander".to_string(), encoded: "21332817548165488690172217217278169335".to_string()},
            "height".to_string() => AttributeValues {raw: "170".to_string(), encoded: "170".to_string()},
            "Age".to_string() => AttributeValues {raw: "28".to_string(), encoded: "28".to_string()}
          }
}

pub fn gvt2_credential_values_json() -> String {
    serde_json::to_string(&gvt2_credential_values()).unwrap()
}

pub fn gvt3_credential_values() -> HashMap<String, AttributeValues> {
    map! {
            "sex".to_string() => AttributeValues {raw: "male".to_string(), encoded: "1234567890442222223345678958394838228692050081607692519917028371144233115103".to_string()},
            "name".to_string() => AttributeValues {raw: "Artem".to_string(), encoded: "12356325715837025980172217217278169335".to_string()},
            "height".to_string() => AttributeValues {raw: "180".to_string(), encoded: "180".to_string()},
            "age".to_string() => AttributeValues {raw: "25".to_string(), encoded: "25".to_string()}
          }
}

pub fn gvt3_credential_values_json() -> String {
    serde_json::to_string(&gvt3_credential_values()).unwrap()
}

pub fn issuer_1_gvt_credential() -> CredentialInfo {
    CredentialInfo {
        schema_id: gvt_schema_id(),
        cred_def_id: issuer_1_gvt_cred_def_id(),
        referent: CREDENTIAL1_ID.to_string(),
        rev_reg_id: None,
        cred_rev_id: None,
        attrs: map! {
                       "sex".to_string() => "male".to_string(),
                       "name".to_string() => "Alex".to_string(),
                       "height".to_string() => "175".to_string(),
                       "age".to_string() => "28".to_string()
                   }
    }
}

pub fn issuer_1_xyz_credential() -> CredentialInfo {
    CredentialInfo {
        schema_id: xyz_schema_id(),
        cred_def_id: issuer_1_xyz_cred_def_id(),
        referent: CREDENTIAL2_ID.to_string(),
        rev_reg_id: None,
        cred_rev_id: None,
        attrs: map! {
                       "status".to_string() => "partial".to_string(),
                       "period".to_string() => "8".to_string()
                   }
    }
}

pub fn issuer_2_gvt_credential() -> CredentialInfo {
    CredentialInfo {
        schema_id: gvt_schema_id(),
        cred_def_id: issuer_2_gvt_cred_def_id(),
        referent: CREDENTIAL3_ID.to_string(),
        rev_reg_id: None,
        cred_rev_id: None,
        attrs: map! {
                       "sex".to_string() => "male".to_string(),
                       "name".to_string() => "Alexander".to_string(),
                       "height".to_string() => "170".to_string(),
                       "Age".to_string() => "28".to_string()
                   }
    }
}

pub fn credential_def_json() -> String {
    r#"{
           "ver":"1.0",
           "id":"NcYxiDXkpYi6ov5FcYDi1e:3:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:CL:TAG_1",
           "schemaId":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
           "type":"CL",
           "tag":"TAG_1",
           "value":{
              "primary":{
                 "n":"94752773003676215520340390286428145970577435379747248974837494389412082076547661891067434652276048522392442077335235388384984508621151996372559370276527598415204914831299768834758349425880859567795461321350412568232531440683627330032285846734752711268206613305069973750567165548816744023441650243801226580089078611213688037852063937259593837571943085718154394160122127891902723469618952030300431400181642597638732611518885616750614674142486169255034160093153314427704384760404032620300207070597238445621198019686315730573836193179483581719638565112589368474184957790046080767607443902003396643479910885086397579016949",
                 "s":"69412039600361800795429063472749802282903100455399422661844374992112119187258494682747330126416608111152308407310993289705267392969490079422545377823004584691698371089275086755756916575365439635768831063415050875440259347714303092581127338698890829662982679857654396534761554232914231213603075653629534596880597317047082696083166437821687405393805812336036647064899914817619861844092002636340952247588092904075021313598848481976631171767602864723880294787434756140969093416957086578979859382777377267118038126527549503876861370823520292585383483415337137062969402135540724590433024573312636828352734474276871187481042",
                 "r":{
                    "master_secret": "51663676247842478814965591806476166314018329779100758392678204435864101706276421100107118776199283981546682625125866769910726045178868995629346547166162207336629797340989495021248125384357605197654315399409367101440127312902706857104045262430326903112478154165057770802221835566137181123204394005042244715693211063132775814710986488082414421678086296488865286754803461178476006057306298883090062534704773627985221339716152111236985859907502262026150818487846053415153813804554830872575193396851274528558072704096323791923604931528594861707067370303707070124331485728734993074005001622035563911923643592706985074084035",
                    "age":"90213462228557102785520674066817329607065098280886260103565465379328385444439123494955469500769864345819799623656302322427095342533906338563811194606234218499052997878891037890681314502037670093285650999142741875494918117023196753133733183769000368858655309319559871473827485381905587653145346258174022279515774231018893119774525087260785417971477049379955435611260162822960318458092151247522911151421981946748062572207451174079699745404644326303405628719711440096340436702151418321760375229323874027809433387030362543124015034968644213166988773750220839778654632868402703075643503247560457217265822566406481434257658",
                    "height":"5391629214047043372090966654120333203094518833743674393685635640778311836867622750170495792524304436281896432811455146477306501487333852472234525296058562723428516533641819658096275918819548576029252844651857904411902677509566190811985500618327955392620642519618001469964706236997279744030829811760566269297728600224591162795849338756438466021999870256717098048301453122263380103723520670896747657149140787953289875480355961166269553534983692005983375091110745903845958291035125718192228291126861666488320123420563113398593180368102996188897121307947248313167444374640621348136184583596487812048321382789134349482978",
                    "name":"77620276231641170120118188540269028385259155493880444038204934044861538875241492581309232702380290690573764595644801264135299029620031922004969464948925209245961139274806949465303313280327009910224580146266877846633558282936147503639084871235301887617650455108586169172459479774206351621894071684884758716731250212971549835402948093455393537573942251389197338609379019568250835525301455105289583537704528678164781839386485243301381405947043141406604458853106372019953011725448481499511842635580639867624862131749700424467221215201558826025502015289693451254344465767556321748122037274143231500322140291667454975911415",
                    "sex":"9589127953934298285127566793382980040568251918610023890115614786922171891298122457059996745443282235104668609426602496632245081143706804923757991602521162900045665258654877250328921570207935035808607238170708932487500434929591458680514420504595293934408583558084774019418964434729989362874165849497341625769388145344718883550286508846516335790153998186614300493752317413537864956171451048868305380731285315760405126912629495204641829764230906698870575251861738847175174907714361155400020318026100833368698707674675548636610079631382774152211885405135045997623813094890524761824654025566099289284433567918244183562578"
                 },
                 "rctxt":"60293229766149238310917923493206871325969738638348535857162249827595080348039120693847207728852550647187915587987334466582959087190830489258423645708276339586344792464665557038628519694583193692804909304334143467285824750999826903922956158114736424517794036832742439893595716442609416914557200249087236453529632524328334442017327755310827841619727229956823928475210644630763245343116656886668444813463622336899670813312626960927341115875144198394937398391514458462051400588820774593570752884252721428948286332429715774158007033348855655388287735570407811513582431434394169600082273657382209764160600063473877124656503",
                 "z":"70486542646006986754234343446999146345523665952265004264483059055307042644604796098478326629348068818272043688144751523020343994424262034067120716287162029288580118176972850899641747743901392814182335879624697285262287085187745166728443417803755667806532945136078671895589773743252882095592683767377435647759252676700424432160196120135306640079450582642553870190550840243254909737360996391470076977433525925799327058405911708739601511578904084479784054523375804238021939950198346585735956776232824298799161587408330541161160988641895300133750453032202142977745163418534140360029475702333980267724847703258887949227842"
              },
              "revocation":null
           }
        }"#.to_string()
}

pub fn proof_request_attr_and_predicate() -> String {
    json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
        }).to_string()
}

pub fn proof_request_attr() -> String {
    json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({}),
        }).to_string()
}

pub fn proof_json() -> String {
    r#"{
            "proof":{
                "proofs":[
                    {
                        "primary_proof":{
                            "eq_proof":{"revealed_attrs":{"name":"1139481716457488690172217916278103335"},"a_prime":"73051896986344783783621559954466052240337632808477729510525777007534198657123370460809453476237905269777928500034476888078179811369103091702326392092669222868996323974762333077146800752404116534730748685092400106417894776122280960547391515814302192999142386455183675790870578615457141270148590712693325301185445330992767208427208215818892089082206123243055148017865514286222759353929656015594529211154843197464055996993778878163967106658629893439206203941596066380562586058713924055616953462170537040600604826428201808405436865130230174790116739542071871153581967170346076628186863101926791732126528122264782281465094","e":"26894279258848531841414955598838798345606055130059418263879278878511424413654641307014787224496208858379991228288791608261549931755104416","v":"769593829417540943566687651216000708099616242062220026508500847265211856977241087739974159673381844796906987056271685312217722655254322996792650873775611656861273544234724432321045515309211146266498852589181986850053751764534235454974453901933962390148609111520973909072559803423360526975061164422239685006387576029266210201929872373313392190241424322333321394922891207577033519614434276723347140746548441162607411616008633618021962845423830579218345578253882839612570986096830936195064001459565147361336597305783767484298283647710212770870573787603073109857430854719681849489345098539472090186844042540487233617799636327572785715912348265648433678177765454231546725849288046905854444755145184654162149010359429569273734847400697627028832950969890252877892391103230391674009825009176344665382964776819962789472959504523580584494299815960094679820651071251157496967617834816772303813309035759721203718921501821175528106375","m":{"age":"1143281854280323408461665818853228702279803847691030529301464848501919856277927436364331044530711281448694432838145799412204154542183613877104383361274202256495017144684827419222","sex":"13123681697669364600723785784083768668401173003182555407713667959884184961072036088391942098105496874381346284841774772987179772727928471347011107103459387881602408580853389973314","height":"5824877563809831190436025794795529331411852203759926644567286594845018041324472260994302109635777382645241758582661313361940262319244084725507113643699421966391425299602530147274","master_secret":"8583218861046444624186479147396651631579156942204850397797096661516116684243552483174250620744158944865553535495733571632663325011575249979223204777745326895517953843420687756433"},"m2":"5731555078708393357614629066851705238802823277918949054467378429261691189252606979808518037016695141384783224302687321866277811431449642994233365265728281815807346591371594096297"},
                            "ge_proofs":[]
                        },
                        "non_revoc_proof":null
                    }
                ],
                "aggregated_proof":{"c_hash":"83823592657816121785961198553253620031199104930943156818597639614860312075063","c_list":[[2,66,174,183,214,178,122,180,186,63,14,80,155,85,150,14,217,66,149,176,133,171,1,26,238,182,223,250,20,5,23,250,187,84,179,207,13,147,67,92,135,47,152,151,93,9,90,133,13,250,155,255,236,150,10,32,56,173,28,213,29,208,126,57,225,129,173,51,233,189,32,201,139,82,153,42,8,222,131,35,246,39,85,114,168,183,150,197,192,212,171,99,158,9,192,212,61,24,7,95,188,144,164,79,43,149,163,156,241,105,34,114,197,160,90,232,244,72,122,177,186,233,82,107,1,66,231,153,178,57,101,174,240,63,7,50,168,21,134,165,133,105,244,106,115,4,93,227,249,77,58,24,219,122,95,128,87,249,247,119,163,1,197,94,230,66,56,58,203,213,201,219,52,134,122,200,20,210,10,225,231,124,232,0,34,112,168,133,157,202,13,47,132,162,140,159,133,104,24,133,150,66,116,106,250,18,9,84,4,249,4,184,75,216,144,55,119,233,139,217,138,27,215,38,114,20,34,209,179,90,237,184,124,207,14,59,104,25,219,37,162,82,5,24,12,20,94,208,227,162,61,76,247,121,109,93,6]]}
            },
            "requested_proof":{
                "revealed_attrs":{
                    "attr1_referent":{"sub_proof_index":0,"raw":"Alex","encoded":"1139481716457488690172217916278103335"}
                },
                "self_attested_attrs":{},
                "unrevealed_attrs":{},
                "predicates":{}
            },
            "identifiers":[
                {
                    "schema_id":"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
                    "cred_def_id":"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0:TAG_1",
                    "rev_reg_id":null,
                    "timestamp":null
                }
            ]
        }"#.to_string()
}

pub fn schemas_for_proof() -> String {
    let schema_id = gvt_schema_id();
    json!({
            schema_id: serde_json::from_str::<Schema>(&gvt_schema_json()).unwrap()
        }).to_string()
}

pub fn cred_defs_for_proof() -> String {
    json!({
            issuer_1_gvt_cred_def_id(): serde_json::from_str::<CredentialDefinition>(&credential_def_json()).unwrap()
        }).to_string()
}

pub fn get_credential_for_attr_referent(credentials_json: &str, referent: &str) -> CredentialInfo {
    let credentials: CredentialsForProofRequest = serde_json::from_str(&credentials_json).unwrap();
    let credentials_for_referent = credentials.attrs.get(referent).unwrap();
    credentials_for_referent[0].cred_info.clone()
}

pub fn get_credential_for_predicate_referent(credentials_json: &str, referent: &str) -> CredentialInfo {
    let credentials: CredentialsForProofRequest = serde_json::from_str(&credentials_json).unwrap();
    let credentials_for_referent = credentials.predicates.get(referent).unwrap();
    credentials_for_referent[0].cred_info.clone()
}

pub fn tails_writer_config() -> String {
    let mut base_dir = environment::tmp_path();
    base_dir.push("tails");

    let json = json!({
                "base_dir": base_dir.to_str().unwrap(),
                "uri_pattern":"",
            });
    json.to_string()
}

pub fn init_common_wallet() -> (&'static str, &'static str, &'static str, &'static str) {
    lazy_static! {
                    static ref COMMON_WALLET_INIT: Once = ONCE_INIT;

                }

    unsafe {
        COMMON_WALLET_INIT.call_once(|| {
            test::cleanup_storage();

            pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

            //1. Create and Open wallet
            wallet::create_wallet(ANONCREDS_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();
            let wallet_handle = wallet::open_wallet(ANONCREDS_WALLET_CONFIG, WALLET_CREDENTIALS).unwrap();

            //2. Issuer1 Creates GVT CredentialDefinition
            let (issuer1_gvt_cred_deg_id, issuer1_gvt_credential_def_json) =
                issuer_create_credential_definition(wallet_handle,
                                                    ISSUER_DID,
                                                    &gvt_schema_json(),
                                                    TAG_1,
                                                    None,
                                                    Some(&default_cred_def_config())).unwrap();

            //3. Issuer1 Creates XYZ CredentialDefinition
            let (issuer1_xyz_cred_deg_id, issuer1_xyz_credential_def_json) =
                issuer_create_credential_definition(wallet_handle,
                                                    ISSUER_DID,
                                                    &xyz_schema_json(),
                                                    TAG_1,
                                                    None,
                                                    Some(&default_cred_def_config())).unwrap();

            //4. Issuer2 Creates GVT CredentialDefinition
            let (issuer2_gvt_cred_def_id, issuer2_gvt_credential_def_json) =
                issuer_create_credential_definition(wallet_handle,
                                                    ISSUER_DID_2,
                                                    &gvt_schema_json(),
                                                    TAG_1,
                                                    None,
                                                    Some(&default_cred_def_config())).unwrap();

            //5. Issuer1 Creates GVT CredentialOffer
            let issuer1_gvt_credential_offer = issuer_create_credential_offer(wallet_handle, &issuer1_gvt_cred_deg_id).unwrap();

            //6. Issuer1 Creates XYZ CredentialOffer
            let issuer1_xyz_credential_offer = issuer_create_credential_offer(wallet_handle, &issuer1_xyz_cred_deg_id).unwrap();

            //7. Issuer2 Creates GVT CredentialOffer
            let issuer2_gvt_credential_offer = issuer_create_credential_offer(wallet_handle, &issuer2_gvt_cred_def_id).unwrap();

            //8. Prover creates MasterSecret
            prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET).unwrap();

            // Issuer1 issues GVT Credential
            //9. Prover creates  Credential Request
            let (issuer1_gvt_credential_req, issuer1_gvt_credential_req_metadata) = prover_create_credential_req(wallet_handle,
                                                                                                                 DID_MY1,
                                                                                                                 &issuer1_gvt_credential_offer,
                                                                                                                 &issuer1_gvt_credential_def_json,
                                                                                                                 COMMON_MASTER_SECRET).unwrap();
            //10. Issuer1 creates GVT Credential
            let (issuer1_gvt_cred, _, _) = issuer_create_credential(wallet_handle,
                                                                    &issuer1_gvt_credential_offer,
                                                                    &issuer1_gvt_credential_req,
                                                                    &gvt_credential_values_json(),
                                                                    None,
                                                                    None).unwrap();

            //11. Prover stores Credential
            prover_store_credential(wallet_handle,
                                    CREDENTIAL1_ID,
                                    &issuer1_gvt_credential_req_metadata,
                                    &issuer1_gvt_cred,
                                    &issuer1_gvt_credential_def_json,
                                    None).unwrap();

            // Issuer1 issue XYZ Credential
            //12. Prover Creates Credential Request
            let (issuer1_xyz_credential_req, issuer1_xyz_credential_req_metadata) = prover_create_credential_req(wallet_handle,
                                                                                                                 DID_MY1,
                                                                                                                 &issuer1_xyz_credential_offer,
                                                                                                                 &issuer1_xyz_credential_def_json,
                                                                                                                 COMMON_MASTER_SECRET).unwrap();
            //13. Issuer1 Creates XYZ Credential
            let (issuer1_xyz_cred, _, _) = issuer_create_credential(wallet_handle,
                                                                    &issuer1_xyz_credential_offer,
                                                                    &issuer1_xyz_credential_req,
                                                                    &xyz_credential_values_json(),
                                                                    None,
                                                                    None).unwrap();

            //14. Prover stores Credential
            prover_store_credential(wallet_handle,
                                    CREDENTIAL2_ID,
                                    &issuer1_xyz_credential_req_metadata,
                                    &issuer1_xyz_cred,
                                    &issuer1_xyz_credential_def_json,
                                    None).unwrap();

            // Issuer2 issues GVT Credential
            //15. Prover Creates Credential Request
            let (issuer2_gvt_credential_req, issuer2_gvt_credential_req_metadata) = prover_create_credential_req(wallet_handle,
                                                                                                                 DID_MY1,
                                                                                                                 &issuer2_gvt_credential_offer,
                                                                                                                 &issuer2_gvt_credential_def_json,
                                                                                                                 COMMON_MASTER_SECRET).unwrap();

            //16. Issuer2 Creates XYZ Credential
            let (issuer2_gvt_cred, _, _) = issuer_create_credential(wallet_handle,
                                                                    &issuer2_gvt_credential_offer,
                                                                    &issuer2_gvt_credential_req,
                                                                    &gvt2_credential_values_json(),
                                                                    None,
                                                                    None).unwrap();

            //17. Prover Stores Credential
            prover_store_credential(wallet_handle,
                                    CREDENTIAL3_ID,
                                    &issuer2_gvt_credential_req_metadata,
                                    &issuer2_gvt_cred,
                                    &issuer2_gvt_credential_def_json,
                                    None).unwrap();

            let res = mem::transmute(&issuer1_gvt_credential_def_json as &str);
            mem::forget(issuer1_gvt_credential_def_json);
            CREDENTIAL_DEF_JSON = res;

            let res = mem::transmute(&issuer1_gvt_credential_offer as &str);
            mem::forget(issuer1_gvt_credential_offer);
            CREDENTIAL_OFFER_JSON = res;

            let res = mem::transmute(&issuer1_gvt_credential_req as &str);
            mem::forget(issuer1_gvt_credential_req);
            CREDENTIAL_REQUEST_JSON = res;

            let res = mem::transmute(&issuer1_gvt_cred as &str);
            mem::forget(issuer1_gvt_cred);
            CREDENTIAL_JSON = res;

            wallet::close_wallet(wallet_handle).unwrap();
        });

        (CREDENTIAL_DEF_JSON, CREDENTIAL_OFFER_JSON, CREDENTIAL_REQUEST_JSON, CREDENTIAL_JSON)
    }
}

pub fn multi_steps_issuer_preparation(wallet_handle: i32,
                                      did: &str,
                                      schema_name: &str,
                                      schema_attrs: &str) -> (String, String, String, String) {
    let (schema_id, schema_json) = issuer_create_schema(did,
                                                        schema_name,
                                                        SCHEMA_VERSION,
                                                        schema_attrs).unwrap();

    let (cred_def_id, cred_def_json) = issuer_create_credential_definition(wallet_handle,
                                                                           did,
                                                                           &schema_json,
                                                                           TAG_1,
                                                                           None,
                                                                           Some(&default_cred_def_config())).unwrap();

    (schema_id, schema_json, cred_def_id, cred_def_json)
}

pub fn multi_steps_issuer_revocation_preparation(wallet_handle: i32,
                                                 did: &str,
                                                 schema_name: &str,
                                                 schema_attrs: &str,
                                                 revoc_reg_def_config: &str) -> (String, String, String, String, String, String, String, i32) {
    // Issuer creates schema
    let (schema_id, schema_json) = issuer_create_schema(did,
                                                        schema_name,
                                                        SCHEMA_VERSION,
                                                        schema_attrs).unwrap();

    // Issuer creates credential definition
    let (cred_def_id, cred_def_json) = issuer_create_credential_definition(wallet_handle,
                                                                           did,
                                                                           &schema_json,
                                                                           TAG_1,
                                                                           None,
                                                                           Some(&revocation_cred_def_config())).unwrap();

    // Issuer creates revocation registry
    let tails_writer_config = tails_writer_config();
    let tails_writer_handle = blob_storage::open_writer("default", &tails_writer_config).unwrap();

    let (rev_reg_id, revoc_reg_def_json, revoc_reg_entry_json) =
        issuer_create_and_store_revoc_reg(wallet_handle,
                                          did,
                                          None,
                                          TAG_1,
                                          &cred_def_id,
                                          revoc_reg_def_config,
                                          tails_writer_handle).unwrap();

    let blob_storage_reader_handle = blob_storage::open_reader(TYPE, &tails_writer_config).unwrap();

    (schema_id, schema_json, cred_def_id, cred_def_json, rev_reg_id, revoc_reg_def_json, revoc_reg_entry_json, blob_storage_reader_handle)
}

pub fn multi_steps_create_credential(prover_master_secret_id: &str,
                                     prover_wallet_handle: i32,
                                     issuer_wallet_handle: i32,
                                     cred_id: &str,
                                     cred_values: &str,
                                     cred_def_id: &str,
                                     cred_def_json: &str) {
    // Issuer creates Credential Offer
    let cred_offer_json = issuer_create_credential_offer(issuer_wallet_handle, &cred_def_id).unwrap();

    // Prover creates Credential Request
    let (cred_req, cred_req_metadata) = prover_create_credential_req(prover_wallet_handle,
                                                                     DID_MY1,
                                                                     &cred_offer_json,
                                                                     &cred_def_json,
                                                                     prover_master_secret_id).unwrap();

    // Issuer creates Credential
    let (cred_json, _, _) = issuer_create_credential(issuer_wallet_handle,
                                                     &cred_offer_json,
                                                     &cred_req,
                                                     &cred_values,
                                                     None,
                                                     None).unwrap();

    // Prover stores received Credential
    prover_store_credential(prover_wallet_handle,
                            cred_id,
                            &cred_req_metadata,
                            &cred_json,
                            &cred_def_json,
                            None).unwrap();
}

pub fn multi_steps_create_revocation_credential(prover_master_secret_id: &str,
                                                prover_wallet_handle: i32,
                                                issuer_wallet_handle: i32,
                                                credential_id: &str,
                                                cred_values: &str,
                                                cred_def_id: &str,
                                                cred_def_json: &str,
                                                rev_reg_id: &str,
                                                revoc_reg_def_json: &str,
                                                blob_storage_reader_handle: i32)
                                                -> (String, Option<String>) {
    // Issuer creates Credential Offer for Prover
    let cred_offer_for_prover1_json = issuer_create_credential_offer(issuer_wallet_handle, cred_def_id).unwrap();

    // Prover creates Credential Request
    let (prover1_cred_req_json, prover1_cred_req_metadata_json) = prover_create_credential_req(prover_wallet_handle,
                                                                                               DID_MY1,
                                                                                               &cred_offer_for_prover1_json,
                                                                                               cred_def_json,
                                                                                               prover_master_secret_id).unwrap();

    // Issuer creates Credential for Prover1
    let (prover1_cred_json, prover1_cred_rev_id, revoc_reg_delta1_json) = issuer_create_credential(issuer_wallet_handle,
                                                                                                   &cred_offer_for_prover1_json,
                                                                                                   &prover1_cred_req_json,
                                                                                                   &cred_values,
                                                                                                   Some(rev_reg_id),
                                                                                                   Some(blob_storage_reader_handle)).unwrap();
    let revoc_reg_delta1_json = revoc_reg_delta1_json;
    let prover1_cred_rev_id = prover1_cred_rev_id.unwrap();

    // Prover1 stores Credential
    prover_store_credential(prover_wallet_handle,
                            credential_id,
                            &prover1_cred_req_metadata_json,
                            &prover1_cred_json,
                            &cred_def_json,
                            Some(&revoc_reg_def_json)).unwrap();

    (prover1_cred_rev_id, revoc_reg_delta1_json)
}
