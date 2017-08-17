﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;


namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class ProverCreateProofTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverCreateProofWorks()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                    "                                \"name\":\"proof_req_1\",\n" +
                    "                                \"version\":\"0.1\",\n" +
                    "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                    "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                    "                              }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);                

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"}}\n" +
                    "                                        }}", claimUuid, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            var proofJson = await AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson,
                    _masterSecretName, claimDefsJson, revocRegsJson);
            Assert.IsNotNull(proofJson);
        }


        [TestMethod]
        public async Task TestProverCreateProofWorksForUsingNotSatisfyClaim()
        {
            await InitCommonWallet();

            var claimsJson = await AnonCreds.ProverGetClaimsAsync(_commonWallet, "{}");

            var claims = JArray.Parse(claimsJson);

            var claimUuid = claims[0].Value<string>("claim_uuid");

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                    "               \"name\":\"proof_req_1\",\n" +
                    "               \"version\":\"0.1\",\n" +
                    "               \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"some_attr\"}},\n" +
                    "               \"requested_predicates\":{}\n" +
                    "              }";

            var requestedClaimsJson = string.Format("{{\"self_attested_attributes\":{{}},\n" +
                    "                                    \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                    \"requested_predicates\":{{}}\n" +
                    "                                   }}", claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson,
                    _masterSecretName, claimDefsJson, revocRegsJson)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidMasterSecret()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "                                \"name\":\"proof_req_1\",\n" +
                "                                \"version\":\"0.1\",\n" +
                "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "                              }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"}}\n" +
                    "                                        }}", claimUuid, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";
            
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson, "wrong_master_secret", claimDefsJson, revocRegsJson)

            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidSchemas()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "                                \"name\":\"proof_req_1\",\n" +
                "                                \"version\":\"0.1\",\n" +
                "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "                              }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"}}\n" +
                    "                                        }}", claimUuid, claimUuid);

            var schemasJson = "{}";
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson, _masterSecretName, claimDefsJson, revocRegsJson)

            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidRequestedClaimsJson()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "                                \"name\":\"proof_req_1\",\n" +
                "                                \"version\":\"0.1\",\n" +
                "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "                              }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            String requestedClaimsJson = "{\"self_attested_attributes\":{},\n" +
                "                      \"requested_predicates\":{}\n" +
                "                    }";

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson, "wrong_master_secret", claimDefsJson, revocRegsJson)

            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
