import json

import pytest

from indy import anoncreds
from tests.conftest import path_home as x_path_home, pool_name as x_pool_name, wallet_type as x_wallet_type, \
    wallet_runtime_config as x_wallet_runtime_config, wallet_handle_cleanup as x_wallet_handle_cleanup, \
    wallet_handle as x_wallet_handle, xwallet as x_xwallet


@pytest.fixture(scope="session")
def path_home():
    # noinspection PyTypeChecker
    for i in x_path_home():
        yield i


@pytest.fixture(scope="session")
def pool_name():
    return x_pool_name()


@pytest.fixture(scope="session")
def wallet_name():
    return 'anoncreds_common_wallet'


@pytest.fixture(scope="session")
def wallet_type():
    return x_wallet_type()


@pytest.fixture(scope="session")
def wallet_runtime_config():
    return x_wallet_runtime_config()


@pytest.fixture(scope="session")
def xwallet_cleanup():
    return False


# noinspection PyUnusedLocal
@pytest.fixture(scope="session")
def xwallet(event_loop, pool_name, wallet_name, wallet_type, xwallet_cleanup, path_home):
    xwallet_gen = x_xwallet(event_loop, pool_name, wallet_name, wallet_type, xwallet_cleanup, path_home, None)
    yield next(xwallet_gen)
    next(xwallet_gen)


@pytest.fixture(scope="session")
def wallet_handle_cleanup():
    return x_wallet_handle_cleanup()


@pytest.fixture(scope="session")
def wallet_handle(event_loop, wallet_name, xwallet, wallet_runtime_config, wallet_handle_cleanup):
    wallet_handle_gen = \
        x_wallet_handle(event_loop, wallet_name, xwallet, wallet_runtime_config, None, wallet_handle_cleanup)
    yield next(wallet_handle_gen)
    next(wallet_handle_gen)


@pytest.fixture(scope="session")
def default_cred_def_config():
    return json.dumps({"support_revocation": False})


@pytest.fixture(scope="session")
def tag():
    return "tag1"


@pytest.fixture(scope="session")
def id_credential_1():
    return "id_credential_1"


@pytest.fixture(scope="session")
def id_credential_2():
    return "id_credential_2"


@pytest.fixture(scope="session")
def id_credential_3():
    return "id_credential_3"


@pytest.fixture(scope="session")
def issuer_did():
    return "NcYxiDXkpYi6ov5FcYDi1e"


@pytest.fixture(scope="session")
def issuer_did_2():
    return "VsKV7grR1BUE29mG2Fm2kX"


@pytest.fixture(scope="session")
def prover_did():
    return "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"


def build_id(identifier: str, marker: str, word1: str, word2: str):
    delimiter = ":"
    return identifier + delimiter + marker + delimiter + word1 + delimiter + word2


@pytest.fixture(scope="session")
async def gvt_schema_tuple(issuer_did):
    return await anoncreds.issuer_create_schema(issuer_did, "gvt", "1.0", json.dumps(["name", "age", "sex", "height"]))


@pytest.fixture(scope="session")
def gvt_schema_id(gvt_schema_tuple):
    (schema_id, _) = gvt_schema_tuple
    return schema_id


@pytest.fixture(scope="session")
def gvt_schema(gvt_schema_tuple):
    (_, schema_json) = gvt_schema_tuple
    return json.loads(schema_json)


@pytest.fixture(scope="session")
async def gvt_schema_json(gvt_schema_tuple):
    (_, schema_json) = gvt_schema_tuple
    return schema_json


@pytest.fixture(scope="session")
async def xyz_schema_tuple(issuer_did):
    return await anoncreds.issuer_create_schema(issuer_did, "xyz", "1.0", json.dumps(["status", "period"]))


@pytest.fixture(scope="session")
def xyz_schema_id(xyz_schema_tuple):
    (schema_id, _) = xyz_schema_tuple
    return schema_id


@pytest.fixture(scope="session")
def xyz_schema(xyz_schema_tuple):
    (_, schema_json) = xyz_schema_tuple
    return json.loads(schema_json)


@pytest.fixture(scope="session")
async def xyz_schema_json(xyz_schema_tuple):
    (_, schema_json) = xyz_schema_tuple
    return schema_json


@pytest.fixture(scope="session")
def master_secret_id():
    return "common_master_secret_name"


@pytest.fixture(scope="session")
def issuer_1_gvt_cred_def_id(issuer_did, gvt_schema_id):
    return build_id(issuer_did, "3", "CL", gvt_schema_id)


@pytest.fixture(scope="session")
def issuer_1_xyz_cred_def_id(issuer_did, xyz_schema_id):
    return build_id(issuer_did, "3", "CL", xyz_schema_id)


@pytest.fixture(scope="session")
def issuer_2_gvt_cred_def_id(issuer_did_2, gvt_schema_id):
    return build_id(issuer_did_2, "3", "CL", gvt_schema_id)


@pytest.fixture(scope="session")
def credential_offer(credential_def_id):
    return {
        "credential_def_id": credential_def_id,
        "nonce": "12345678",
        "key_correctness_proof": {
            "c": "40983841062403114696351105468714473190092945361781922980284036284848255102181",
            "xz_cap": "213464720484089744362618540118202909431724596227070046572799595772146912256777238162172299903278411669692103853805864530643873553457214761106883317761027120427539370942370962393243000202519433219346910021513926418330149853541417254168868250711855156634731074143905917765057402152157911116556639972831731251935718700937677764992720913700241472793061399051130783380568753128719906986229399266611916274676591429895036623790763995576609783339973743504299970516925347817430071017432740932267531399950884977365109931994767334355518355993866340663897276958741231492087880026463329993917787126111107043213858416969200146216919724481899757743335320111464498989394895406342200088098092363803780335427461",
            "xr_cap": {
                "age": "428551817403717345496666225339792093375807052545681523267375890344316772830711607454203456132869878139513849454441680693213618371219989695825485734499740269394536720808195352535836241683894046154422601982139187282874298237872016126349886775459552952772019552508536658271791921689339332976876098011786609958781975486656330572741194023204426484952261795227021910523213710060753808292174119734371160619117183552237714301799404966964550022274275304553920529452381773256570920044318126774874528737572897410668952113510485617038975261423255802900402119311766709164727308287389090038205944899002702399329914812218958604251060872288309671952489910694000990814697006984351987506978581894973799401592211",
                "height": "411657369769012290217285704283327594441623457932786578784814099069982464122634859546109989278195207865751652851745795318226450710002590664034182094394469565119510105316695655956892227633500179147318075067585551834678079812461536830790528252558459232505987954022333485123452283103020768033499524916346016600527748463901810773406682862302906632677327131603416116045070433377354243916235831969703006718595171020843309342047669615896623593427073236313132690348520294916012881797187163597866196204765064323603066770657621979137899593499157032831120638301825327588467780000638198687916279993936278677557249905181200340769464532921226462449219936857163316761986408035441733035901688059567989300117882",
                "name": "869322975653258356083915983990526979728408630010817458571291042713509811788475578121058954003892631131467356704604351238331780894204614591041662184716582274656810743747383953498817535302551304877321807454020020152874312640585570851593902460677745364557958108957714916300524302083561141490749493731078047092029485764829854763907822331747337420362381448975375124969403844387156269077805007874513313426920627145892677170274354768722781701010279364942880411045002631531693007422568259696565436694533169879230288912084861052431355880089929921941631003274141039364415665970063262534617675591334457554707139889594182371941548502441982219614399925468254660740292400093419154694726435630358592702798293",
                "sex": "80391464088175985479491145491149691676821702211894975540979533937774408491785219834122762944971811095537317848654416410580026667952335862665033546961195841179049138780634877378888139872391903804566992942049889566118414459535461354834916790111149556147862372720479995171424595620702416860508557772658191427975040372006893431243929350584258325646184152369207604974849840003307909256680303811690743921237117427932325288396536300357224457903672928805464748280413883820982138162562660615091490216949908906589977916965927522227509078411025411863914347809289131586019476288990589861921562466467956967324009607175203666778312423056471533641756179235960697838324279027572094105302470967687825859737087"
            }
        }
    }


@pytest.fixture(scope="session")
def issuer_1_gvt_cred_offer(issuer_1_gvt_cred_def_id):
    return credential_offer(issuer_1_gvt_cred_def_id)


@pytest.fixture(scope="session")
def issuer_1_gvt_cred_offer_json(credential_offer_issuer_1_schema_1):
    return json.dumps(credential_offer_issuer_1_schema_1)


@pytest.fixture(scope="session")
def issuer_1_xyz_cred_offer_json(issuer_1_xyz_cred_def_id):
    return credential_offer(issuer_1_xyz_cred_def_id)


@pytest.fixture(scope="session")
def issuer_1_xyz_cred_offer_json(credential_offer_issuer_1_schema_2):
    return json.dumps(credential_offer_issuer_1_schema_2)


@pytest.fixture(scope="session")
def issuer_2_gvt_cred_offer(issuer_2_gvt_cred_def_id):
    return credential_offer(issuer_2_gvt_cred_def_id)


@pytest.fixture(scope="session")
def issuer_2_gvt_cred_offer_json(credential_offer_issuer_2_schema_1):
    return json.dumps(credential_offer_issuer_2_schema_1)


@pytest.fixture(scope="session")
def gvt_cred_values():
    return {
        "sex": {
            "raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103"},
        "name": {"raw": "Alex", "encoded": "1139481716457488690172217916278103335"},
        "height": {"raw": "175", "encoded": "175"},
        "age": {"raw": "28", "encoded": "28"}
    }


@pytest.fixture(scope="session")
def gvt_cred_values_json(gvt_cred_values):
    return json.dumps(gvt_cred_values)


@pytest.fixture(scope="session")
def gvt_cred_values_2():
    return {
        "sex": {
            "raw": "male", "encoded": "2142657394558967239210949258394838228692050081607692519917028371144233115103"},
        "name": {"raw": "Alexander", "encoded": "21332817548165488690172217217278169335"},
        "height": {"raw": "170", "encoded": "170"},
        "age": {"raw": "28", "encoded": "28"}
    }


@pytest.fixture(scope="session")
def gvt_2_cred_values_json(gvt_cred_values_2):
    return json.dumps(gvt_cred_values_2)


@pytest.fixture(scope="session")
def xyz_cred_values():
    return {
        "status": {"raw": "partial", "encoded": "51792877103171595686471452153480627530895"},
        "period": {"raw": "8", "encoded": "8"}
    }


@pytest.fixture(scope="session")
def xyz_cred_values_json(xyz_cred_values):
    return json.dumps(xyz_cred_values)


@pytest.fixture(scope="session")
def predicate_value():
    return 18


@pytest.fixture(scope="session")
def proof_req(predicate_value):
    return {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {"name": "name"}
        },
        "requested_predicates": {
            "predicate1_referent": {
                "name": "age",
                "p_type": ">=",
                "p_value": predicate_value
            }
        }
    }


@pytest.fixture(scope="session")
def proof_req_json(proof_req):
    return json.dumps(proof_req)


@pytest.fixture(scope="session")
def credential_def(gvt_schema_id, issuer_1_gvt_cred_def_id):
    return {
        "ver": "1.0",
        "id": issuer_1_gvt_cred_def_id,
        "schemaId": gvt_schema_id,
        "type": "CL",
        "tag": "TAG_1",
        "value": {
            "primary": {
                "n": "98206882360971928369196105915435152527421956254937750475519267499007806663391571745484918982656093953343286311715548338386494827455950694504247990618614540419719453821052280488939560651768030023102922820022379607954311939968228869879319131861698867825538440885652224664797521755146985516082218573248868494259134372222451038634416706505724369699221157072214834080250647666971872365320719129818653651393802166817972469916801549123118785068276179352018046467135720349194185079737083023041289485895659600718780884897144649124680774354249625885140765757058565747007750287241711870659835903274623276781539864049164047211293",
                "s": "61355798642237969535831640161382793364508530051127861297543308153275544436705256345547159693357456886759605659527594585887008007925479262045693906803523614229369431018781715080548074679180844984156237718326043756282619909771415031659237808334078917672023307978006447478064113502482427755418102866946546269241478790365574406112581680340823843385185183546436420545318109544663205976811848080462267091699497986826292068407090980481248864817900641618075334981878600450301644015232311133269714264095563618016925650484800967029924171823347865765870910530664369947765943509373402197002400734850531391943703675161079118595940",
                "rms": "91503921556334093243681583905976809372147290567533229401566644509255303788560246979515739903411793120717460448520717663988803817275596788232191777969503727045208285929150590155772700102563663858621498558087459402754932839570079001200452738216980374822081249180129868989388615063555529743431180783535737776204867267180307434564503960837581775139881330438613791067455484379261699228902179153811604076533687497101880764157033741703439790821706460778846848228072123914596814264361963425857076366199857712468880297408555332148592410108885650806402624105552371004821241073612284744310482873722439882300555294717653278604358",
                "r": {
                    "age": "62436888527497266562241632168895963239884040168488645775808670145385847001383105067174675710288799460197494381313313874575635822060710638304599206163867687680833746741024307035600574385421369643343662643945148302821718334129041567650003532026492030969733075343961045095979286842967526971994726316286155053569740176740451724473202516953091029768616601779644803704630966281326921022004818765633434771934622925065943660784946909601001488078466299460754498413648749376316894517385395279057235563709069691612880775831876446142317330879392906696251094683074099707589933967066995940844123816756857099105868409019345112685552",
                    "sex": "91837235575630719252776129914807536685557223878543031469876692127116765922362701818772482185024900450179892491308055930560939840598411926406853735110788073498999087882794558156889089332048370514627720061325694549285422823909646710469941808077303198792460536908281828588007433374086338597982302391946193743698907895126247236896651417345498586845580947151631050288154263362870887793632458026512529794661907864100793109593540442374176964831877718668877291263537312045023613461663675397132148432132319821045347763477548464484005494684721268788612209503855451814551800847731754288211693571462715399946321792530201844812686",
                    "name": "94557655970359763071788298994776595106356264395111381095163812816109912149854488830327543060428891262387326543721348922460316395451650435884138229041558375546613960138555573276884141928832868170334977037872366527513274114180153379086686059140667211974844760063773558887702128603735814058929919777811148353228018276072746364445003323447169681783619899050545894824379123918967224382268784862546432232442834009026493297357753623375904195826613376758637378850798144844411649023843003128925963090557681135370493459679213244332718594628087968682793895163863141947605851977382741148160048182891844287673727936943443834324975",
                    "height": "65760409986513807165277067452238625602976282305489225269386540909947038689327730067847178236316825930326230704731494170607006727044592127723801615060646637991655927543097004820911920716894249811571307235993294584276975802420953280698796275154998808491585671614097340325078715285084675318300554352915213325703575941251354023032753296280266910970115248650392935524541072273617029667155708974651547317982107796941861730875865684900063727040708804585467197015885592334850761627410862918261928841755969596053205667891719965229172154378144043871384017706872070570992615652651060786262382908812242083305484280790695205708133"
                },
                "rctxt": "64984441431268925979196377380511974816339019734721341180866656405225558036171558067242362547190094022337034274803836405948165851254119954947006444637326414912875339858550844011168978691484696961999842771571312435052336451643667618359805260080143191742053817319692189519472131082545397130443799403747021428224569216375890502605632741250598169212759616162819568658640169282763002116726420569662467202356557648248991423175162111894410406680221439525944495227782491979333795011609010368858193175237730719831361148948713702970385661350256899628598517067042362015272658080877100053818295366602497539363914947703328053386207",
                "z": "20977594959882886944229500446794250461964739183115113198513639888383026989086068574412764868069956167376187055226682817778158846146382758624367900490778639035562836376766741551367340858579892597295831159905085954084212051112711220116374593052724380409815637162198417152319592062839875229752550092814919229083873591540538467864261567046839734744502456586530200275598560478105666411537423297921523555725933156406112257304862257815965664553158073946973447703637389492121877656875734443980093764336494058929440511963192385776971895497620503302088020390842302066281149985107038828180003693367004171601521516868536976425411"
            },
            "revocation": None
        }
    }


@pytest.fixture(scope="session")
def credential_def_json(credential_def):
    return json.dumps(credential_def)


@pytest.fixture(scope="session")
async def prepopulated_wallet(wallet_handle, gvt_schema_json, xyz_schema_json, gvt_cred_values_json,
                              gvt_2_cred_values_json, xyz_cred_values_json, issuer_did, issuer_did_2, master_secret_id,
                              prover_did, tag, default_cred_def_config, id_credential_1, id_credential_2,
                              id_credential_3):
    # Create GVT credential by Issuer1
    (issuer1_gvt_cred_deg_id, issuer1_gvt_credential_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, issuer_did, gvt_schema_json, tag,
                                                               None, default_cred_def_config)

    # Create XYZ credential by Issuer1
    (issuer1_xyz_cred_deg_id, issuer1_xyz_credential_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, issuer_did, xyz_schema_json, tag,
                                                               None, default_cred_def_config)

    # Create GVT credential by Issuer2
    (issuer2_gvt_cred_def_id, issuer2_gvt_credential_def_json) = \
        await anoncreds.issuer_create_and_store_credential_def(wallet_handle, issuer_did_2, gvt_schema_json, tag,
                                                               None, default_cred_def_config)

    issuer_1_gvt_credential_offer_json = \
        await anoncreds.issuer_create_credential_offer(wallet_handle, issuer1_gvt_cred_deg_id)
    issuer_1_xyz_credential_offer_json = \
        await anoncreds.issuer_create_credential_offer(wallet_handle, issuer1_xyz_cred_deg_id)
    issuer_2_gvt_credential_offer_json = \
        await anoncreds.issuer_create_credential_offer(wallet_handle, issuer2_gvt_cred_def_id)

    await anoncreds.prover_create_master_secret(wallet_handle, master_secret_id)

    (issuer_1_gvt_cred_req, issuer_1_gvt_cred_req_metadata) = \
        await anoncreds.prover_create_credential_req(wallet_handle, prover_did, issuer_1_gvt_credential_offer_json,
                                                     issuer1_gvt_credential_def_json, master_secret_id)

    (issuer_1_gvt_cred, _, _) = \
        await anoncreds.issuer_create_credential(wallet_handle, issuer_1_gvt_credential_offer_json,
                                                 issuer_1_gvt_cred_req, gvt_cred_values_json, None, None)

    await anoncreds.prover_store_credential(wallet_handle, id_credential_1, issuer_1_gvt_cred_req,
                                            issuer_1_gvt_cred_req_metadata,
                                            issuer_1_gvt_cred, issuer1_gvt_credential_def_json, None)

    (issuer_1_xyz_cred_req, issuer_1_xyz_cred_req_metadata) = \
        await anoncreds.prover_create_credential_req(wallet_handle, prover_did, issuer_1_xyz_credential_offer_json,
                                                     issuer1_xyz_credential_def_json, master_secret_id)

    (issuer_1_xyz_cred, _, _) = \
        await anoncreds.issuer_create_credential(wallet_handle, issuer_1_xyz_credential_offer_json,
                                                 issuer_1_xyz_cred_req, xyz_cred_values_json, None, None)

    await anoncreds.prover_store_credential(wallet_handle, id_credential_2, issuer_1_xyz_cred_req,
                                            issuer_1_xyz_cred_req_metadata, issuer_1_xyz_cred,
                                            issuer1_xyz_credential_def_json, None)

    (issuer_2_gvt_cred_req, issuer_2_gvt_cred_req_metadata) = \
        await anoncreds.prover_create_credential_req(wallet_handle, prover_did, issuer_2_gvt_credential_offer_json,
                                                     issuer2_gvt_credential_def_json, master_secret_id)

    (issuer_2_gvt_cred, _, _) = \
        await anoncreds.issuer_create_credential(wallet_handle, issuer_2_gvt_credential_offer_json,
                                                 issuer_2_gvt_cred_req, gvt_2_cred_values_json, None, None)

    await anoncreds.prover_store_credential(wallet_handle, id_credential_3, issuer_2_gvt_cred_req,
                                            issuer_2_gvt_cred_req_metadata,
                                            issuer_2_gvt_cred, issuer2_gvt_credential_def_json, None)

    return issuer1_gvt_credential_def_json, issuer_1_gvt_credential_offer_json, issuer_1_gvt_cred_req, \
           issuer_1_gvt_cred_req_metadata, issuer_1_gvt_cred,
