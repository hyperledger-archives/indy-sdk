//
//  AnoncredsUtils.m
//  Indy-demo
//
//  Created by Kirill Neznamov on 24/05/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "AnoncredsUtils.h"
#import "TestUtils.h"

@interface AnoncredsUtils ()

@property(assign) BOOL isCommonWalletCreated;
@property(assign) IndyHandle walletHandle;
@property(strong) NSString *singletoneCredentialdefJson;
@property(strong) NSString *singletoneCredentialofferJson;
@property(strong) NSString *singletoneCredentialreqJson;
@property(strong) NSString *singletoneCredentialJson;

@end

@implementation AnoncredsUtils


+ (AnoncredsUtils *)sharedInstance {
    static AnoncredsUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [AnoncredsUtils new];
        instance.isCommonWalletCreated = false;
        instance.walletHandle = 0;
    });

    return instance;
}

// MARK: - Json configurators

- (NSString *)defaultCredentialDefConfig {
    return @"{\"support_revocation\": false}";
}

- (NSString *)getGvtSchemaId {
    return @"NcYxiDXkpYi6ov5FcYDi1e:1:gvt:1.0";
}

- (NSString *)getXyzSchemaId {
    return @"NcYxiDXkpYi6ov5FcYDi1e:1:xyz:1.0";
}

- (NSString *)getGvtSchemaJson {
    return [NSString stringWithFormat:@"{"
                                              "\"id\":\"%@\","
                                              "\"name\":\"gvt\","
                                              "\"version\":\"1.0\","
                                              "\"attrNames\":[\"age\",\"sex\",\"height\",\"name\"]}", [self getGvtSchemaId]];
}

- (NSString *)getXyzSchemaJson {
    return [NSString stringWithFormat:@"{"
                                              "\"id\":\"%@\","
                                              "\"name\":\"xyz\","
                                              "\"version\":\"1.0\","
                                              "\"attrNames\":[\"status\",\"period\"]}", [self getXyzSchemaId]];
}

- (NSString *)getIssuer1GvtCredDefId {
    return @"NcYxiDXkpYi6ov5FcYDi1e:2:NcYxiDXkpYi6ov5FcYDi1e:1:gvt:1.0:CL:TAG1";
}

- (NSString *)getIssuer1XyzCredDefId {
    return @"NcYxiDXkpYi6ov5FcYDi1e:2:NcYxiDXkpYi6ov5FcYDi1e:1:xyz:1.0:CL:TAG1";
}

- (NSString *)getIssuer2GvtCredDefId {
    return @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW:2:NcYxiDXkpYi6ov5FcYDi1e:1:gvt:1.0:CL:TAG1";
}

- (NSString *)getCredentialOfferJson:(NSString *)issuerDid
                           credDefId:(NSString *)credDefId {
    return [NSString stringWithFormat:@"{"\
            "\"issuer_did\":\"%@\"," \
            "\"cred_def_id\":\"%@\"," \
            "\"nonce\":\"12345678\"," \
            "\"key_correctness_proof\": {\"c\":\"1\",\"xz_cap\":\"1\",\"xr_cap\":{\"name\":\"1\"}}" \
            "}", issuerDid, credDefId];
}

- (NSString *)getGvtCredentialValuesJson {
    return [NSString stringWithFormat:@"{"\
            "\"sex\":{\"raw\":\"male\",\"encoded\":\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"},"\
            "\"name\":{\"raw\":\"Alex\",\"encoded\":\"1139481716457488690172217916278103335\"},"\
            "\"height\":{\"raw\":\"175\",\"encoded\":\"175\"},"\
            "\"age\":{\"raw\":\"28\",\"encoded\":\"28\"}"\
            "}"];
}

- (NSString *)getXyzCredentialValuesJson {
    return [NSString stringWithFormat:@"{"\
            "\"status\":{\"raw\":\"partial\",\"encoded\":\"123455672645217415414\"},"\
            "\"period\":{\"raw\":\"8\",\"encoded\":\"8\"}"\
            "}"];
}

- (NSString *)getGvt2CredentialValuesJson {
    return [NSString stringWithFormat:@"{"\
            "\"sex\":{\"raw\":\"male\",\"encoded\":\"3423574238417531735213261554781254821457218541265418245812541274517645176\"},"\
            "\"name\":{\"raw\":\"Alexander\",\"encoded\":\"245372858126541862352154819291254612\"},"\
            "\"height\":{\"raw\":\"170\",\"encoded\":\"170\"},"\
            "\"age\":{\"raw\":\"28\",\"encoded\":\"28\"}"\
            "}"];
}

- (NSString *)credentialId1 {
    return @"credentialID1";
}

- (NSString *)credentialId2 {
    return @"credentialID2";
}

// NOTE: currently it only used in Ledger tests
- (NSString *)getGvtCredentialDef {
    return @"{"
            "\"ref\":1,"
            "\"signature_type\":\"CL\","
            "\"data\":{"
            "\"primary\":{"
            "\"n\":\"83469852984476956871633111285697420678256060723156580163068122759469567425381600849138438902552107548539766861666590365174848381535291010418041757276710240953030842046122202402016906205924972182252295487319094577329593677544393592632224714613427822130473474379696616183721440743475053734247824037725487533789856061706740833324717788602268746116297029721621398888459529131593826880823126900285858832457134377949183677639585442886904844793608783831753240185678448312284269486845497720949217396146132958861735347072722092449280372574205841746312833280031873247525372459800132930201998084029506922484661426185450002143461\","
            "\"s\":\"36598527821865478336201608644402887021319976830281254144922838415193047189326184120876650311572665920640111967758238425066565864958446810892401358531545590342401290056836149703549220109981509774843525259400920352082531560361277411808530872594109525982462491998670199872903823657869742599086495624835178373073050767142081484206776345277546531080450529061958937980460303537107061046725579009809137197541389237618812289642185603461102513124991949835948586623327143696280240600789928383168220919049794681181861776889681393339729944540218566460627715413465709316412838042632482652979005394086058441511591756153781159121227\","
            "\"rms\":\"23836382972046033209463023456985914927629254782286444334728987813724281765327660893337383450653748691133061782449580026414785334582859397732571499366000805280757877601804441568263743400086744823885203441870748890135445454347495577599234477813361254101857848089907496868136222777024967328411073984887059985103475100012787049491016895625234124538894645853939428009610771524580099452739392988318945585946758355611531582519514003714424216836334706370901576611410508113637778751976890941210538254418937285168453791223070083264852447713991114719905171445881819334587600693321106919667204512182250084258986170095774914769107\","
            "\"r\":{"
            "\"age\":\"15428480888651268593621235736458685943389726269437020388313417035842991073151072061010468945249435098482625869236498750525662874597991333642865524104221652457788998109101749530884821300954337535472137069380551054204373136155978715752232238326100335828797868667735730830741789880726890058203015780792583471770404478023662994191588489722949795849990796063953164194432710764145637578113087142419634074378464118254848566088943085760634805903735300398689750649630456000759025366784986694635635510206166144055869823907120081668956271923743188342071093889666111639924270726451727101864752767708690529389259470017692442002767\","
            "\"name\":\"74008461204977705404956807338714891429397387365673402608947856456696416827848931951447004905350314563364442667940680669672331872875260077257474781261367591510351742401708951175978700805098470304211391452678992053755170054677498844656517106987419550598382601263743442309896367374279461481792679346472671426558385003925845431219156475505665973289508268634194964491418902859845301351562575713510002838692825728016254605821829245646855474149449192539144107522081712005891593405826343897070114168186645885993480245755494685105636168333649181939830898090651120926156152753918086493335382129839850609934233944397884745134858\","
            "\"sex\":\"40646934914193532541511585946883243600955533193734077080129022860038019728021796610599139377923881754708640252789475144625086755150150612623804964347576907276705600241268266301487516824189453869193926251791711672689649199860304727280764676403810510047397326018392955950249975529169980045664874433828266623495515931483137318724210483205730962036282580749206735450480976847188040030165278917936054139926609849181885654646269083459580415131952938813839182742590617440550773580790446467896469090164142755499827316294406540664375065617280568303837662431668218593808092907551353093044984225946834165309588512359379032847125\","
            "\"height\":\"60077310006190228182950362501472785016827894350517184186566050586806482282196022414888288252599211919680339352529750982779980002923071031258837648242708410943080288964834346858544959217874890558006056551949543916094446891954292824146212277550956558692224016968153138097595802008943263818064605343108607131298420107448075252583276684858815371561492996587478784667827675142382692061950832554910690663724101360454494298013020706080246986445114235542283015624510836206522238238728405826712730615187235709554561445384409566940622412591208469650855059870671771721035756205878334354791247751663679130847366069215369484993653\"},"
            "\"rctxt\":\"36378575722516953828830668112614685244571602424420162720244033008706985740860180373728219883172046821464173434592331868657297711725743060654773725561634332269874655603697872022630999786617840856366807034806938874090561725454026277048301648000835861491030368037108847175790943895107305383779598585532854170748970999977490244057635358075906501562932970296830906796719844887269636297064678777638050708353254894155336111384638276041851818109156194135995350001255928374102089400812445206030019103440866343736761861049159446083221399575945128480681798837648578166327164995640582517916904912672875184928940552983440410245037\","
            "\"z\":\"65210811645114955910002482704691499297899796787292244564644467629838455625296674951468505972574512639263601600908664306008863647466643899294681985964775001929521624341158696866597713112430928402519124073453804861185882073381514901830347278653016300430179820703804228663001232136885036042101307423527913402600370741689559698469878576457899715687929448757963179899698951620405467976414716277505767979494596626867505828267832223147104774684678295400387894506425769550011471322118172087199519094477785389750318762521728398390891214426117908390767403438354009767291938975195751081032073309083309746656253788033721103313036\""
            "}"
            "}"
            "}";
}

// MARK: issuer credential


- (NSError *)issuerCreateSchemaForIssuerDID:(NSString *)issuerDid
                                       name:(NSString *)name
                                    version:(NSString *)version
                                      attrs:(NSString *)attrs
                                   schemaId:(NSString **)schemaId
                                 schemaJson:(NSString **)schemaJson {
    __block NSError *err = nil;
    __block NSString *outSchemaId = nil;
    __block NSString *outSchemaJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [IndyAnoncreds issuerCreateSchemaForIssuerDID:issuerDid
                                             name:name
                                          version:version
                                            attrs:attrs
                                       completion:^(NSError *error, NSString *schemaID, NSString *schemaJSON) {
                                           err = error;
                                           outSchemaId = schemaID;
                                           outSchemaJson = schemaJSON;

                                           [completionExpectation fulfill];
                                       }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (schemaId) {*schemaId = outSchemaId;}
    if (schemaJson) {*schemaJson = outSchemaJson;}

    return err;
}

- (NSError *)issuerCreateCredentialDefinitionWithWalletHandle:(IndyHandle)walletHandle
                                                    issuerDid:(NSString *)issuerDid
                                                   schemaJson:(NSString *)schemaJson
                                                          tag:(NSString *)tag
                                                         type:(NSString *)type
                                                   configJson:(NSString *)configJson
                                              credentialDefId:(NSString **)credentialDefId
                                            credentialDefJson:(NSString **)credentialDefJson {
    __block NSError *err = nil;
    __block NSString *outCredentialDefId = nil;
    __block NSString *outCredentialDefJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [IndyAnoncreds issuerCreateAndStoreCredentialDefForIssuerDID:issuerDid
                                                      schemaJSON:schemaJson
                                                             tag:tag
                                                            type:type
                                                      configJSON:configJson
                                                    walletHandle:walletHandle
                                                      completion:^(NSError *error, NSString *credentialDefID, NSString *credentialDefJSON) {
                                                          err = error;
                                                          outCredentialDefId = credentialDefID;
                                                          outCredentialDefJson = credentialDefJSON;

                                                          [completionExpectation fulfill];
                                                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (credentialDefId) {*credentialDefId = outCredentialDefId;}
    if (credentialDefJson) {*credentialDefJson = outCredentialDefJson;}

    return err;
}

- (NSError *)issuerCreateAndStoreRevocRegForWithWalletHandle:(IndyHandle)walletHandle
                                                   issuerDid:(NSString *)issuerDID
                                                        type:(NSString *)type
                                                         tag:(NSString *)tag
                                                   credDefId:(NSString *)credDefId
                                                  configJSON:(NSString *)configJSON
                                             tailsWriterType:(NSString *)tailsWriterType
                                           tailsWriterConfig:(NSString *)tailsWriterConfig
                                                  revocRegId:(NSString **)revocRegId
                                             revocRegDefJson:(NSString **)revocRegDefJson
                                           revocRegEntryJson:(NSString **)revocRegEntryJson {
    __block NSError *err = nil;
    __block NSString *outRevRegId = nil;
    __block NSString *outRevocRegDefJson = nil;
    __block NSString *outRevocRegEntryJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [IndyAnoncreds issuerCreateAndStoreRevocRegForIssuerDid:issuerDID
                                                       type:type
                                                        tag:tag
                                                  credDefId:credDefId
                                                 configJSON:configJSON
                                            tailsWriterType:tailsWriterType
                                          tailsWriterConfig:tailsWriterConfig
                                               walletHandle:walletHandle
                                                 completion:^(NSError *error, NSString *revocRegID, NSString *revocRegDefJSON, NSString *revocRegEntryJSON) {
                                                     err = error;
                                                     outRevRegId = revocRegID;
                                                     outRevocRegDefJson = revocRegDefJSON;
                                                     outRevocRegEntryJson = revocRegEntryJSON;

                                                     [completionExpectation fulfill];
                                                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revocRegId) {*revocRegId = outRevRegId;}
    if (revocRegDefJson) {*revocRegDefJson = outRevocRegDefJson;}
    if (revocRegEntryJson) {*revocRegEntryJson = outRevocRegEntryJson;}

    return err;
}

- (NSError *)issuerRevokeCredentialForWalletHandle:(IndyHandle)walletHandle
                                          RevRegId:(NSString *)revRegId
                                 tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                    userRevocIndex:(NSNumber *)userRevocIndex
                                 revocRegDeltaJson:(NSString **)revocRegDeltaJson {
    __block NSError *err = nil;
    __block NSString *outRevocRegDeltaJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds issuerRevokeCredentialForRevRegId:revRegId
                                   tailsReaderHandle:tailsReaderHandle
                                      userRevocIndex:userRevocIndex
                                        walletHandle:walletHandle
                                          completion:^(NSError *error, NSString *revocRegDeltaJSON) {
                                              err = error;
                                              outRevocRegDeltaJson = revocRegDeltaJSON;

                                              [completionExpectation fulfill];
                                          }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revocRegDeltaJson) {*revocRegDeltaJson = outRevocRegDeltaJson;}

    return err;
}

- (NSError *)issuerRecoverCredentialForWalletHandle:(IndyHandle)walletHandle
                                           RevRegId:(NSString *)revRegId
                                  tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                     userRevocIndex:(NSNumber *)userRevocIndex
                                  revocRegDeltaJson:(NSString **)revocRegDeltaJson {
    __block NSError *err = nil;
    __block NSString *outRevocRegDeltaJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds issuerRecoverCredentialForRevRegId:revRegId
                                    tailsReaderHandle:tailsReaderHandle
                                       userRevocIndex:userRevocIndex
                                         walletHandle:walletHandle
                                           completion:^(NSError *error, NSString *revocRegDeltaJSON) {
                                               err = error;
                                               outRevocRegDeltaJson = revocRegDeltaJSON;

                                               [completionExpectation fulfill];
                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revocRegDeltaJson) {*revocRegDeltaJson = outRevocRegDeltaJson;}

    return err;
}


- (NSError *)issuerCreateCredentialOfferWithWalletHandle:(IndyHandle)walletHandle
                                         credentialDefId:(NSString *)credentialDefId
                                               issuerDid:(NSString *)issuerDid
                                               proverDid:(NSString *)proverDid
                                     credentialOfferJson:(NSString **)credentialOfferJson {
    __block NSError *err = nil;
    __block NSString *outCredentialOfferJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [IndyAnoncreds issuerCreateCredentialOfferForProverDID:proverDid
                                                 issuerDID:issuerDid
                                                 credDefId:credentialDefId
                                              walletHandle:walletHandle
                                                completion:^(NSError *error, NSString *credentialOfferJSON) {
                                                    err = error;
                                                    outCredentialOfferJson = credentialOfferJSON;

                                                    [completionExpectation fulfill];
                                                }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (credentialOfferJson) {*credentialOfferJson = outCredentialOfferJson;}

    return err;
}

- (NSError *)issuerCreateCredentialWithWalletHandle:(IndyHandle)walletHandle
                                  credentialReqJson:(NSString *)credentialReqJson
                               credentialValuesJson:(NSString *)credentialValuesJson
                                           revRegId:(NSString *)revRegId
                                  tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                     userRevocIndex:(NSNumber *)userRevocIndex
                                  outCredentialJson:(NSString **)xCredentialJson
                               outRevocRegDeltaJSON:(NSString **)revocRegDeltaJson {
    __block NSError *err = nil;
    __block NSString *outCredentialJson;
    __block NSString *outRevocRegDeltaJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds issuerCreateCredentialWithRequest:credentialReqJson
                                credentialValuesJSON:credentialValuesJson
                                            revRegId:revRegId
                                   tailsReaderHandle:tailsReaderHandle
                                      userRevocIndex:userRevocIndex
                                        walletHandle:walletHandle
                                          completion:^(NSError *error, NSString *revocRegDeltaJSON, NSString *credentialJSON) {
                                              err = error;
                                              outRevocRegDeltaJson = revocRegDeltaJSON;
                                              outCredentialJson = credentialJSON;
                                              [completionExpectation fulfill];
                                          }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (xCredentialJson) {*xCredentialJson = outCredentialJson;}
    if (revocRegDeltaJson) {*revocRegDeltaJson = outRevocRegDeltaJson;}
    return err;
}

- (NSError *)proverCreateMasterSecretNamed:(NSString *)masterSecretName
                              walletHandle:(IndyHandle)walletHandle {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverCreateMasterSecretNamed:masterSecretName
                                    walletHandle:walletHandle
                                      completion:^(NSError *error) {
                                          err = error;
                                          [completionExpectation fulfill];
                                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];
    return err;
}

- (NSError *)proverStoreCredentialOffer:(IndyHandle)walletHandle
                    credentialOfferJson:(NSString *)str {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverStoreCredentialOffer:str
                             WithWalletHandle:walletHandle
                                   completion:^(NSError *error) {
                                       err = error;
                                       [completionExpectation fulfill];
                                   }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;

}

- (NSError *)proverGetCredentialOffers:(IndyHandle)walletHandle
                            filterJson:(NSString *)filterJson
               outCredentialOffersJSON:(NSString **)outJson {
    __block NSString *json;
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverGetCredentialOffersWithFilter:filterJson
                                          walletHandle:walletHandle
                                            completion:^(NSError *error, NSString *credentialOffersJSON) {
                                                err = error;
                                                json = credentialOffersJSON;
                                                [completionExpectation fulfill];
                                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (outJson) {*outJson = json;}
    return err;
}

- (NSError *)proverCreateAndStoreCredentialReqWithDef:(NSString *)credentialDefJSON
                                            proverDid:(NSString *)proverDid
                                  credentialOfferJson:(NSString *)credentialOfferJSON
                                     masterSecretName:(NSString *)name
                                         walletHandle:(IndyHandle)walletHandle
                                 outCredentialReqJson:(NSString **)outJson {
    __block NSError *err = nil;
    __block NSString *json;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverCreateAndStoreCredentialReqWithCredentialDef:credentialDefJSON
                                                            proverDID:proverDid
                                                  credentialOfferJSON:credentialOfferJSON
                                                     masterSecretName:name
                                                         walletHandle:walletHandle
                                                           completion:^(NSError *error, NSString *credentialReqJSON) {
                                                               err = error;
                                                               json = credentialReqJSON;
                                                               [completionExpectation fulfill];
                                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (outJson) {*outJson = json;}
    return err;
}


- (NSError *)proverStoreCredentialWithWalletHandle:(IndyHandle)walletHandle
                                      credentialId:(NSString *)credentialId
                                   credentialsJson:(NSString *)credentialsJson
                                     revRegDefJSON:(NSString *)revRegDefJSON {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverStoreCredential:credentialsJson
                            credentialId:credentialId
                           revRegDefJSON:revRegDefJSON
                            walletHandle:walletHandle
                              completion:^(NSError *error) {
                                  XCTAssertEqual(err.code, Success, @"proverStoreCredential failed!");
                                  err = error;
                                  [completionExpectation fulfill];
                              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)proverGetCredentialsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                            proofRequestJson:(NSString *)proofRequestJson
                                          outCredentialsJson:(NSString **)outCredentialsJson {
    __block NSError *err = nil;
    __block NSString *outJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverGetCredentialsForProofReq:proofRequestJson
                                      walletHandle:walletHandle
                                        completion:^(NSError *error, NSString *credentialsJSON) {
                                            err = error;
                                            outJson = credentialsJSON;
                                            [completionExpectation fulfill];
                                        }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (outCredentialsJson) {*outCredentialsJson = outJson;}

    return err;
}

- (NSError *)proverGetCredentialsForWalletHandle:(IndyHandle)walletHandle
                                      filterJson:(NSString *)filterJson
                               outCredentilsJson:(NSString **)credentialsJson {
    __block NSError *err = nil;
    __block NSString *outJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverGetCredentialsWithFilter:filterJson
                                     walletHandle:walletHandle
                                       completion:^(NSError *error, NSString *credentialsJSON) {
                                           err = error;
                                           outJson = credentialsJSON;
                                           [completionExpectation fulfill];
                                       }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (credentialsJson) {*credentialsJson = outJson;}

    return err;
}

- (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
                                  proofReqJson:(NSString *)proofReqJson
                      requestedCredentialsJson:(NSString *)requestedCredentialsJson
                                   schemasJson:(NSString *)schemasJson
                              masterSecretName:(NSString *)masterSecreteName
                            credentialDefsJson:(NSString *)credentialDefsJson
                                revocInfosJSON:(NSString *)revocInfosJSON
                                  outProofJson:(NSString **)outProofJson {
    __block NSError *err = nil;
    __block NSString *outJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverCreateProofForRequest:proofReqJson
                      requestedCredentialsJSON:requestedCredentialsJson
                                   schemasJSON:schemasJson
                              masterSecretName:masterSecreteName
                            credentialDefsJSON:credentialDefsJson
                                revocInfosJSON:revocInfosJSON
                                  walletHandle:walletHandle
                                    completion:^(NSError *error, NSString *proofJSON) {
                                        err = error;
                                        outJson = proofJSON;

                                        [completionExpectation fulfill];
                                    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (outProofJson) {*outProofJson = outJson;}
    return err;
}

- (NSError *)createRevocationInfoForTimestamp:(NSNumber *)timestamp
                                revRegDefJSON:(NSString *)revRegDefJSON
                              revRegDeltaJSON:(NSString *)revRegDeltaJSON
                            tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                       revIdx:(NSNumber *)revIdx
                                  revInfoJson:(NSString **)revInfoJson {
    __block NSError *err = nil;
    __block NSString *outRevInfoJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds createRevocationInfoForTimestamp:timestamp
                                      revRegDefJSON:revRegDefJSON
                                    revRegDeltaJSON:revRegDeltaJSON
                                  tailsReaderHandle:tailsReaderHandle
                                             revIdx:revIdx
                                         completion:^(NSError *error, NSString *revInfoJSON) {
                                             err = error;
                                             outRevInfoJson = revInfoJSON;

                                             [completionExpectation fulfill];
                                         }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revInfoJson) {*revInfoJson = outRevInfoJson;}
    return err;
}

- (NSError *)updateRevocationInfoForTimestamp:(NSNumber *)timestamp
                                  revInfoJSON:(NSString *)revInfoJSON
                                revRegDefJSON:(NSString *)revRegDefJSON
                              revRegDeltaJSON:(NSString *)revRegDeltaJSON
                            tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                       revIdx:(NSNumber *)revIdx
                           updatedRevInfoJson:(NSString **)updatedRevInfoJson {
    __block NSError *err = nil;
    __block NSString *outUpdatedRevInfoJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds updateRevocationInfoForTimestamp:timestamp
                                        revInfoJSON:revInfoJSON
                                      revRegDefJSON:revRegDefJSON
                                    revRegDeltaJSON:revRegDeltaJSON
                                  tailsReaderHandle:tailsReaderHandle
                                             revIdx:revIdx
                                         completion:^(NSError *error, NSString *updatedRevInfoJSON) {
                                             err = error;
                                             outUpdatedRevInfoJson = updatedRevInfoJSON;

                                             [completionExpectation fulfill];
                                         }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (updatedRevInfoJson) {*updatedRevInfoJson = outUpdatedRevInfoJson;}
    return err;
}

- (NSError *)storeRevocationInfoForWalletHandle:(IndyHandle)walletHandle
                                             id:(NSString *)id
                                    revInfoJSON:(NSString *)revInfoJSON {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds storeRevocationInfoForId:id
                                revInfoJSON:revInfoJSON
                               walletHandle:walletHandle
                                 completion:^(NSError *error) {
                                     err = error;
                                     [completionExpectation fulfill];
                                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;
}

- (NSError *)getRevocationInfoForWalletHandle:(IndyHandle)walletHandle
                                           id:(NSString *)id
                                    timestamp:(NSNumber *)timestamp
                                  revInfoJson:(NSString **)revInfoJson {
    __block NSError *err = nil;
    __block NSString *outRevInfoJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds getRevocationInfoForId:id
                                timestamp:timestamp
                             walletHandle:walletHandle
                               completion:^(NSError *error, NSString *revInfoJSON) {
                                   err = error;
                                   outRevInfoJson = revInfoJSON;

                                   [completionExpectation fulfill];
                               }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revInfoJson) {*revInfoJson = outRevInfoJson;}
    return err;
}

- (NSError *)verifierVerifyProof:(NSString *)proofRequestJson
                       proofJson:(NSString *)proofJson
                     schemasJson:(NSString *)schemasJson
              credentialDefsJson:(NSString *)credentialDefsJson
                revocRegDefsJSON:(NSString *)revocRegDefsJSON
                   revocRegsJson:(NSString *)revocRegsJson
                        outValid:(BOOL *)isValid {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds verifierVerifyProofRequest:proofRequestJson
                                    proofJSON:proofJson
                                  schemasJSON:schemasJson
                           credentialDefsJSON:credentialDefsJson
                             revocRegDefsJSON:revocRegDefsJSON
                                revocRegsJSON:revocRegsJson
                                   completion:^(NSError *error, BOOL valid) {
                                       err = error;
                                       if (isValid) {
                                           *isValid = valid;
                                       }
                                       [completionExpectation fulfill];
                                   }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];
    return err;

}

- (NSError *)initializeCommonWalletAndReturnHandle:(IndyHandle *)walletHandle
                                 credentialDefJson:(NSString **)credentialDefJson
                               credentialOfferJson:(NSString **)credentialOfferJson
                                 credentialReqJson:(NSString **)credentialReqJson
                                    credentialJson:(NSString **)credentialJson {
    if (self.isCommonWalletCreated) {
        if (walletHandle) {
            *walletHandle = self.walletHandle;
        }
        if (credentialDefJson) {
            *credentialDefJson = _singletoneCredentialdefJson;
        }
        if (credentialOfferJson) {
            *credentialOfferJson = _singletoneCredentialofferJson;
        }
        if (credentialReqJson) {
            *credentialReqJson = _singletoneCredentialreqJson;
        }
        if (credentialJson) {
            *credentialJson = _singletoneCredentialJson;
        }
        return [NSError errorWithDomain:@""
                                   code:Success
                               userInfo:nil];
    }

    [TestUtils cleanupStorage];

    NSError *ret;

    // 1. Create and open wallet
    self.walletHandle = 0;
    IndyHandle tempWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                                  xtype:nil
                                                                 handle:&tempWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed");
    if (ret.code != Success) {return ret;}

    //2. Create GVT Schema
    NSString *gvtSchemaId;
    NSString *gvtSchemaJson;
    ret = [self issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                          name:[TestUtils gvtSchemaName]
                                       version:[TestUtils schemaVersion]
                                         attrs:[TestUtils gvtSchemaAttrs]
                                      schemaId:&gvtSchemaId
                                    schemaJson:&gvtSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([gvtSchemaId isValid], @"invalid gvtSchemaId: %@", gvtSchemaId);
    XCTAssertTrue([gvtSchemaJson isValid], @"invalid gvtSchemaJson: %@", gvtSchemaJson);

    //3. Create XYZ Schema
    NSString *xyzSchemaId;
    NSString *xyzSchemaJson;
    ret = [self issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                          name:[TestUtils xyzSchemaName]
                                       version:[TestUtils schemaVersion]
                                         attrs:[TestUtils xyzSchemaAttrs]
                                      schemaId:&xyzSchemaId
                                    schemaJson:&xyzSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([xyzSchemaId isValid], @"invalid xyzSchemaId: %@", xyzSchemaId);
    XCTAssertTrue([xyzSchemaJson isValid], @"invalid xyzSchemaJson: %@", xyzSchemaJson);

    //4. Issuer1 create GVT CredentialDef
    NSString *issuer1GvtCredentialDefId;
    NSString *issuer1GvtCredentialDefJson;
    ret = [self issuerCreateCredentialDefinitionWithWalletHandle:tempWalletHandle
                                                       issuerDid:[TestUtils issuerDid]
                                                      schemaJson:gvtSchemaJson
                                                             tag:[TestUtils tag]
                                                            type:nil
                                                      configJson:[self defaultCredentialDefConfig]
                                                 credentialDefId:&issuer1GvtCredentialDefId
                                               credentialDefJson:&issuer1GvtCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    XCTAssertTrue([issuer1GvtCredentialDefId isValid], @"invalid issuer1GvtCredentialDefId: %@", issuer1GvtCredentialDefId);
    XCTAssertTrue([issuer1GvtCredentialDefJson isValid], @"invalid issuer1GvtCredentialDefJson: %@", issuer1GvtCredentialDefJson);

    //5. Issuer1 create XYZ CredentialDef
    NSString *issuer1XyzCredentialDefId;
    NSString *issuer1XyzCredentialDefJson;
    ret = [self issuerCreateCredentialDefinitionWithWalletHandle:tempWalletHandle
                                                       issuerDid:[TestUtils issuerDid]
                                                      schemaJson:xyzSchemaJson
                                                             tag:[TestUtils tag]
                                                            type:nil
                                                      configJson:[self defaultCredentialDefConfig]
                                                 credentialDefId:&issuer1XyzCredentialDefId
                                               credentialDefJson:&issuer1XyzCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    XCTAssertTrue([issuer1XyzCredentialDefId isValid], @"invalid issuer1XyzCredentialDefId: %@", issuer1XyzCredentialDefId);
    XCTAssertTrue([issuer1XyzCredentialDefJson isValid], @"invalid issuer1XyzCredentialDefJson: %@", issuer1XyzCredentialDefJson);

    //6. Issuer2 create XYZ CredentialDef
    NSString *issuer2GvtCredentialDefId;
    NSString *issuer2GvtCredentialDefJson;
    ret = [self issuerCreateCredentialDefinitionWithWalletHandle:tempWalletHandle
                                                       issuerDid:[TestUtils issuer2Did]
                                                      schemaJson:gvtSchemaJson
                                                             tag:[TestUtils tag]
                                                            type:nil
                                                      configJson:[self defaultCredentialDefConfig]
                                                 credentialDefId:&issuer2GvtCredentialDefId
                                               credentialDefJson:&issuer2GvtCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    XCTAssertTrue([issuer2GvtCredentialDefId isValid], @"invalid issuer2GvtCredentialDefId: %@", issuer2GvtCredentialDefId);
    XCTAssertTrue([issuer2GvtCredentialDefJson isValid], @"invalid issuer2GvtCredentialDefJson: %@", issuer2GvtCredentialDefJson);


    // 7. Issuer1 create GVT Credential Offer
    NSString *issuer1GvtCredentialOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferWithWalletHandle:tempWalletHandle
                                                                       credentialDefId:issuer1GvtCredentialDefId
                                                                             issuerDid:[TestUtils issuerDid]
                                                                             proverDid:[TestUtils proverDid]
                                                                   credentialOfferJson:&issuer1GvtCredentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //8. Prover store Issuer1 GVT Credential Offer
    ret = [self proverStoreCredentialOffer:tempWalletHandle credentialOfferJson:issuer1GvtCredentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredentialOffer failed for issuer1GvtCredentialOfferJSON");

    //9. Issuer1 create XYZ Credential Offer
    NSString *issuer1XyzCredentialOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferWithWalletHandle:tempWalletHandle
                                                                       credentialDefId:issuer1XyzCredentialDefId
                                                                             issuerDid:[TestUtils issuerDid]
                                                                             proverDid:[TestUtils proverDid]
                                                                   credentialOfferJson:&issuer1XyzCredentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //10. Prover store Issuer1 XYZ Credential Offer
    ret = [self proverStoreCredentialOffer:tempWalletHandle credentialOfferJson:issuer1XyzCredentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredentialOffer failed for issuer1XyzCredentialOfferJSON");

    // 11. Issuer2 create GVT Credential Offer
    NSString *issuer2GvtCredentialOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferWithWalletHandle:tempWalletHandle
                                                                       credentialDefId:issuer2GvtCredentialDefId
                                                                             issuerDid:[TestUtils issuer2Did]
                                                                             proverDid:[TestUtils proverDid]
                                                                   credentialOfferJson:&issuer2GvtCredentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //12. Prover store Issuer2 GVT Credential Offer
    ret = [self proverStoreCredentialOffer:tempWalletHandle credentialOfferJson:issuer2GvtCredentialOfferJSON];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredentialOffer failed for issuer2GvtCredentialOfferJSON");


    //13. Create MasterSecret
    ret = [self proverCreateMasterSecretNamed:[TestUtils commonMasterSecretName]
                                 walletHandle:tempWalletHandle];
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret failed");

    //14. Prover create and Store Credential Request for Issuer1 GVT Credential
    NSString *issuer1GvtCredentialRequest;
    ret = [self proverCreateAndStoreCredentialReqWithDef:issuer1GvtCredentialDefJson
                                               proverDid:[TestUtils proverDid]
                                     credentialOfferJson:issuer1GvtCredentialOfferJSON
                                        masterSecretName:[TestUtils commonMasterSecretName]
                                            walletHandle:tempWalletHandle
                                    outCredentialReqJson:&issuer1GvtCredentialRequest];

    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreCredentialReq failed for issuer1GvtCredentialRequest");
    XCTAssertTrue([issuer1GvtCredentialRequest isValid], @"invalid credential request: %@", issuer1GvtCredentialRequest);

    if (ret.code != Success) {return ret;}

    //15. Issuer1 create  GVT Credential
    NSString *issuer1GvtCredential;
    ret = [self issuerCreateCredentialWithWalletHandle:tempWalletHandle
                                     credentialReqJson:issuer1GvtCredentialRequest
                                  credentialValuesJson:[self getGvtCredentialValuesJson]
                                              revRegId:nil
                                     tailsReaderHandle:nil
                                        userRevocIndex:nil
                                     outCredentialJson:&issuer1GvtCredential
                                  outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialWithWalletHandle failed");
    XCTAssertTrue([issuer1GvtCredential isValid], @"invalid issuer1GvtCredential: %@", issuer1GvtCredential);

    //16. Prover store GVT credential from issuer1
    ret = [self proverStoreCredentialWithWalletHandle:tempWalletHandle
                                         credentialId:[self credentialId1]
                                      credentialsJson:issuer1GvtCredential
                                        revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredentialWithWalletHandle failed");

    //17. Prover create and Store Credential Request for Issuer1 XYZ Credential
    NSString *issuer1XyzCredentialRequest;
    ret = [self proverCreateAndStoreCredentialReqWithDef:issuer1XyzCredentialDefJson
                                               proverDid:[TestUtils proverDid]
                                     credentialOfferJson:issuer1XyzCredentialOfferJSON
                                        masterSecretName:[TestUtils commonMasterSecretName]
                                            walletHandle:tempWalletHandle
                                    outCredentialReqJson:&issuer1XyzCredentialRequest];

    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreCredentialReq failed for issuer1XyzCredentialRequest");
    XCTAssertTrue([issuer1XyzCredentialRequest isValid], @"invalid credential request: %@", issuer1XyzCredentialRequest);

    if (ret.code != Success) {return ret;}

    //18. Issuer1 create XYZ Credential
    NSString *issuer1XyzCredential;
    ret = [self issuerCreateCredentialWithWalletHandle:tempWalletHandle
                                     credentialReqJson:issuer1XyzCredentialRequest
                                  credentialValuesJson:[self getXyzCredentialValuesJson]
                                              revRegId:nil
                                     tailsReaderHandle:nil
                                        userRevocIndex:nil
                                     outCredentialJson:&issuer1XyzCredential
                                  outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialWithWalletHandle failed");
    XCTAssertTrue([issuer1XyzCredential isValid], @"invalid issuer1XyzCredential: %@", issuer1XyzCredential);

    //19. Prover store XYZ credential from issuer1
    ret = [self proverStoreCredentialWithWalletHandle:tempWalletHandle
                                         credentialId:[self credentialId2]
                                      credentialsJson:issuer1XyzCredential
                                        revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredentialWithWalletHandle failed");


    //20. Prover create and Store Credential Request for Issuer2 GVT Credential
    NSString *issuer2GvtCredentialRequest;
    ret = [self proverCreateAndStoreCredentialReqWithDef:issuer2GvtCredentialDefJson
                                               proverDid:[TestUtils proverDid]
                                     credentialOfferJson:issuer2GvtCredentialOfferJSON
                                        masterSecretName:[TestUtils commonMasterSecretName]
                                            walletHandle:tempWalletHandle
                                    outCredentialReqJson:&issuer2GvtCredentialRequest];

    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreCredentialReq failed for issuer2GvtCredentialRequest");
    XCTAssertTrue([issuer2GvtCredentialRequest isValid], @"invalid credential request: %@", issuer2GvtCredentialRequest);

    if (ret.code != Success) {return ret;}

    //21. Issuer2 create GVT Credential
    NSString *issuer2GvtCredential;
    ret = [self issuerCreateCredentialWithWalletHandle:tempWalletHandle
                                     credentialReqJson:issuer2GvtCredentialRequest
                                  credentialValuesJson:[self getGvt2CredentialValuesJson]
                                              revRegId:nil
                                     tailsReaderHandle:nil
                                        userRevocIndex:nil
                                     outCredentialJson:&issuer2GvtCredential
                                  outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialWithWalletHandle failed");
    XCTAssertTrue([issuer2GvtCredential isValid], @"invalid issuer2GvtCredential: %@", issuer2GvtCredential);

    //22. Prover store GVT credential from Issuer2
    ret = [self proverStoreCredentialWithWalletHandle:tempWalletHandle
                                         credentialId:@"CredentialId3"
                                      credentialsJson:issuer2GvtCredential
                                        revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreCredentialWithWalletHandle failed");

    if (walletHandle) {*walletHandle = tempWalletHandle;}
    if (credentialDefJson) {*credentialDefJson = issuer1GvtCredentialDefJson;}
    if (credentialOfferJson) {*credentialOfferJson = issuer1GvtCredentialOfferJSON;}
    if (credentialReqJson) {*credentialReqJson = issuer1GvtCredentialRequest;}
    if (credentialJson) {*credentialJson = issuer1GvtCredential;}
    self.walletHandle = tempWalletHandle;
    _singletoneCredentialdefJson = issuer1GvtCredentialDefJson;
    _singletoneCredentialofferJson = issuer1GvtCredentialOfferJSON;
    _singletoneCredentialreqJson = issuer1GvtCredentialRequest;
    _singletoneCredentialJson = issuer1GvtCredential;

    self.isCommonWalletCreated = true;

    return ret;
}

@end
