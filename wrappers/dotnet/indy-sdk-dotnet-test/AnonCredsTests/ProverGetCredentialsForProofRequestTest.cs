using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverGetCredentialsForProofRequestTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttribute()
        {
            var proofRequest = "{" +
                "   \"nonce\":\"123432421212\"," +
                "   \"name\":\"proof_req_1\"," +
                "   \"version\":\"0.1\"," +
                "   \"requested_attributes\":{" +
                "       \"attr1_referent\":{\"name\":\"name\"}" +
                "   }," +
                "   \"requested_predicates\":{}" +
                "}";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, JObject.Parse(proofRequest).ToString());                

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["attrs"]["attr1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeInUpperCase()
        {
            var proofRequest = "{" +
                "   \"nonce\":\"123432421212\"," +
                "   \"name\":\"proof_req_1\"," +
                "   \"version\":\"0.1\"," +
                "   \"requested_attributes\":{" +
                "       \"attr1_referent\":{\"name\":\"NAME\"}" +
                "   }," +
                "   \"requested_predicates\":{}" +
                "}";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["attrs"]["attr1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeContainsSpaces()
        {
            var proofRequest = "{" +
                   "   \"nonce\":\"123432421212\"," +
                   "   \"name\":\"proof_req_1\"," +
                   "   \"version\":\"0.1\"," +
                   "   \"requested_attributes\":{" +
                   "       \"attr1_referent\":{\"name\":\" name \"}" +
                   "   }," +
                   "   \"requested_predicates\":{}" +
                   "}";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["attrs"]["attr1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForNotFoundAttribute()
        {
            var proofRequest = "{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{" +
                "                   \"attr1_referent\":{\"name\":\"attribute\"}" +
                "               }," +
                "              \"requested_predicates\":{}" +
                "             }";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["attrs"]["attr1_referent"];
            Assert.AreEqual(0, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicate()
        {
            var proofRequest = 
                "{" +
                "   \"nonce\":\"123432421212\"," +
                "   \"name\":\"proof_req_1\"," +
                "   \"version\":\"0.1\"," +
                "   \"requested_attributes\":{}," +
                "   \"requested_predicates\":{" +
                "       \"predicate1_referent\":{" +
                "           \"name\":\"age\",\"p_type\":\">=\",\"p_value\":18" +
                "       }" +
                "   }" +
                "}";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateAttrInUpperCase()
        {
            var proofRequest = "{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{}," +
                "              \"requested_predicates\":{" +
                "                   \"predicate1_referent\":{" +
                "                       \"name\":\"AGE\",\"p_type\":\">=\",\"p_value\":18" +
                "                   }" +
                "               }" +
                "             }";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateAttrContainsSpaces()
        {
            var proofRequest = "{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{}," +
                "              \"requested_predicates\":{" +
                "                   \"predicate1_referent\":{" +
                "                       \"name\":\" age \",\"p_type\":\">=\",\"p_value\":18" +
                "                   }" +
                "               }" +
                "             }";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForNotSatisfiedPredicate()
        {
            var proofRequest = "{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{}," +
                "              \"requested_predicates\":{" +
                "                   \"predicate1_referent\":{" +
                "                       \"name\":\"age\",\"p_type\":\">=\",\"p_value\":58" +
                "                   }" +
                "               }" +
                "             }";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(0, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForMultipleAttributesAndPredicates()
        {
            var proofRequest = "{\"nonce\":\"123432421212\"," +
                "               \"name\":\"proof_req_1\"," +
                "               \"version\":\"0.1\"," +
                "               \"requested_attributes\":{" +
                "                     \"attr1_referent\":{\"name\":\"name\"}," +
                "                     \"attr2_referent\":{\"name\":\"sex\"}" +
                "               }," +
                "               \"requested_predicates\":{" +
                "                     \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}," +
                "                     \"predicate2_referent\":{\"name\":\"height\",\"p_type\":\">=\",\"p_value\":160}" +
                "               }}";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["attrs"]["attr1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);

            var credentialsForAttribute2 = (JArray)credentials["attrs"]["attr2_referent"];
            Assert.AreEqual(2, credentialsForAttribute2.Count);

            var credentialsForPredicate1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForPredicate1.Count);

            var credentialsForPredicate2 = (JArray)credentials["predicates"]["predicate2_referent"];
            Assert.AreEqual(2, credentialsForPredicate2.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForEmptyRequest()
        {
            var proofRequest = "{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{}," +
                "              \"requested_predicates\":{}" +
                "             }";

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            Assert.AreEqual(0, ((JObject)credentials["attrs"]).Count);
            Assert.AreEqual(0, ((JObject)credentials["predicates"]).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeBySpecificIssuer()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{" +
                "                   \"attr1_referent\":{{" +
                "                       \"name\":\"name\"," +
                "                       \"restrictions\":[{{\"issuer_did\":\"{0}\"}}]" +
                "                   }}" +
                "               }}," +
                "              \"requested_predicates\":{{}}" +
                "             }}", issuerDid);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = credentials["attrs"]["attr1_referent"];

            Assert.AreEqual(1, ((JArray)credentialsForAttribute1).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeBySchemaId()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{" +
                "                   \"attr1_referent\":{{" +
                "                       \"name\":\"name\"," +
                "                       \"restrictions\":[{{\"schema_id\":\"{0}\"}}]" +
                "                   }}" +
                "               }}," +
                "              \"requested_predicates\":{{}}" +
                "             }}", gvtSchemaId);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = credentials["attrs"]["attr1_referent"];

            Assert.AreEqual(2, ((JArray)credentialsForAttribute1).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeBySchemaName()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{" +
                "                   \"attr1_referent\":{{" +
                "                       \"name\":\"name\"," +
                "                       \"restrictions\":[{{\"schema_name\":\"{0}\"}}]" +
                "                   }}" +
                "               }}," +
                "              \"requested_predicates\":{{}}" +
                "             }}", gvtSchemaName);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = credentials["attrs"]["attr1_referent"];

            Assert.AreEqual(2, ((JArray)credentialsForAttribute1).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeByMultipleSchemas()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{" +
                "                   \"attr1_referent\":{{" +
                "                       \"name\":\"name\"," +
                "                       \"restrictions\":[{{\"schema_id\":\"{0}\"}}, {{\"schema_id\":\"{1}\"}}]" +
                "                   }}" +
                "               }}," +
                "              \"requested_predicates\":{{}}" +
                "             }}", gvtSchemaId, xyzSchemaId);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = credentials["attrs"]["attr1_referent"];

            Assert.AreEqual(2, ((JArray)credentialsForAttribute1).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeByCredDefId()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{" +
                "                   \"attr1_referent\":{{" +
                "                       \"name\":\"name\"," +
                "                       \"restrictions\":[{{\"cred_def_id\":\"{0}\"}}]" +
                "                   }}" +
                "               }}," +
                "              \"requested_predicates\":{{}}" +
                "             }}", issuer1gvtCredDefId);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = credentials["attrs"]["attr1_referent"];

            Assert.AreEqual(1, ((JArray)credentialsForAttribute1).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForRevealedAttributeBySpecificSchemaOrSpecificIssuer()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{" +
                "                   \"attr1_referent\":{{" +
                "                       \"name\":\"name\"," +
                "                       \"restrictions\":[{{\"schema_id\":\"{0}\"}}, {{\"issuer_did\":\"{1}\"}}]" +
                "                   }}" +
                "               }}," +
                "              \"requested_predicates\":{{}}" +
                "             }}", gvtSchemaId, issuerDid);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = credentials["attrs"]["attr1_referent"];

            Assert.AreEqual(2, ((JArray)credentialsForAttribute1).Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateBySpecificIssuer()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{}}," +
                "              \"requested_predicates\":{{" +
                "                   \"predicate1_referent\":{{" +
                "                       \"name\":\"age\", \"p_type\":\">=\", \"p_value\":18," +
                "                       \"restrictions\":[{{\"issuer_did\":\"{0}\"}}]" +
                "                   }}" +
                "              }}" +
                "             }}", issuerDid);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(1, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateBySchemaId()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{}}," +
                "              \"requested_predicates\":{{" +
                "                   \"predicate1_referent\":{{" +
                "                       \"name\":\"age\", \"p_type\":\">=\", \"p_value\":18," +
                "                       \"restrictions\":[{{\"schema_id\":\"{0}\"}}]" +
                "                   }}" +
                "              }}" +
                "             }}", gvtSchemaId);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateByMultipleSchemas()
        {
            var proofRequest = string.Format("{{" +
                "              \"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{}}," +
                "              \"requested_predicates\":{{" +
                "                   \"predicate1_referent\":{{" +
                "                       \"name\":\"age\", \"p_type\":\">=\", \"p_value\":18," +
                "                       \"restrictions\":[{{\"schema_id\":\"{0}\"}}, {{\"schema_id\":\"{1}\"}}]" +
                "                   }}" +
                "              }}" +
                "             }}", gvtSchemaId, xyzSchemaId);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateBySpecificCredDefId()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{}}," +
                "              \"requested_predicates\":{{" +
                "                   \"predicate1_referent\":{{" +
                "                       \"name\":\"age\", \"p_type\":\">=\", \"p_value\":18," +
                "                       \"restrictions\":[{{\"cred_def_id\":\"{0}\"}}]" +
                "                   }}" +
                "              }}" +
                "             }}", issuer1gvtCredDefId);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(1, credentialsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetCredentialsForProofRequestWorksForPredicateBySpecificIssuerOrSpecificSchema()
        {
            var proofRequest = string.Format("{{\"nonce\":\"123432421212\"," +
                "              \"name\":\"proof_req_1\"," +
                "              \"version\":\"0.1\"," +
                "              \"requested_attributes\":{{}}," +
                "              \"requested_predicates\":{{" +
                "                   \"predicate1_referent\":{{" +
                "                       \"name\":\"age\", \"p_type\":\">=\", \"p_value\":18," +
                "                       \"restrictions\":[{{\"schema_id\":\"{0}\"}}, {{\"issuer_id\":\"{1}\"}}]" +
                "                   }}" +
                "              }}" +
                "             }}", gvtSchemaId, issuerDid);

            var credentialsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(wallet, proofRequest);

            var credentials = JObject.Parse(credentialsJson);

            var credentialsForAttribute1 = (JArray)credentials["predicates"]["predicate1_referent"];
            Assert.AreEqual(2, credentialsForAttribute1.Count);
        }
    }
}
