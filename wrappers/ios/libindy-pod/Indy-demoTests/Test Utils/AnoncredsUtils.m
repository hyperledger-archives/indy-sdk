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
@property(strong) NSString *singletoneClaimdefJson;
@property(strong) NSString *singletoneClaimofferJson;
@property(strong) NSString *singletoneClaimreqJson;
@property(strong) NSString *singletoneClaimJson;

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

- (NSString *)buildSchemaId:(NSString *)identifier
                       name:(NSString *)name
                    version:(NSString *)version {
    NSString *delimiter = @":";
    return [NSString stringWithFormat:@"%@%@%@%@%@%@%@", identifier, delimiter, @"1", delimiter, name, delimiter, version];
}

- (NSString *)buildClaimDefId:(NSString *)identifier
                     schemaId:(NSString *)schemaId
                         type:(NSString *)type
                          tag:(NSString *)tag {
    NSString *delimiter = @":";
    return [NSString stringWithFormat:@"%@%@%@%@%@%@%@%@%@", identifier, delimiter, @"2", delimiter, schemaId, delimiter, type, delimiter, tag];
}

- (NSString *)defaultClaimDefConfig {
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

- (NSString *)getClaimOfferJson:(NSString *)issuerDid
                      credDefId:(NSString *)credDefId {
    return [NSString stringWithFormat:@"{"\
            "\"issuer_did\":\"%@\"," \
            "\"cred_def_id\":\"%@\"," \
            "\"nonce\":\"12345678\"," \
            "\"key_correctness_proof\": {\"c\":\"1\",\"xz_cap\":\"1\",\"xr_cap\":{\"name\":\"1\"}}" \
            "}", issuerDid, credDefId];
}

- (NSString *)getGvtClaimValuesJson {
    return [NSString stringWithFormat:@"{"\
            "\"sex\":{\"raw\":\"male\",\"encoded\":\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"},"\
            "\"name\":{\"raw\":\"Alex\",\"encoded\":\"1139481716457488690172217916278103335\"},"\
            "\"height\":{\"raw\":\"175\",\"encoded\":\"175\"},"\
            "\"age\":{\"raw\":\"28\",\"encoded\":\"28\"}"\
            "}"];
}

- (NSString *)getXyzClaimValuesJson {
    return [NSString stringWithFormat:@"{"\
            "\"status\":{\"raw\":\"partial\",\"encoded\":\"123455672645217415414\"},"\
            "\"period\":{\"raw\":\"8\",\"encoded\":\"8\"}"\
            "}"];
}

- (NSString *)getGvt2ClaimValuesJson {
    return [NSString stringWithFormat:@"{"\
            "\"sex\":{\"raw\":\"male\",\"encoded\":\"3423574238417531735213261554781254821457218541265418245812541274517645176\"},"\
            "\"name\":{\"raw\":\"Alexander\",\"encoded\":\"245372858126541862352154819291254612\"},"\
            "\"height\":{\"raw\":\"170\",\"encoded\":\"170\"},"\
            "\"age\":{\"raw\":\"28\",\"encoded\":\"28\"}"\
            "}"];
}

- (NSString *)getGvtClaimDef {
    return [NSString stringWithFormat:@"{"
                                              "\"id\":\"%@\","
                                              "\"schemaId\":\"%@\","
                                              "\"type\":\"CL\","
                                              "\"tag\":\"TAG1\","
                                              "\"value\":{"
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
                                              "}", [self getIssuer1GvtCredDefId], [self getGvtSchemaId]];
}

// MARK: issuer claim
- (NSError *)issuerCreateClaimWithWalletHandle:(IndyHandle)walletHandle
                                  claimReqJson:(NSString *)claimReqJson
                               claimValuesJson:(NSString *)claimValuesJson
                                      revRegId:(NSString *)revRegId
                             tailsReaderHandle:(NSNumber *)tailsReaderHandle
                                userRevocIndex:(NSNumber *)userRevocIndex
                                  outClaimJson:(NSString **)xClaimJson
                          outRevocRegDeltaJSON:(NSString **)revocRegDeltaJson {
    __block NSError *err = nil;
    __block NSString *outClaimJson;
    __block NSString *outRevocRegDeltaJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds issuerCreateClaimWithRequest:claimReqJson
                                claimValuesJSON:claimValuesJson
                                       revRegId:revRegId
                              tailsReaderHandle:tailsReaderHandle
                                 userRevocIndex:userRevocIndex
                                   walletHandle:walletHandle
                                     completion:^(NSError *error, NSString *revocRegDeltaJSON, NSString *claimJSON) {
                                         err = error;
                                         outRevocRegDeltaJson = revocRegDeltaJSON;
                                         outClaimJson = claimJSON;
                                         [completionExpectation fulfill];
                                     }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (xClaimJson) {*xClaimJson = outClaimJson;}
    if (revocRegDeltaJson) {*revocRegDeltaJson = outRevocRegDeltaJson;}
    return err;
}

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

- (NSError *)issuerCreateClaimDefinifionWithWalletHandle:(IndyHandle)walletHandle
                                               issuerDid:(NSString *)issuerDid
                                              schemaJson:(NSString *)schemaJson
                                                     tag:(NSString *)tag
                                                    type:(NSString *)type
                                              configJson:(NSString *)configJson
                                              claimDefId:(NSString **)claimDefId
                                            claimDefJson:(NSString **)claimDefJson {
    __block NSError *err = nil;
    __block NSString *outClaimDefId = nil;
    __block NSString *outClaimDefJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [IndyAnoncreds issuerCreateAndStoreClaimDefForIssuerDID:issuerDid
                                                 schemaJSON:schemaJson
                                                        tag:tag
                                                       type:type
                                                 configJSON:configJson
                                               walletHandle:walletHandle
                                                 completion:^(NSError *error, NSString *claimDefID, NSString *claimDefJSON) {
                                                     err = error;
                                                     outClaimDefId = claimDefID;
                                                     outClaimDefJson = claimDefJSON;

                                                     [completionExpectation fulfill];
                                                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (claimDefId) {*claimDefId = outClaimDefId;}
    if (claimDefJson) {*claimDefJson = outClaimDefJson;}

    return err;
}

- (NSError *)issuerCreateClaimOfferWithWalletHandle:(IndyHandle)walletHandle
                                         claimDefId:(NSString *)claimDefId
                                          issuerDid:(NSString *)issuerDid
                                          proverDid:(NSString *)proverDid
                                     claimOfferJson:(NSString **)claimOfferJson {
    __block NSError *err = nil;
    __block NSString *outClaimOfferJson = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];


    [IndyAnoncreds issuerCreateClaimOfferForProverDID:proverDid
                                            issuerDID:issuerDid
                                            credDefId:claimDefId
                                         walletHandle:walletHandle
                                           completion:^(NSError *error, NSString *claimOfferJSON) {
                                               err = error;
                                               outClaimOfferJson = claimOfferJSON;

                                               [completionExpectation fulfill];
                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (claimOfferJson) {*claimOfferJson = outClaimOfferJson;}

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

- (NSError *)proverStoreClaimOffer:(IndyHandle)walletHandle
                    claimOfferJson:(NSString *)str {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverStoreClaimOffer:str
                        WithWalletHandle:walletHandle
                              completion:^(NSError *error) {
                                  err = error;
                                  [completionExpectation fulfill];
                              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    return err;

}

- (NSError *)proverGetClaimOffers:(IndyHandle)walletHandle
                       filterJson:(NSString *)filterJson
               outClaimOffersJSON:(NSString **)outJson {
    __block NSString *json;
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverGetClaimOffersWithFilter:filterJson
                                     walletHandle:walletHandle
                                       completion:^(NSError *error, NSString *claimOffersJSON) {
                                           err = error;
                                           json = claimOffersJSON;
                                           [completionExpectation fulfill];
                                       }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (outJson) {*outJson = json;}
    return err;
}

- (NSError *)proverCreateAndStoreClaimReqWithDef:(NSString *)claimDefJSON
                                       proverDid:(NSString *)proverDid
                                  claimOfferJson:(NSString *)claimOfferJSON
                                masterSecretName:(NSString *)name
                                    walletHandle:(IndyHandle)walletHandle
                                 outClaimReqJson:(NSString **)outJson {
    __block NSError *err = nil;
    __block NSString *json;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverCreateAndStoreClaimReqWithClaimDef:claimDefJSON
                                                  proverDID:proverDid
                                             claimOfferJSON:claimOfferJSON
                                           masterSecretName:name
                                               walletHandle:walletHandle
                                                 completion:^(NSError *error, NSString *claimReqJSON) {
                                                     err = error;
                                                     json = claimReqJSON;
                                                     [completionExpectation fulfill];
                                                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (outJson) {*outJson = json;}
    return err;
}


- (NSError *)proverStoreClaimWithWalletHandle:(IndyHandle)walletHandle
                                      claimId:(NSString *)claimId
                                   claimsJson:(NSString *)claimsJson
                                revRegDefJSON:(NSString *)revRegDefJSON {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverStoreClaim:claimsJson
                            claimId:claimId
                      revRegDefJSON:revRegDefJSON
                       walletHandle:walletHandle
                         completion:^(NSError *error) {
                             XCTAssertEqual(err.code, Success, @"proverStoreClaim failed!");
                             err = error;
                             [completionExpectation fulfill];
                         }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    return err;
}

- (NSError *)proverGetClaimsForProofReqWithWalletHandle:(IndyHandle)walletHandle
                                       proofRequestJson:(NSString *)proofRequestJson
                                          outClaimsJson:(NSString **)outClaimsJson {
    __block NSError *err = nil;
    __block NSString *outJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverGetClaimsForProofReq:proofRequestJson
                                 walletHandle:walletHandle
                                   completion:^(NSError *error, NSString *claimsJSON) {
                                       err = error;
                                       outJson = claimsJSON;
                                       [completionExpectation fulfill];
                                   }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (outClaimsJson) {*outClaimsJson = outJson;}

    return err;
}

- (NSError *)proverGetClaimsForWalletHandle:(IndyHandle)walletHandle
                                 filterJson:(NSString *)filterJson
                              outClaimsJson:(NSString **)claimsJson {
    __block NSError *err = nil;
    __block NSString *outJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverGetClaimsWithFilter:filterJson
                                walletHandle:walletHandle
                                  completion:^(NSError *error, NSString *claimsJSON) {
                                      err = error;
                                      outJson = claimsJSON;
                                      [completionExpectation fulfill];
                                  }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (claimsJson) {*claimsJson = outJson;}

    return err;
}

- (NSError *)proverCreateProofWithWalletHandle:(IndyHandle)walletHandle
                                  proofReqJson:(NSString *)proofReqJson
                           requestedClaimsJson:(NSString *)requestedClaimsJson
                                   schemasJson:(NSString *)schemasJson
                              masterSecretName:(NSString *)masterSecreteName
                                 claimDefsJson:(NSString *)claimDefsJson
                                revocInfosJSON:(NSString *)revocInfosJSON
                                  outProofJson:(NSString **)outProofJson {
    __block NSError *err = nil;
    __block NSString *outJson;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds proverCreateProofForRequest:proofReqJson
                           requestedClaimsJSON:requestedClaimsJson
                                   schemasJSON:schemasJson
                              masterSecretName:masterSecreteName
                                 claimDefsJSON:claimDefsJson
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

- (NSError *)verifierVerifyProof:(NSString *)proofRequestJson
                       proofJson:(NSString *)proofJson
                     schemasJson:(NSString *)schemasJson
                   claimDefsJson:(NSString *)claimDefsJson
                revocRegDefsJSON:(NSString *)revocRegDefsJSON
                   revocRegsJson:(NSString *)revocRegsJson
                        outValid:(BOOL *)isValid {
    __block NSError *err = nil;
    XCTestExpectation *completionExpectation = nil;

    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    [IndyAnoncreds verifierVerifyProofRequest:proofRequestJson
                                    proofJSON:proofJson
                                  schemasJSON:schemasJson
                                claimDefsJSON:claimDefsJson
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
                                      claimDefJson:(NSString **)claimDefJson
                                    claimOfferJson:(NSString **)claimOfferJson
                                      claimReqJson:(NSString **)claimReqJson
                                         claimJson:(NSString **)claimJson {
    if (self.isCommonWalletCreated) {
        if (walletHandle) {
            *walletHandle = self.walletHandle;
        }
        if (claimDefJson) {
            *claimDefJson = _singletoneClaimdefJson;
        }
        if (claimOfferJson) {
            *claimOfferJson = _singletoneClaimofferJson;
        }
        if (claimReqJson) {
            *claimReqJson = _singletoneClaimreqJson;
        }
        if (claimJson) {
            *claimJson = _singletoneClaimJson;
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
                                          name:@"gvt"
                                       version:@"1.0"
                                         attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                      schemaId:&gvtSchemaId
                                    schemaJson:&gvtSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([gvtSchemaId isValid], @"invalid gvtSchemaId: %@", gvtSchemaId);
    XCTAssertTrue([gvtSchemaJson isValid], @"invalid gvtSchemaJson: %@", gvtSchemaJson);

    //3. Create XYZ Schema
    NSString *xyzSchemaId;
    NSString *xyzSchemaJson;
    ret = [self issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                          name:@"xyz"
                                       version:@"1.0"
                                         attrs:@"[\"status\",\"period\"]"
                                      schemaId:&xyzSchemaId
                                    schemaJson:&xyzSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([xyzSchemaId isValid], @"invalid xyzSchemaId: %@", xyzSchemaId);
    XCTAssertTrue([xyzSchemaJson isValid], @"invalid xyzSchemaJson: %@", xyzSchemaJson);

    //4. Issuer1 create GVT ClaimDef
    NSString *issuer1GvtClaimDefId;
    NSString *issuer1GvtClaimDefJson;
    ret = [self issuerCreateClaimDefinifionWithWalletHandle:tempWalletHandle
                                                  issuerDid:[TestUtils issuerDid]
                                                 schemaJson:gvtSchemaJson
                                                        tag:@"TAG1"
                                                       type:nil
                                                 configJson:@"{\"support_revocation\": false}"
                                                 claimDefId:&issuer1GvtClaimDefId
                                               claimDefJson:&issuer1GvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    XCTAssertTrue([issuer1GvtClaimDefId isValid], @"invalid issuer1GvtClaimDefId: %@", issuer1GvtClaimDefId);
    XCTAssertTrue([issuer1GvtClaimDefJson isValid], @"invalid issuer1GvtClaimDefJson: %@", issuer1GvtClaimDefJson);

    //5. Issuer1 create XYZ ClaimDef
    NSString *issuer1XyzClaimDefId;
    NSString *issuer1XyzClaimDefJson;
    ret = [self issuerCreateClaimDefinifionWithWalletHandle:tempWalletHandle
                                                  issuerDid:[TestUtils issuerDid]
                                                 schemaJson:xyzSchemaJson
                                                        tag:@"TAG1"
                                                       type:nil
                                                 configJson:@"{\"support_revocation\": false}"
                                                 claimDefId:&issuer1XyzClaimDefId
                                               claimDefJson:&issuer1XyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    XCTAssertTrue([issuer1XyzClaimDefId isValid], @"invalid issuer1XyzClaimDefId: %@", issuer1XyzClaimDefId);
    XCTAssertTrue([issuer1XyzClaimDefJson isValid], @"invalid issuer1XyzClaimDefJson: %@", issuer1XyzClaimDefJson);

    //6. Issuer2 create XYZ ClaimDef
    NSString *issuer2GvtClaimDefId;
    NSString *issuer2GvtClaimDefJson;
    ret = [self issuerCreateClaimDefinifionWithWalletHandle:tempWalletHandle
                                                  issuerDid:[TestUtils issuer2Did]
                                                 schemaJson:gvtSchemaJson
                                                        tag:@"TAG1"
                                                       type:nil
                                                 configJson:@"{\"support_revocation\": false}"
                                                 claimDefId:&issuer2GvtClaimDefId
                                               claimDefJson:&issuer2GvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    XCTAssertTrue([issuer2GvtClaimDefId isValid], @"invalid issuer2GvtClaimDefId: %@", issuer2GvtClaimDefId);
    XCTAssertTrue([issuer2GvtClaimDefJson isValid], @"invalid issuer2GvtClaimDefJson: %@", issuer2GvtClaimDefJson);


    // 7. Issuer1 create GVT Claim Offer
    NSString *issuer1GvtClaimOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:tempWalletHandle
                                                                       claimDefId:issuer1GvtClaimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer1GvtClaimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    //8. Prover store Issuer1 GVT Claim Offer
    ret = [self proverStoreClaimOffer:tempWalletHandle claimOfferJson:issuer1GvtClaimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaimOffer failed for issuer1GvtClaimOfferJSON");

    //9. Issuer1 create XYZ Claim Offer
    NSString *issuer1XyzClaimOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:tempWalletHandle
                                                                       claimDefId:issuer1XyzClaimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer1XyzClaimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    //10. Prover store Issuer1 XYZ Claim Offer
    ret = [self proverStoreClaimOffer:tempWalletHandle claimOfferJson:issuer1XyzClaimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaimOffer failed for issuer1XyzClaimOfferJSON");

    // 11. Issuer2 create GVT Claim Offer
    NSString *issuer2GvtClaimOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:tempWalletHandle
                                                                       claimDefId:issuer2GvtClaimDefId
                                                                        issuerDid:[TestUtils issuer2Did]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer2GvtClaimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    //12. Prover store Issuer2 GVT Claim Offer
    ret = [self proverStoreClaimOffer:tempWalletHandle claimOfferJson:issuer2GvtClaimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaimOffer failed for issuer2GvtClaimOfferJSON");


    //13. Create MasterSecret
    ret = [self proverCreateMasterSecretNamed:[TestUtils commonMasterSecretName]
                                 walletHandle:tempWalletHandle];
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret failed");

    //14. Prover create and Store Claim Request for Issuer1 GVT Claim
    NSString *issuer1GvtClaimRequest;
    ret = [self proverCreateAndStoreClaimReqWithDef:issuer1GvtClaimDefJson
                                          proverDid:[TestUtils proverDid]
                                     claimOfferJson:issuer1GvtClaimOfferJSON
                                   masterSecretName:[TestUtils commonMasterSecretName]
                                       walletHandle:tempWalletHandle
                                    outClaimReqJson:&issuer1GvtClaimRequest];

    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq failed for issuer1GvtClaimRequest");
    XCTAssertTrue([issuer1GvtClaimRequest isValid], @"invalid claim request: %@", issuer1GvtClaimRequest);

    if (ret.code != Success) {return ret;}

    //15. Issuer1 create  GVT Claim
    NSString *issuer1GvtClaim;
    ret = [self issuerCreateClaimWithWalletHandle:tempWalletHandle
                                     claimReqJson:issuer1GvtClaimRequest
                                  claimValuesJson:[self getGvtClaimValuesJson]
                                         revRegId:nil
                                tailsReaderHandle:nil
                                   userRevocIndex:nil
                                     outClaimJson:&issuer1GvtClaim
                             outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([issuer1GvtClaim isValid], @"invalid issuer1GvtClaim: %@", issuer1GvtClaim);

    //16. Prover store GVT claim from issuer1
    ret = [self proverStoreClaimWithWalletHandle:tempWalletHandle
                                         claimId:@"ClaimId1"
                                      claimsJson:issuer1GvtClaim
                                   revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaimWithWalletHandle failed");

    //17. Prover create and Store Claim Request for Issuer1 XYZ Claim
    NSString *issuer1XyzClaimRequest;
    ret = [self proverCreateAndStoreClaimReqWithDef:issuer1XyzClaimDefJson
                                          proverDid:[TestUtils proverDid]
                                     claimOfferJson:issuer1XyzClaimOfferJSON
                                   masterSecretName:[TestUtils commonMasterSecretName]
                                       walletHandle:tempWalletHandle
                                    outClaimReqJson:&issuer1XyzClaimRequest];

    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq failed for issuer1XyzClaimRequest");
    XCTAssertTrue([issuer1XyzClaimRequest isValid], @"invalid claim request: %@", issuer1XyzClaimRequest);

    if (ret.code != Success) {return ret;}

    //18. Issuer1 create XYZ Claim
    NSString *issuer1XyzClaim;
    ret = [self issuerCreateClaimWithWalletHandle:tempWalletHandle
                                     claimReqJson:issuer1XyzClaimRequest
                                  claimValuesJson:[self getXyzClaimValuesJson]
                                         revRegId:nil
                                tailsReaderHandle:nil
                                   userRevocIndex:nil
                                     outClaimJson:&issuer1XyzClaim
                             outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([issuer1XyzClaim isValid], @"invalid issuer1XyzClaim: %@", issuer1XyzClaim);

    //19. Prover store XYZ claim from issuer1
    ret = [self proverStoreClaimWithWalletHandle:tempWalletHandle
                                         claimId:@"ClaimId2"
                                      claimsJson:issuer1XyzClaim
                                   revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaimWithWalletHandle failed");


    //20. Prover create and Store Claim Request for Issuer2 GVT Claim
    NSString *issuer2GvtClaimRequest;
    ret = [self proverCreateAndStoreClaimReqWithDef:issuer2GvtClaimDefJson
                                          proverDid:[TestUtils proverDid]
                                     claimOfferJson:issuer2GvtClaimOfferJSON
                                   masterSecretName:[TestUtils commonMasterSecretName]
                                       walletHandle:tempWalletHandle
                                    outClaimReqJson:&issuer2GvtClaimRequest];

    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq failed for issuer2GvtClaimRequest");
    XCTAssertTrue([issuer2GvtClaimRequest isValid], @"invalid claim request: %@", issuer2GvtClaimRequest);

    if (ret.code != Success) {return ret;}

    //21. Issuer2 create GVT Claim
    NSString *issuer2GvtClaim;
    ret = [self issuerCreateClaimWithWalletHandle:tempWalletHandle
                                     claimReqJson:issuer2GvtClaimRequest
                                  claimValuesJson:[self getGvt2ClaimValuesJson]
                                         revRegId:nil
                                tailsReaderHandle:nil
                                   userRevocIndex:nil
                                     outClaimJson:&issuer2GvtClaim
                             outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([issuer2GvtClaim isValid], @"invalid issuer2GvtClaim: %@", issuer2GvtClaim);

    //22. Prover store GVT claim from Issuer2
    ret = [self proverStoreClaimWithWalletHandle:tempWalletHandle
                                         claimId:@"ClaimId3"
                                      claimsJson:issuer2GvtClaim
                                   revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaimWithWalletHandle failed");

    if (walletHandle) {*walletHandle = tempWalletHandle;}
    if (claimDefJson) {*claimDefJson = issuer1GvtClaimDefJson;}
    if (claimOfferJson) {*claimOfferJson = issuer1GvtClaimOfferJSON;}
    if (claimReqJson) {*claimReqJson = issuer1GvtClaimRequest;}
    if (claimJson) {*claimJson = issuer1GvtClaim;}
    self.walletHandle = tempWalletHandle;
    _singletoneClaimdefJson = issuer1GvtClaimDefJson;
    _singletoneClaimofferJson = issuer1GvtClaimOfferJSON;
    _singletoneClaimreqJson = issuer1GvtClaimRequest;
    _singletoneClaimJson = issuer1GvtClaim;

    self.isCommonWalletCreated = true;

    return ret;
}

@end
