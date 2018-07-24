extern crate serde_json;

use std::collections::HashMap;
use object_cache::ObjectCache;
use api::VcxStateType;
use utils::error;
use connection;
use messages;
use messages::GeneralMessage;
use messages::proofs::proof_message::{ProofMessage };
use messages::proofs::proof_request::{ ProofRequestMessage };
use messages::extract_json_payload;
use messages::to_u8;

use credential_def::{ retrieve_credential_def };
use schema::{ LedgerSchema };

use utils::libindy::anoncreds;
use utils::libindy::crypto;

use settings;
use utils::httpclient;
use utils::constants::{ DEFAULT_SERIALIZE_VERSION, SEND_MESSAGE_RESPONSE, CREDS_FROM_PROOF_REQ };

use serde_json::{Value};

use error::ToErrorCode;
use error::proof::ProofError;

lazy_static! {
    static ref HANDLE_MAP: ObjectCache<DisclosedProof>  = Default::default();
}

impl Default for DisclosedProof {
    fn default() -> DisclosedProof
    {
        DisclosedProof {
            source_id: String::new(),
            my_did: None,
            my_vk: None,
            state: VcxStateType::VcxStateNone,
            proof_request: None,
            proof: None,
            link_secret_alias: settings::DEFAULT_LINK_SECRET_ALIAS.to_string(),
            their_did: None,
            their_vk: None,
            agent_did: None,
            agent_vk: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisclosedProof {
    source_id: String,
    my_did: Option<String>,
    my_vk: Option<String>,
    state: VcxStateType,
    proof_request: Option<ProofRequestMessage>,
    proof: Option<ProofMessage>,
    link_secret_alias: String,
    their_did: Option<String>,
    their_vk: Option<String>,
    agent_did: Option<String>,
    agent_vk: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedCreds {
    pub self_attested_attributes: HashMap<String, String>,
    pub requested_attrs: HashMap<String, (String, bool)>,
    pub requested_predicates: HashMap<String, String>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CredInfo {
    pub referent: String,
    pub schema_id: String,
    pub cred_def_id: String,
}

fn credential_def_identifiers(credentials: &str) -> Result<Vec<(String, String, String, String)>, ProofError> {
    let mut rtn = Vec::new();

    let credentials: Value = serde_json::from_str(credentials)
        .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

    if let Value::Object(ref map) = credentials["attrs"] {
        for (requested_attr, value) in map {
            if let Value::Object(ref attr_obj) = value["cred_info"] {

                let cred_uuid = match attr_obj.get("referent") {
                    Some(i) => if i.is_string() { i.as_str().unwrap() } else { return Err(ProofError::CommonError(error::INVALID_JSON.code_num))},
                    None => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
                };

                let schema_id = match attr_obj.get("schema_id") {
                    Some(i) => if i.is_string() { i.as_str().unwrap() } else { return Err(ProofError::CommonError(error::INVALID_JSON.code_num))},
                    None => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
                };

                let cred_def_id = match attr_obj.get("cred_def_id") {
                    Some(i) => if i.is_string() { i.as_str().unwrap() } else { return Err(ProofError::CommonError(error::INVALID_JSON.code_num))},
                    None => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
                };

                rtn.push((requested_attr.to_string(),
                          cred_uuid.to_string(),
                          schema_id.to_string(),
                          cred_def_id.to_string()))
            }
        }
    }
    Ok(rtn)
}

impl DisclosedProof {

    fn set_proof_request(&mut self, req: ProofRequestMessage) {self.proof_request = Some(req)}

    fn get_state(&self) -> u32 {self.state as u32}
    fn set_state(&mut self, state: VcxStateType) {self.state = state}

    fn retrieve_credentials(&self) -> Result<String, ProofError> {
        if settings::test_indy_mode_enabled() {return Ok(CREDS_FROM_PROOF_REQ.to_string())}

        let proof_req = self.proof_request.as_ref().ok_or(ProofError::ProofNotReadyError())?;
        let indy_proof_req = serde_json::to_string(&proof_req.proof_request_data)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        anoncreds::libindy_prover_get_credentials_for_proof_req(&indy_proof_req)
            .map_err(|err| ProofError::CommonError(err))
    }

    fn _find_schemas(&self, credentials_identifiers: &Vec<(String, String, String, String)>) -> Result<String, ProofError> {
        let mut rtn: HashMap<String, Value> = HashMap::new();

        for &(ref attr_id, ref cred_uuid, ref schema_id, ref cred_def_id) in credentials_identifiers {
            let schema = LedgerSchema::new_from_ledger(schema_id)
                .or( Err(ProofError::InvalidSchema()))?;

            let schema_json = serde_json::from_str(&schema.schema_json)
                .or(Err(ProofError::InvalidSchema()))?;

            rtn.insert(schema_id.to_owned(), schema_json);
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&rtn)
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }
    }

    fn _find_credential_def(&self, credentials_identifiers: &Vec<(String, String, String, String)>) -> Result<String, ProofError> {

        let mut rtn: HashMap<String, Value> = HashMap::new();

        for &(ref attr_id, ref cred_uuid, ref schema_id, ref cred_def_id) in credentials_identifiers {

            let (_, credential_def) = retrieve_credential_def(cred_def_id)
                .or(Err(ProofError::InvalidCredData()))?;

            let credential_def = serde_json::from_str(&credential_def)
                .or(Err(ProofError::InvalidCredData()))?;

            rtn.insert(cred_def_id.to_owned(), credential_def);
        }

        match rtn.is_empty() {
            false => Ok(serde_json::to_string(&rtn)
                .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?),
            true => Err(ProofError::CommonError(error::INVALID_JSON.code_num))
        }

    }

    fn _build_requested_credentials(&self,
                                    credentials_identifiers: &Vec<(String, String, String, String)>,
                                    self_attested_attrs: &str) -> Result<String, ProofError> {
        let mut rtn: Value = json!({
              "self_attested_attributes":{},
              "requested_attributes":{},
              "requested_predicates":{}
        });
        //Todo: need to do same for predicates and self_attested
        //Todo: need to handle if the attribute is not revealed
        if let Value::Object(ref mut map) = rtn["requested_attributes"] {
            for &(ref attr_id, ref cred_uuid, ref schema_id, ref cred_def_id) in credentials_identifiers {

                let insert_val = json!({"cred_id": cred_uuid, "revealed": true});
                map.insert(attr_id.to_owned(), insert_val);
            }
        }

        let self_attested_attrs: Value = serde_json::from_str(self_attested_attrs)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;
        rtn["self_attested_attributes"] = self_attested_attrs;

        let rtn = serde_json::to_string(&rtn)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        Ok(rtn)
    }

    fn generate_proof(&mut self, credentials: &str, self_attested_attrs: &str) -> Result<u32, ProofError> {
        if settings::test_indy_mode_enabled() {return Ok(error::SUCCESS.code_num)}

        let proof_req = self.proof_request.as_ref()
            .ok_or(ProofError::CreateProofError())?;
        let proof_req_data_json = serde_json::to_string(&proof_req.proof_request_data)
            .or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?;

        let credentials_identifiers = credential_def_identifiers(credentials)?;
        let requested_credentials = self._build_requested_credentials(&credentials_identifiers,
                                                                      self_attested_attrs)?;
        let schemas = match self._find_schemas(&credentials_identifiers) {
            Ok(x) => x,
            Err(_) => format!("{{}}"),
        };
        let credential_defs_json = match self._find_credential_def(&credentials_identifiers) {
            Ok(x) => x,
            Err(_) => format!("{{}}"),
        };
        let revoc_regs_json = Some("{}");

        let proof = anoncreds::libindy_prover_create_proof(&proof_req_data_json,
                                                           &requested_credentials,
                                                          &self.link_secret_alias,
                                                           &schemas,
                                                          &credential_defs_json,
                                                          revoc_regs_json).map_err(|ec| ProofError::CommonError(ec))?;

        let mut proof_msg = ProofMessage::new();
        proof_msg.libindy_proof = proof;
        self.proof = Some(proof_msg);

        Ok(error::SUCCESS.code_num)
    }

    fn send_proof(&mut self, connection_handle: u32) -> Result<u32, ProofError> {
        debug!("sending proof via connection connection: {}", connection_handle);
        // There feels like there's a much more rusty way to do the below.
        self.my_did = Some(connection::get_pw_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.my_vk = Some(connection::get_pw_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.agent_did = Some(connection::get_agent_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.agent_vk = Some(connection::get_agent_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.their_did = Some(connection::get_their_pw_did(connection_handle).or(Err(ProofError::ProofConnectionError()))?);
        self.their_vk = Some(connection::get_their_pw_verkey(connection_handle).or(Err(ProofError::ProofConnectionError()))?);


        debug!("verifier_did: {:?} -- verifier_vk: {:?} -- agent_did: {:?} -- agent_vk: {:?} -- remote_vk: {:?}",
               self.my_did,
               self.agent_did,
               self.agent_vk,
               self.their_vk,
               self.my_vk);

        let e_code: u32 = error::INVALID_CONNECTION_HANDLE.code_num;

        let local_their_did = self.their_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_their_vk = self.their_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_agent_did = self.agent_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_agent_vk = self.agent_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_my_did = self.my_did.as_ref().ok_or(ProofError::ProofConnectionError())?;
        let local_my_vk = self.my_vk.as_ref().ok_or(ProofError::ProofConnectionError())?;

        let proof_req = self.proof_request.as_ref().ok_or(ProofError::CreateProofError())?;
        let ref_msg_uid = proof_req.msg_ref_id.as_ref().ok_or(ProofError::CreateProofError())?;

        let proof = match settings::test_indy_mode_enabled() {
            false => {
                let proof: &ProofMessage = self.proof.as_ref().ok_or(ProofError::CreateProofError())?;
                serde_json::to_string(&proof).or(Err(ProofError::CommonError(error::INVALID_JSON.code_num)))?
            },
            true => String::from(r#"{"version":null,"to_did":null,"from_did":"2hoqvcwupRTUNkXn6ArYzs","proof_request_id":null,"libindy_proof":"{\"proof\":{\"proofs\":[{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"name\":\"1139481716457488690172217916278103335\"},\"a_prime\":\"53312829942635033213547785663032527700667569977758152702065911726040780590237870482614642520743973046831751397226330629402283915298010755286359042214254186073131293256617522243788869067062459048819049093000861706976559454353054415224217541508251828901230315727452194906707146182038292774837874158123905132204754494099957935893710311307611185556253562731302085643190380093796712150073863236374671766683830116643642814511141387202276677179978778023868191620803031555572096505493695458848336750072872824082946166624025826402185079078955571640643781644608569289737247239944293374495461225509029397867929209294024213709788\",\"e\":\"20682359643412682295831027361890575475838523648741421551903080595188181833333576182907108270431142275883424564833283730774400687766783937\",\"v\":\"977904633616831258258208792044552558313031225274080786369260591379176356693699547546357564320894782538899916819106615861870184406180902549346128003521172715089106402210625380702987862241587585159809860067416270881226844522905301128237139154351788756006744361780087312281145149472912522702788932085436364256473799830420855157934668266924875801216119120004335695991621286240487033787853268010363269151831620402262266602483169804514477708724227545481582471160023611649085383221053639652355027928189779678096568066482567763571624994971564070998443128322674311603044207676915009631014778272133804086604499562982743685276710603982347204200935086324427392193112051934585025409396669790007564450985142207219311632469976008804262561102394462677381575113741321505633197641754405090922962040100369869763363395309722094214098657007292521892941813525965557387184306029245810578685136184682462250087780312322969478679080585625021520210\",\"m\":{\"height\":\"12263289415832681222191231307742005757620106449305717047784670792788875695539753364781375492671454287793766184443302346922042521188980267021306912439837184942463831344267513114539\",\"sex\":\"15037155454094459527731434206481114319240521503794827852070701802307168241949946923083664784355274753205297122386791667526757839001084114446291855325384881755476108903076644102524\",\"age\":\"14501872409940327998412893157081512363089790754079941699437594851467665059803693524100074428544944084570214138897184729758726613071650314480579074556700200366688895565145382360698\"},\"m1\":\"5111614492731222933682238606986742012649186832802695436293583878319992887971581188876806759447594326173957365879113848145931336889892325622590289133581694699335215858805105064331\",\"m2\":\"15455662254366457503819701346580083774039475855758049615942760668169011894662780202836319433863205952152879959998434723106532779215477449781534318996974001599714966396856374581486\"},\"ge_proofs\":[{\"u\":{\"3\":\"13338797419010819430313158414239588911533702998711113636065786974475516880285905693812255129869592972310118234542465190202031458473416955680083762161755097308017613738409014450412\",\"1\":\"799492542075042460468743423224449526404299930570906500607913945716944502802045365895975118206585431769760718077298247129653462146705083199486640323388289350373502832336220747818\",\"2\":\"1410669775663194506029421689725160524313531018451204956432149799559481970811992598634649361849192146578548675891162987371736423348002714852997563863676564228939963027067794874871\",\"0\":\"2284908542447285531642293235205764719596189096401757772990648246811741899742027021760873962759808826767095318153983117478606733551882287443416905369212830402625019520316000619122\"},\"r\":{\"DELTA\":\"446417378686684511234026773355708125239824632634437692777199207510593775769661320517505495782808123901931509966024639039038108997849239457006650910166579521188145804592891114429551884188163168202240716931710467143952184978891694998359503161214624561825833908073711391637139239900433773736886398246516765844798574260038720645282874847988024757269025066849487120769033835284892684827974590847202980928318284802296184879430262352838537995329568347704443032358436401925907006203800781805302411290413797547703643680523816450964907269365740333954767250872005080383786962836349437972233102686007070151913231405673053111357257541070794190346854946155348000938794168313555054935097511280055857357714044575099713604514868026213\",\"0\":\"76323471287537725745058968243946536923132059523832270143088476741530931794292362597818595246023215646685960860543985669003637356653968580726012831272952726648097379707978025827324461370746080302334946615636230982573057929178570795576025377975675772919883441071319109877674255209425751831351871122268677949866093732374914212035903588005718999977836601694274936653297249597377458306018690838897074948900601500608361610629829956778952463888545043329274127468867562886471766759587776471429982222060455628836484851435349858651386947282024263753935919991184370769532443355748040360929886906396781059431869286808165873481291594432375474267943726789479499451113914106577274491433396849021865876595625676087390243000023624762\",\"2\":\"92201512116493836828631529733129001060186027428900226009667207946775663412763049323063566382410298659680809630308918826774715965924345348873964163349957666233303908982512533058477107440041164974471923932525392131507208058972114013554261186876156391013030249940165674350329969469493690172204462521643306554255514954820056300279294082665993982068489632717990288083501332286520044058836966987218446835040483313621165017234567451018319889811119538866717954177080460042489566032046447251130130245574659638664255489998777489482211990568338261294591442065689728409233570738254849320926466856819550224710437157933072455901697612704266158325888789692029073807847963389087595009709947178982240740611685681284283779085698675948\",\"1\":\"523519181672047308247759255710896855210622133048051288689628050530593819825092742868259096958091636388599567602947573803006117544722711419791216490462806808879903445126483144871684220922578271665249669036925968176205614101006831570497324462920653772741371462368258040337051743083703373268865149218215217028092524525773305797542980367455534854526011265982129123432958566815191454606539227183131194637859155089868145241173590901657135733891325388928960829686799252575344385280631185145400972822216509709304106388326226886283646678379038689195399855788521138175316361682508141995206060813895339550522593818086203090245120243475360699742963456947833927949579002835590226242375390852007351288105362306570115755708545985217\",\"3\":\"219708053581575062582533005752631707205208991118487656765616830751022269293827575497929496961405315686245411560943699849420449187007381492241392380392082110189993691561802473698218934318387062653841098498548279928332075909765566959065512235256509146413077084637192127841275265722661868451059419828110871219158575869744195099188903695177096530456644287214222020238239519153912497327047225695946382054980942953751030857203011122068559656187038626791446516654182155158539057712697737764700558352779296500466327830395357680350841458615344128789679867036656697605720376436366435199261670114032178466528256435156834078889201567594491266464345415135571924808730955663963107805898452164603883815760222880803183090122927021906\"},\"mj\":\"14501872409940327998412893157081512363089790754079941699437594851467665059803693524100074428544944084570214138897184729758726613071650314480579074556700200366688895565145382360698\",\"alpha\":\"31918703918424605421867306637529380827987785788405050672774138736166394193904621089524533944750195114129096119755051526846725254763824200066138161138712728191811663989790595399577931777057069644826760716174215502760332823087353208209062750316348096800751400775616367613846591019341998518739176827591884828803457769836022658736894307470228021107390985355341006358063783380352553203686844117554456841862664698030419319060016855160424705460267488204254799487731578966486350864297692169864269632174633849489841974267276169865234681524317353667399250655802076478193790198931703997043279311660619853321692326370677880789301081973803355206050854006163585246822795898668673143707227426024087230394272183289129116861001488380021845460347354339123316270676159384403687413396981497957623652714825893844224460981469334265786569340866948245386177786943\",\"t\":{\"3\":\"37972928040287019966279705290077177120438916881166595729433656845236894764203450825352358745872530678723193673168699261559252347206134931851900792354929811701409594264464224953419079438712468720585420884977482943060379064169436974390111782700278522811285372914014699402037920450145036232607643190702877276047529693903313274481563898145881289947298191322165264501424360596211660104692237473307294793997549168958412186734308994849480325136098603164600244189033368937216041232700684717195143448897309921696037996821682161245633099514032672108362471847287780767536527922125290354007208461534506680429079325673303847122602\",\"1\":\"83797923111124698337190350740625419461331674597317499285711196059201116233696351462219655473974285728417042636321002662540084865059877415988492658002455681892357965734412778441359694363956150504207034364774632269626588616820867786845681629308573877664225545768569004728653608256309649757797799701694734953511407991945606233015704498351349679028096727127592824167470239121617521236284944552022042989195045796550206930535014890946963714532865413773302556921033807408941052143462861253036857235374142831681609638723266113292539696031604138697651663589237083275182589077537865812203675363931276014854720987778657614102744\",\"0\":\"2650503753268559234325045605088774291426714447530920032686699660331132818418294401386458113929015669970054987046869952559938250072323202428595050264148451597342024999195988956825995585576913035057727067574585765007573631661190106325951562647973999556307904156602972256772368626124956359113948479337998473700163736913949914618468786982078961864054155716098967233493298331429094961852050251480937244097084480079338650069474343849653498204836647208425673909949338344039864962930360099888379112227879147124479674454219360564016333692287612795255249839516579685644216601710763951730704414780889479757764771824686121135664\",\"DELTA\":\"43065835375665819440999515421835663370003698766999136081645804528693116242835242268976886541500885372919193595521649742583847412078937558594134652898329531345474973694469474048143077702226876323148384500134900126983776005048815938010984852671864994349715180582574635986767847143457191095540293933564404627275951176669589446874803422805492459886354589921566504955505072907201600897292446445273226759645152347760045030326954742075710148481072174908814490535664213161063279049745367193309329034076163035585786466248661857406761347994603442904391759603262653605643737187511663278904722133109597876198342257959851721834192\",\"2\":\"46628952298959333080121710882308601694841107884016671262481120139852326078270985558474211468413535251365897343039473985423394663439915886543709585947919510897780945029279321912919654098202591041333546305345226891491934752585644557915489535154254673543901496062795297123823959517190269228530111782605034098586928917098782686018936730269712739545898008102022872832813918298315656085362237855944363927676075909182683069003791720095898011148294690599783422365311217737781228088563580874619330671307258995382310169708388804474101048348557353152301570472790186309273164115806835950131701314455074661797762932617450697701058\"},\"predicate\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}]},\"non_revoc_proof\":null}],\"aggregated_proof\":{\"c_hash\":\"15107591148123921137146194357185316416658106446533633516768364316943089580042\",\"c_list\":[[1,166,81,167,129,168,9,195,206,209,106,181,150,16,195,117,63,177,44,215,8,36,96,16,191,201,123,173,242,90,252,147,29,164,230,22,148,201,70,219,1,174,68,212,172,20,247,106,239,96,84,62,213,205,22,27,56,32,13,87,86,154,47,87,223,133,48,103,181,173,8,101,1,63,95,172,109,254,160,238,44,184,16,143,59,38,30,170,49,125,197,221,222,54,58,210,252,137,145,70,99,142,233,158,162,174,6,107,31,14,50,111,244,137,122,2,16,218,2,88,200,72,40,141,139,188,43,48,237,245,169,162,89,112,81,0,23,46,191,152,221,93,125,105,238,1,222,157,68,61,23,189,7,113,162,61,230,172,143,8,157,13,229,221,214,79,84,246,75,156,240,76,124,93,39,30,167,122,175,246,117,105,6,203,207,87,217,40,215,20,209,188,175,6,250,31,13,64,1,250,64,154,232,64,46,84,7,180,121,52,1,230,122,30,98,146,220,6,168,19,57,78,77,114,84,96,48,162,54,35,30,141,186,163,99,185,123,24,217,139,218,227,141,236,105,160,241,83,201,152,66,98,37,253,162,195,220],[20,254,252,22,231,75,18,170,179,109,220,230,27,219,149,32,214,209,149,133,50,88,217,78,5,96,145,131,250,159,61,108,65,219,94,75,52,84,251,169,192,84,135,58,131,26,36,149,27,224,106,61,5,194,196,247,53,71,217,115,115,132,215,240,231,19,131,106,252,9,88,66,237,19,105,221,127,177,250,4,127,23,167,68,255,182,234,55,202,199,159,66,1,181,22,40,112,144,219,78,251,172,8,193,139,175,3,160,6,255,76,232,215,67,157,239,13,179,173,174,158,120,7,226,48,99,36,129,146,85,194,64,86,26,242,248,192,182,225,193,10,146,113,249,183,64,189,101,69,118,249,46,155,206,85,150,245,66,4,163,212,168,222,76,136,56,135,229,188,125,178,188,21,65,44,234,5,180,140,65,53,166,191,2,189,207,73,50,129,191,82,166,60,187,79,211,63,56,226,25,192,0,128,192,7,238,189,90,86,104,27,109,27,29,173,79,147,84,233,75,239,245,26,179,148,50,134,109,223,59,184,98,107,186,187,38,86,156,133,144,251,188,199,72,236,181,121,83,42,40,1,194,57,176,174,48],[2,151,206,178,220,199,113,118,112,154,103,78,56,113,141,74,10,56,152,93,116,185,146,211,42,2,254,129,120,167,193,111,62,83,136,100,137,60,10,89,85,123,222,79,208,55,202,115,234,43,194,248,210,138,193,144,157,71,101,81,43,33,151,141,55,92,42,114,29,192,189,151,220,131,246,173,109,195,249,125,111,169,33,225,48,222,212,173,45,212,122,65,211,165,171,31,71,92,245,126,6,243,245,205,125,66,2,239,248,10,205,128,15,81,8,193,131,142,211,152,239,47,107,132,165,148,143,146,109,223,120,238,31,28,96,25,152,215,85,135,253,233,19,199,201,99,222,166,79,176,71,41,161,134,175,119,139,9,131,186,181,136,113,138,35,98,110,117,210,238,230,61,190,149,39,225,67,42,138,77,55,246,7,180,41,3,148,146,45,43,130,135,63,53,167,185,242,80,38,15,91,238,13,13,137,124,14,101,150,253,50,47,111,240,104,207,75,127,150,104,108,182,19,6,169,100,100,105,166,122,36,46,179,102,27,75,48,112,36,177,20,50,78,71,60,230,152,74,140,24,66,110,61,196,205,0,216],[1,113,95,89,11,69,152,90,18,216,24,156,66,191,203,117,10,208,99,133,64,42,122,96,194,177,70,138,177,159,33,124,184,183,112,154,58,220,182,81,86,131,189,209,35,124,193,235,128,126,150,198,178,115,184,184,89,33,230,170,166,53,172,221,81,116,196,199,189,160,225,117,248,102,119,136,37,32,43,148,254,173,142,37,231,160,178,147,86,97,179,23,221,199,203,111,144,203,88,58,97,177,251,142,27,254,184,79,127,115,255,220,135,230,244,226,64,155,177,99,31,101,29,137,82,138,130,206,27,214,44,60,123,165,240,93,120,221,175,197,86,209,165,169,126,120,26,87,208,76,147,245,97,146,19,50,122,150,78,122,91,204,43,5,23,165,87,194,156,233,181,194,113,30,41,54,92,56,202,89,193,221,92,170,106,198,86,216,186,29,219,42,25,137,16,252,163,121,104,91,212,8,170,94,57,112,229,235,175,87,146,38,240,143,85,184,221,112,75,2,53,218,71,201,169,19,15,46,154,229,187,41,236,202,20,134,202,154,42,180,167,70,211,31,104,123,255,81,25,51,59,169,127,48,242,182,194],[1,44,205,180,74,73,89,226,22,38,162,169,161,207,183,58,59,73,38,49,235,171,190,130,141,255,139,247,73,23,181,130,100,78,170,42,180,180,45,149,34,47,199,128,32,65,11,41,49,64,154,228,143,180,162,242,111,153,39,240,72,135,156,80,213,119,51,221,200,46,160,251,0,163,1,97,198,190,173,148,231,6,208,5,246,176,236,50,17,251,251,81,211,98,220,156,137,242,110,4,8,33,150,228,60,165,152,86,166,222,198,166,146,84,61,113,100,61,2,172,74,34,220,127,233,119,206,92,107,53,40,124,206,56,181,238,154,45,96,171,219,46,160,7,6,255,55,169,116,170,143,35,128,238,84,206,13,224,149,97,238,195,108,47,67,253,195,103,194,155,52,231,91,57,6,25,123,45,11,68,23,174,108,40,165,39,63,239,46,121,234,1,242,182,75,83,79,218,251,173,227,193,171,30,87,95,28,175,175,134,236,66,29,119,76,224,84,77,215,29,188,12,201,226,22,244,149,30,177,159,181,23,30,77,112,46,180,160,56,93,124,20,39,232,119,38,198,7,235,207,38,197,229,102,158,182,170],[1,85,37,170,71,223,136,209,189,66,85,250,129,32,243,62,118,211,168,5,95,28,44,129,250,185,203,123,147,53,34,245,167,28,169,147,180,42,39,199,220,155,138,79,217,252,145,154,78,61,97,190,121,5,5,87,56,143,166,135,18,135,29,111,74,191,222,74,116,60,88,247,15,151,227,200,233,63,44,167,9,246,72,200,252,80,110,164,15,57,19,45,39,68,207,127,14,192,3,136,233,85,232,79,153,99,57,216,152,158,94,234,76,181,45,199,42,44,54,6,79,78,146,215,194,112,170,95,176,204,167,3,17,51,160,197,17,237,1,125,221,51,229,207,109,211,223,229,5,154,78,163,233,34,93,197,219,178,92,191,229,13,158,203,37,53,197,96,40,159,61,106,74,52,190,14,75,180,8,202,247,84,198,150,104,141,88,55,136,191,17,190,57,210,96,41,44,179,169,57,236,176,30,190,52,97,165,53,51,0,134,82,86,64,115,138,89,11,192,250,161,160,44,94,35,156,249,51,100,246,78,70,48,24,226,63,67,221,216,230,65,28,149,59,77,197,172,189,208,6,195,219,101,83,137,62,208]]}},\"requested_proof\":{\"revealed_attrs\":{\"attr1_referent\":{\"sub_proof_index\":0,\"raw\":\"Alex\",\"encoded\":\"1139481716457488690172217916278103335\"}},\"self_attested_attrs\":{\"attr3_referent\":\"8-800-300\"},\"unrevealed_attrs\":{\"attr2_referent\":{\"sub_proof_index\":0}},\"predicates\":{\"predicate1_referent\":{\"sub_proof_index\":0}}},\"identifiers\":[{\"schema_id\":\"NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0\",\"cred_def_id\":\"NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0\",\"rev_reg_id\":null,\"timestamp\":null}]}"}"#)
        };

        let data: Vec<u8> = connection::generate_encrypted_payload(local_my_vk, local_their_vk, &proof, "PROOF")
            .or(Err(ProofError::ProofConnectionError()))?;

        if settings::test_agency_mode_enabled() { httpclient::set_next_u8_response(SEND_MESSAGE_RESPONSE.to_vec()); }

        match messages::send_message().to(local_my_did)
            .to_vk(local_my_vk)
            .msg_type("proof")
            .agent_did(local_agent_did)
            .agent_vk(local_agent_vk)
            .edge_agent_payload(&data)
            .ref_msg_id(ref_msg_uid)
            .send_secure() {
            Ok(response) => {
                self.state = VcxStateType::VcxStateAccepted;
                return Ok(error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("could not send proof: {}", x);
                return Err(ProofError::CommonError(x));
            }
        }
    }

    fn set_source_id(&mut self, id: &str) { self.source_id = id.to_string(); }
    fn get_source_id(&self) -> &String { &self.source_id }
    fn to_string(&self) -> String {
        json!({
            "version": DEFAULT_SERIALIZE_VERSION,
            "data": json!(self),
        }).to_string()
    }
    fn from_str(s: &str) -> Result<DisclosedProof, ProofError> {
        let s:Value = serde_json::from_str(&s)
            .or(Err(ProofError::InvalidJson()))?;
        let proof: DisclosedProof= serde_json::from_value(s["data"].clone())
            .or(Err(ProofError::InvalidJson()))?;
        Ok(proof)
    }
}

//********************************************
//         HANDLE FUNCTIONS
//********************************************
fn handle_err(code_num: u32) -> u32 {
    if code_num == error::INVALID_OBJ_HANDLE.code_num {
        error::INVALID_DISCLOSED_PROOF_HANDLE.code_num
    }
    else {
        code_num
    }
}

pub fn create_proof(source_id: String, proof_req: String) -> Result<u32, ProofError> {
    debug!("creating disclosed proof with id: {}", source_id);

    let mut new_proof: DisclosedProof = Default::default();

    new_proof.set_source_id(&source_id);
    new_proof.set_proof_request(serde_json::from_str(&proof_req)
        .map_err(|_| ProofError::CommonError(error::INVALID_JSON.code_num))?);

    new_proof.set_state(VcxStateType::VcxStateRequestReceived);

    Ok(HANDLE_MAP.add(new_proof).map_err(|ec| ProofError::CommonError(ec))?)
}

pub fn get_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_state())
    }).map_err(handle_err)
}

// update_state is just the same as get_state for disclosed_proof
pub fn update_state(handle: u32) -> Result<u32, u32> {
    HANDLE_MAP.get(handle, |obj|{
        Ok(obj.get_state())
    })
}

pub fn to_string(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj|{
        Ok(DisclosedProof::to_string(&obj))
    })
}

pub fn from_string(proof_data: &str) -> Result<u32, ProofError> {
    let derived_proof: DisclosedProof = match DisclosedProof::from_str(proof_data) {
        Ok(x) => x,
        Err(y) => return Err(ProofError::CommonError(error::INVALID_JSON.code_num)),
    };

    let new_handle = HANDLE_MAP.add(derived_proof).map_err(|ec| ProofError::CommonError(ec))?;

    info!("inserting handle {} into proof table", new_handle);

    Ok(new_handle)
}

pub fn release(handle: u32) -> Result<(), u32> {
    HANDLE_MAP.release(handle).map_err(handle_err)
}

pub fn release_all() {
    match HANDLE_MAP.drain() {
        Ok(_) => (),
        Err(_) => (),
    };
}

pub fn send_proof(handle: u32, connection_handle: u32) -> Result<u32, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.send_proof(connection_handle).map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn generate_proof(handle: u32, credentials: String, self_attested_attrs: String) -> Result<u32, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.generate_proof(&credentials, &self_attested_attrs).map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn retrieve_credentials(handle: u32) -> Result<String, ProofError> {
    HANDLE_MAP.get_mut(handle, |obj|{
        obj.retrieve_credentials().map_err(|e| e.to_error_code())
    }).map_err(|ec| ProofError::CommonError(ec))
}

pub fn is_valid_handle(handle: u32) -> bool {
    HANDLE_MAP.has_handle(handle)
}

//TODO one function with credential
pub fn get_proof_request(connection_handle: u32, msg_id: &str) -> Result<String, ProofError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec()); }

    let message = messages::get_message::get_connection_messages(&my_did,
                                                                 &my_vk,
                                                                 &agent_did,
                                                                 &agent_vk,
                                                                 Some(vec![msg_id.to_string()])).map_err(|ec| ProofError::CommonError(ec))?;

    if message[0].msg_type.eq("proofReq") {
        let (_, msg_data) = match message[0].payload {
            Some(ref data) => {
                let data = to_u8(data);
                crypto::parse_msg(&my_vk, data.as_slice()).map_err(|ec| ProofError::CommonError(ec))?
            },
            None => return Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num))
        };

        let request = extract_json_payload(&msg_data).map_err(|ec| ProofError::CommonError(ec))?;
        let mut request: ProofRequestMessage = serde_json::from_str(&request)
           .or(Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num)))?;

        request.msg_ref_id = Some(message[0].uid.to_owned());
        Ok(serde_json::to_string_pretty(&request).unwrap())
    } else {
        Err(ProofError::CommonError(error::INVALID_MESSAGES.code_num))
    }
}

//TODO one function with credential
pub fn get_proof_request_messages(connection_handle: u32, match_name: Option<&str>) -> Result<String, ProofError> {
    let my_did = connection::get_pw_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let my_vk = connection::get_pw_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_did = connection::get_agent_did(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;
    let agent_vk = connection::get_agent_verkey(connection_handle).map_err(|e| ProofError::CommonError(e.to_error_code()))?;

    if settings::test_agency_mode_enabled() { ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec()); }

    let payload = messages::get_message::get_connection_messages(&my_did,
                                                                 &my_vk,
                                                                 &agent_did,
                                                                 &agent_vk,
                                                                 None).map_err(|ec| ProofError::CommonError(ec))?;

    let mut messages: Vec<ProofRequestMessage> = Default::default();

    for msg in payload {
        if msg.sender_did.eq(&my_did){ continue; }

        if msg.msg_type.eq("proofReq") {
            let (_, msg_data) = match msg.payload {
                Some(ref data) => {
                    let data = to_u8(data);
                    crypto::parse_msg(&my_vk, data.as_slice())
                        .map_err(|ec| ProofError::CommonError(ec))?
                },
                None => return Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num))
            };

            let req = extract_json_payload(&msg_data).map_err(|ec| ProofError::CommonError(ec))?;

            let mut req: ProofRequestMessage = serde_json::from_str(&req)
                .or(Err(ProofError::CommonError(error::INVALID_HTTP_RESPONSE.code_num)))?;

            req.msg_ref_id = Some(msg.uid.to_owned());
            messages.push(req);
        }
    }

    Ok(serde_json::to_string_pretty(&messages).unwrap())
}

pub fn get_source_id(handle: u32) -> Result<String, u32> {
    HANDLE_MAP.get(handle, |obj| {
        Ok(obj.get_source_id().clone())
    }).map_err(handle_err)
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use utils::libindy::wallet;
    use utils::constants::{ ADDRESS_CRED_ID, LICENCE_CRED_ID, ADDRESS_SCHEMA_ID, ADDRESS_CRED_DEF_ID, CRED_DEF_ID, SCHEMA_ID };
    use serde_json::Value;

    #[test]
    fn test_create_proof() {
        settings::set_defaults();
        assert!(create_proof("1".to_string(), ::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap() > 0);
    }

    #[test]
    fn test_create_fails() {
        settings::set_defaults();
        assert_eq!(create_proof("1".to_string(),"{}".to_string()).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_proof_cycle() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let connection_h = connection::build_connection("test_send_credential_offer").unwrap();

        let requests = get_proof_request_messages(connection_h, None).unwrap();
        let requests:Value = serde_json::from_str(&requests).unwrap();
        let requests = serde_json::to_string(&requests[0]).unwrap();

        let handle = create_proof("TEST_CREDENTIAL".to_owned(), requests).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap());
        send_proof(handle, connection_h).unwrap();
        assert_eq!(VcxStateType::VcxStateAccepted as u32, get_state(handle).unwrap());
    }

    #[test]
    fn get_state_test(){
        settings::set_defaults();
        let proof: DisclosedProof =  Default::default();
        assert_eq!(VcxStateType::VcxStateNone as u32, proof.get_state());
        let handle = create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        assert_eq!(VcxStateType::VcxStateRequestReceived as u32, get_state(handle).unwrap())
    }

    #[test]
    fn to_string_test() {
        settings::set_defaults();
        let handle = create_proof("id".to_string(),::utils::constants::PROOF_REQUEST_JSON.to_string()).unwrap();
        let serialized = to_string(handle).unwrap();
        let j:Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(j["version"], "1.0");
        DisclosedProof::from_str(&serialized).unwrap();
    }

    #[test]
    fn test_deserialize_fails() {
        assert_eq!(from_string("{}").err(),
        Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_find_schemas() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let creds = vec![cred1, cred2];

        let proof: DisclosedProof = Default::default();
        let schemas = proof._find_schemas(&creds).unwrap();
        assert!(schemas.len() > 0);
        assert!(schemas.contains(r#""id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","name":"test-licence""#));
    }

    #[test]
    fn test_find_schemas_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let credential_ids = Vec::new();
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_schemas(&credential_ids).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_find_credential_def() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let creds = vec![cred1, cred2];

        let proof: DisclosedProof = Default::default();
        let credential_def = proof._find_credential_def(&creds).unwrap();
        assert!(credential_def.len() > 0);
        assert!(credential_def.contains(r#""id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:2471","schemaId":"2471""#));
    }

    #[test]
    fn test_find_credential_def_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let credential_ids = Vec::new();
        let proof: DisclosedProof = Default::default();
        assert_eq!(proof._find_credential_def(&credential_ids).err(),
                   Some(ProofError::CommonError(error::INVALID_JSON.code_num)));
    }

    #[test]
    fn test_build_requested_credentials() {
        settings::set_defaults();
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let creds = vec![cred1, cred2];
        let self_attested_attrs = json!({
            "self_attested_attr_3": "my self attested 1",
            "self_attested_attr_4": "my self attested 2",
        }).to_string();

        let test: Value = json!({
              "self_attested_attributes":{
                  "self_attested_attr_3": "my self attested 1",
                  "self_attested_attr_4": "my self attested 2",
              },
              "requested_attributes":{
                  "height_1": {"cred_id": LICENCE_CRED_ID, "revealed": true },
                  "zip_2": {"cred_id": ADDRESS_CRED_ID, "revealed": true },
              },
              "requested_predicates":{}
        });

        let proof: DisclosedProof = Default::default();
        let requested_credential = proof._build_requested_credentials(&creds, &self_attested_attrs).unwrap();
        println!("requested_cred: {}", requested_credential);
        assert_eq!(test.to_string(), requested_credential);
    }

    #[test]
    fn test_get_proof_request() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        wallet::init_wallet("test_get_proof_request").unwrap();

        let connection_h = connection::build_connection("test_get_proof_request").unwrap();

        let request = get_proof_request(connection_h, "123").unwrap();
        assert!(request.len() > 50);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_retrieve_credentials() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_retrieve_credentials";
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);
        ::utils::libindy::payments::mint_tokens_and_set_fees(None, Some(10000000), None, false).unwrap();
        ::utils::libindy::anoncreds::tests::create_and_store_credential();
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (_, _, req, _) = ::utils::libindy::anoncreds::tests::create_proof();

        let mut proof_req = ProofRequestMessage::create();
        let mut proof: DisclosedProof = Default::default();
        proof_req.proof_request_data = serde_json::from_str(&req).unwrap();
        proof.proof_request = Some(proof_req);

        let retrieved_creds = proof.retrieve_credentials().unwrap();
        assert!(retrieved_creds.len() > 500);

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
    }

    #[test]
    fn test_retrieve_credentials_fails_with_no_proof_req() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "retrieve_credentials_fails_with_no_proof_req";

        let proof: DisclosedProof = Default::default();
        assert_eq!(proof.retrieve_credentials(), Err(ProofError::ProofNotReadyError()));
    }

    #[test]
    fn test_credential_def_identifiers() {
        let cred1 = ("height_1".to_string(), LICENCE_CRED_ID.to_string(), SCHEMA_ID.to_string(), CRED_DEF_ID.to_string() );
        let cred2 = ("zip_2".to_string(), ADDRESS_CRED_ID.to_string(), ADDRESS_SCHEMA_ID.to_string(), ADDRESS_CRED_DEF_ID.to_string() );
        let selected_credentials : Value = json!({
           "attrs":{
              "height_1":{
                "cred_info":{
                   "referent":LICENCE_CRED_ID,
                   "attrs":{
                      "sex":"male",
                      "age":"111",
                      "name":"Bob",
                      "height":"4'11"
                   },
                   "schema_id": SCHEMA_ID,
                   "cred_def_id": CRED_DEF_ID,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
              },
              "zip_2":{
                "cred_info":{
                   "referent":ADDRESS_CRED_ID,
                   "attrs":{
                      "address1":"101 Tela Lane",
                      "address2":"101 Wilson Lane",
                      "zip":"87121",
                      "state":"UT",
                      "city":"SLC"
                   },
                   "schema_id":ADDRESS_SCHEMA_ID,
                   "cred_def_id":ADDRESS_CRED_DEF_ID,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
             }
           },
           "predicates":{

           }
        });
        let creds = credential_def_identifiers(&selected_credentials.to_string()).unwrap();
        assert_eq!(creds, vec![cred1, cred2]);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_generate_proof() {
        use utils::libindy::wallet::delete_wallet;
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_generate_proof";
        match delete_wallet(wallet_name) {
            Ok(_) => {},
            Err(_) => {},
        };
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schema_id, _, cred_def_id, _, _, _, _, cred_id) = ::utils::libindy::anoncreds::tests::create_and_store_credential();

        let mut proof_req = ProofRequestMessage::create();
        let indy_proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "self_attested_attr_3": json!({
                   "name":"self_attested_attr",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();
        proof_req.proof_request_data = serde_json::from_str(&indy_proof_req).unwrap();

        let selected_credentials : Value = json!({
           "attrs":{
              "address1_1":{
                "cred_info":{
                   "referent":cred_id,
                   "attrs":{
                      "sex":"male",
                      "age":"111",
                      "name":"Bob",
                      "height":"4'11"
                   },
                   "schema_id": schema_id,
                   "cred_def_id": cred_def_id,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
              },
              "zip_2":{
                "cred_info":{
                   "referent":cred_id,
                   "attrs":{
                      "address1":"101 Tela Lane",
                      "address2":"101 Wilson Lane",
                      "zip":"87121",
                      "state":"UT",
                      "city":"SLC"
                   },
                   "schema_id":schema_id,
                   "cred_def_id":cred_def_id,
                   "rev_reg_id":null,
                   "cred_rev_id":null
                },
                "interval":null
             }
           },
           "predicates":{ }
        });

        let self_attested: Value = json!({
              "self_attested_attr_3":"attested_val"
        });

        let mut proof: DisclosedProof = Default::default();
        proof.proof_request = Some(proof_req);
        proof.link_secret_alias = "main".to_string();
        let generated_proof = proof.generate_proof(&selected_credentials.to_string(), &self_attested.to_string());

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
        assert!(generated_proof.is_ok());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_generate_self_attested_proof() {
        use utils::libindy::wallet::delete_wallet;
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_generate_self_attested_proof";
        match delete_wallet(wallet_name) {
            Ok(_) => {},
            Err(_) => {},
        };
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let mut proof_req = ProofRequestMessage::create();
        let indy_proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
               }),
               "zip_2": json!({
                   "name":"zip",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();
        proof_req.proof_request_data = serde_json::from_str(&indy_proof_req).unwrap();

        let selected_credentials : Value = json!({});

        let self_attested: Value = json!({
              "address1_1":"attested_address",
              "zip_2": "attested_zip"
        });

        let mut proof: DisclosedProof = Default::default();
        proof.proof_request = Some(proof_req);
        proof.link_secret_alias = "main".to_string();
        let generated_proof = proof.generate_proof(&selected_credentials.to_string(), &self_attested.to_string());

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
        assert!(generated_proof.is_ok());
    }
}
