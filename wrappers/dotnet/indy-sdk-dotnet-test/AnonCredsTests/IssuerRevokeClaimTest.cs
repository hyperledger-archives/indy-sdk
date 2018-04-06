using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerRevokeClaimTest : AnonCredsIntegrationTestBase
    {
        private Wallet _issuerWallet;
        private const string _walletName = "issuerWallet";
        private const int _userRevocIndex = 1;
        private const string _proofReqJson = "{" +
                                       "\"nonce\":\"123432421212\"," +
                                       "\"name\":\"proof_req_1\"," +
                                       "\"version\":\"0.1\"," +
                                       "\"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_seq_no\":1}]}}," +
                                       "\"requested_predicates\":{}" +
                                    "}";    
        
        private const string _requestedClaimsJsonTemplate = "{{" +
                                                      "\"self_attested_attributes\":{{}}," +
                                                      "\"requested_attrs\":{{\"attr1_referent\":[\"{0}\", true]}}," +
                                                      "\"requested_predicates\":{{}}" +
                                                    "}}";
        private string _claimDefJson;
        private IssuerCreateClaimResult _claimResult;

        [TestInitialize]
        public async Task Before()
        {
            StorageUtils.CleanupStorage();

            //1. Create Issuer wallet, get wallet handle
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _issuerWallet = await Wallet.OpenWalletAsync(_walletName, null, null);

            //2. Issuer create claim definition
            _claimDefJson = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_issuerWallet, issuerDid, schema, null, true);

            //3. Issuer create revocation registry
            await AnonCreds.IssuerCreateAndStoreRevocRegAsync(_issuerWallet, issuerDid, 1, 5);

            //4. Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(_issuerWallet, masterSecretName);

            //5. Prover store Claim Offer received from Issuer
            var claimOfferJson = string.Format(claimOfferTemplate, issuerDid, 1);
            await AnonCreds.ProverStoreClaimOfferAsync(_issuerWallet, claimOfferJson);

            //6. Prover create Claim Request
            var proverDid = "BzfFCYk";
            var claimReq = await AnonCreds.ProverCreateAndStoreClaimReqAsync(
                _issuerWallet,
                proverDid,
                claimOfferJson,
                _claimDefJson,
                masterSecretName);

            //7. Issuer create Claim
            var claimJson = "{" +
                "\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"]," +
               "\"name\":[\"Alex\",\"1139481716457488690172217916278103335\"]," +
               "\"height\":[\"175\",\"175\"]," +
               "\"age\":[\"28\",\"28\"]" +
            "}";

            
            _claimResult = await AnonCreds.IssuerCreateClaimAsync(_issuerWallet, claimReq, claimJson, _userRevocIndex);

            //8. Prover store received Claim
            await AnonCreds.ProverStoreClaimAsync(_issuerWallet, _claimResult.ClaimJson, _claimResult.RevocRegUpdateJson);
        }

        [TestCleanup]
        public async Task After()
        {
            await _issuerWallet.CloseAsync();
            StorageUtils.CleanupStorage();
        }

        [TestMethod]
        public async Task TestAnoncredsWorksForClaimRevokedBeforeProofCreated()
        {
            //9. Issuer revoke claim
            var revocRegUpdateJson = await AnonCreds.IssuerRevokeClaimAsync(
                _issuerWallet,
                issuerDid,
                1,
                1);

            //10. Prover gets Claims for Proof Request
            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_issuerWallet, _proofReqJson);
            var claims = JObject.Parse(claimsJson);
            var claimsForAttr1 = claims["attrs"]["attr1_referent"];
            var claim = claimsForAttr1[0];
            var claimUuid = claim.Value<string>("referent");

            //11. Prover create Proof
            var requestedClaimsJson = string.Format(_requestedClaimsJsonTemplate, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDefJson);
            var revocRegsJsons = string.Format("{{\"{0}\":{1}}}", claimUuid, revocRegUpdateJson);

            var ex = await Assert.ThrowsExceptionAsync<ClaimRevokedException>(() =>
                AnonCreds.ProverCreateProofAsync(
                    _issuerWallet,
                    _proofReqJson,
                    requestedClaimsJson,
                    schemasJson,
                    masterSecretName,
                    claimDefsJson,
                    revocRegsJsons)
            );
        }

        [TestMethod]
        public async Task TestAnoncredsWorksForClaimRevokedAfterProofCreated()
        {
            //9. Prover gets Claims for Proof Request
            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_issuerWallet, _proofReqJson);
            var claims = JObject.Parse(claimsJson);
            var claimsForAttr1 = claims["attrs"]["attr1_referent"];
            var claim = claimsForAttr1[0];
            var claimUuid = claim.Value<string>("referent");

            //10. Prover create Proof
            var requestedClaimsJson = string.Format(_requestedClaimsJsonTemplate, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDefJson);
            var revocRegsJsons = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimResult.RevocRegUpdateJson);

            var proofJson = await AnonCreds.ProverCreateProofAsync(
                _issuerWallet,
                _proofReqJson,
                requestedClaimsJson,
                schemasJson,
                masterSecretName,
                claimDefsJson,
                revocRegsJsons);

            //11. Issuer revoke prover claim
            var revocRegUpdateJson = await AnonCreds.IssuerRevokeClaimAsync(
                _issuerWallet,
                issuerDid,
                1,
                1);

            //12. Verifier verify proof
            var updatedRevocRegsJsons = string.Format("{{\"{0}\":{1}}}", claimUuid, revocRegUpdateJson);

            var valid = await AnonCreds.VerifierVerifyProofAsync(
                _proofReqJson,
                proofJson,
                schemasJson,
                claimDefsJson,
                updatedRevocRegsJsons);

            Assert.IsFalse(valid);
        }
    }
}
