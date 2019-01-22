from indy.anoncreds import verifier_verify_proof
from indy.error import ErrorCode, IndyError

import json
import pytest

proof = {
    "proof": {
        "proofs": [
            {
                "primary_proof": {
                    "eq_proof": {
                        "revealed_attrs": {"name": "1139481716457488690172217916278103335"},
                        "a_prime": "69611333210246297576674481997568350651893753169421675404246876477070036036239748981216968970512619021799004203771400004745254757568519299187403980953726423401562027797824596981803651956815187614880367067301256241097316464043413280050386421276754980523795874441669038235540140152656908807883825784389214619685018577672760314282808562442029843500291305232360392810866899927630168027160726855963031718816003269941833088853627074874414033579209800976408716012337949780018118282090855610902050775692383849092885096943305849969830611477014891883778727552346997585823322521250895404620518020788058681521917908040585781734067",
                        "e": "81997215335430300827922871366991164391005799351165496986980891266603511485892000209592180159374529694715868455586389695646654479833034370",
                        "v": "32782954961914516006929947213986869948692168538360049258687349075247272204613878574070856880834872045205057475334027733394962548472043563337437602789133971395548247999626513733176359099383385194438878908259188028211542679253994949929575742614433996692495254993822996369163735079781691759719898135196766016342321999133966411928405589554348103569872899345465126476989438710701806074959101389034068697155872379036577050686491975117030482096804843211430915855417128832872693971375502099774489871267125816084393192714678134103895689079058929103812329992054428133120225577520318488258071881679293704747710704597070041406207148072340210499233299784895244133717672877798697782134198701118771838826231256346275458292651558998202624408710499781368839761664636874421927015663020439827618429936752280834497539997936244979147788286071991754365356807574919976551950538173762001192358536353716193794673263137779788159041386709140821203",
                        "m": {
                            "height": "644661525186404763073940113925189284399802974351075121099843277144169464277989925412469337223584090097518652961751117818377851925608165622042123927003184744895325285859259895137",
                            "master_secret": "565103107690950209567260176581405152588756866843977477605720615638626097020212857648927234056238060079923284656984433623039675964740231040219306442454547101602808729671253494116",
                            "age": "5284383760970370269648766907933638199428222482643563758416683902026426410513643984935437869399510331816719649912008370311959387905772883278353421522148721288628772925652054921277",
                            "sex": "3055027352388605443362412916804749467376785200254605029426986674097166523552496572383751806329430159164180380154311761175487624079637842401252633327500680923590031092457240612097"},
                        "m2": "9896338009583091813083207288865871877689349761014485227109334393082244453651004498708224344089082995670102330344137500838507144560401467324174529793376581615034515792555167630079"},
                    "ge_proofs": []
                },
                "non_revoc_proof": None
            }
        ],
        "aggregated_proof": {
            "c_hash": "25761899986302698893447148155874056850328918157347065810441059305315654886874", "c_list": [
                [2, 39, 109, 143, 241, 151, 10, 188, 158, 124, 228, 215, 128, 78, 82, 202, 109, 38, 159, 26, 163, 126,
                 219, 31,
                 238, 71, 200, 115, 225, 248, 216, 128, 25, 16, 146, 196, 98, 143, 49, 6, 70, 127, 85, 149, 63, 226, 99,
                 146,
                 51, 188, 141, 215, 138, 73, 144, 52, 170, 59, 65, 80, 163, 76, 175, 60, 107, 98, 250, 177, 154, 210,
                 185, 193,
                 31, 129, 7, 211, 41, 27, 21, 189, 86, 154, 104, 168, 158, 60, 240, 206, 91, 166, 195, 159, 66, 108, 54,
                 137,
                 106, 72, 67, 121, 121, 131, 211, 180, 229, 75, 223, 66, 34, 234, 199, 248, 124, 212, 131, 217, 46, 33,
                 177,
                 110, 181, 194, 7, 78, 102, 40, 97, 87, 51, 155, 102, 201, 174, 236, 217, 29, 214, 58, 149, 123, 70, 38,
                 88,
                 160, 149, 144, 35, 234, 104, 91, 104, 67, 24, 18, 65, 86, 201, 88, 144, 8, 195, 179, 216, 126, 95, 45,
                 121, 62,
                 70, 183, 95, 42, 68, 222, 244, 122, 236, 145, 236, 35, 53, 90, 218, 194, 94, 137, 206, 249, 201, 172,
                 236, 153,
                 78, 215, 108, 248, 190, 101, 21, 170, 26, 89, 159, 93, 89, 146, 78, 222, 30, 122, 128, 4, 71, 55, 155,
                 240, 78,
                 222, 63, 30, 245, 149, 92, 1, 87, 78, 169, 46, 93, 95, 206, 7, 162, 199, 112, 126, 213, 216, 153, 18,
                 147, 103,
                 50, 27, 238, 192, 176, 167, 138, 210, 219, 71, 117, 146, 252, 106, 179]]
        }
    },
    "requested_proof": {
        "revealed_attrs": {
            "attr1_referent": {
                "sub_proof_index": 0, "raw": "Alex", "encoded": "1139481716457488690172217916278103335"
            }
        },
        "self_attested_attrs": {},
        "unrevealed_attrs": {},
        "predicates": {}
    },
    "identifiers": [
        {
            "schema_id": "NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0",
            "cred_def_id": "NcYxiDXkpYi6ov5FcYDi1e:3:CL:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0", "rev_reg_id": None,
            "timestamp": None
        }]
}

proof_req = {
    "nonce": "123432421212",
    "name": "proof_req_1",
    "version": "0.1",
    "requested_attributes": {
        "attr1_referent": {"name": "name"}
    },
    "requested_predicates": {}
}

@pytest.mark.asyncio
async def test_verifier_verify_proof_works_for_correct_proof(credential_def, gvt_schema, gvt_schema_id,
                                                             issuer_1_gvt_cred_def_id):
    schemas = {
        gvt_schema_id: gvt_schema
    }

    credential_defs = {
        issuer_1_gvt_cred_def_id: credential_def
    }

    proof["identifiers"][0]["schema_id"] = gvt_schema_id
    proof["identifiers"][0]["cred_def_id"] = issuer_1_gvt_cred_def_id

    valid = await verifier_verify_proof(json.dumps(proof_req), json.dumps(proof),
                                        json.dumps(schemas), json.dumps(credential_defs), "{}", "{}")

    assert valid


@pytest.mark.asyncio
async def test_verifier_verify_proof_works_for_proof_does_not_correspond_to_request(credential_def, gvt_schema,
                                                                                    gvt_schema_id,
                                                                                    issuer_1_gvt_cred_def_id):
    xproof_req = {
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {"name": "sex"}
        },
        "requested_predicates": {}
    }

    schemas = {
        gvt_schema_id: gvt_schema
    }

    credential_defs = {
        issuer_1_gvt_cred_def_id: credential_def
    }

    proof["identifiers"][0]["schema_id"] = gvt_schema_id
    proof["identifiers"][0]["cred_def_id"] = issuer_1_gvt_cred_def_id

    with pytest.raises(IndyError) as e:
        await verifier_verify_proof(json.dumps(xproof_req), json.dumps(proof),
                                    json.dumps(schemas), json.dumps(credential_defs), "{}", "{}")

    assert ErrorCode.AnoncredsProofRejected == e.value.error_code


@pytest.mark.asyncio
async def test_verifier_verify_proof_works_for_wrong_proof(credential_def, gvt_schema, gvt_schema_id,
                                                           issuer_1_gvt_cred_def_id):
    schemas = {
        gvt_schema_id: gvt_schema
    }
    credential_defs = {
        issuer_1_gvt_cred_def_id: credential_def
    }
    proof["identifiers"][0]["schema_id"] = gvt_schema_id
    proof["identifiers"][0]["cred_def_id"] = issuer_1_gvt_cred_def_id
    proof["proof"]["aggregated_proof"]["c_hash"] = "111111111111111"

    valid = await verifier_verify_proof(json.dumps(proof_req), json.dumps(proof),
                                        json.dumps(schemas), json.dumps(credential_defs), "{}", "{}")

    assert not valid
