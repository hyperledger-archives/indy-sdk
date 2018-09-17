using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerRevokeClaimTest : AnonCredsIntegrationTestBase
    {
        private const string WALLET_NAME = "commonWallet";
        private const string WALLET_KEY = "commonWalletKey";
        private Wallet _issuerWallet;
        private const string _walletName = "issuerWallet";
        private string _tailsWriterConfig = string.Format("{{\"base_dir\":\"{0}\", \"uri_pattern\":\"\"}}", "TODO");
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
        private IssuerCreateCredentialResult _claimResult;

        [TestInitialize]
        public async Task Before()
        {
            StorageUtils.CleanupStorage();

            //1. Create Issuer wallet, get wallet handle
            await WalletUtils.CreateWallet(WALLET_NAME, WALLET_KEY);
            _issuerWallet = await WalletUtils.OpenWallet(WALLET_NAME, WALLET_KEY);

            //2. Issuer create claim definition
            IssuerCreateAndStoreCredentialDefResult result = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(_issuerWallet, issuerDid, schema, null, null, null);
            _claimDefJson = result.CredDefJson;

            //3. Issuer create revocation registry
            BlobStorageWriter tailsWriter = await BlobStorage.OpenWriterAsync("default", _tailsWriterConfig);
            await AnonCreds.IssuerCreateAndStoreRevocRegAsync(_issuerWallet, issuerDid, null, null, null, null, tailsWriter);

            //4. Prover create Master Secret
            await AnonCreds.ProverCreateMasterSecretAsync(_issuerWallet, masterSecretName);

            //5. Prover store Claim Offer received from Issuer
            var claimOfferJson = string.Format(claimOfferTemplate, issuerDid, 1);
            // TODO await AnonCreds.ProverStoreCredentialOfferAsync(_issuerWallet, claimOfferJson);

            //6. Prover create Claim Request
            var proverDid = "BzfFCYk";
            var claimReq = await AnonCreds.ProverCreateCredentialReqAsync(
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

            
            _claimResult = null; // TODO await AnonCreds.IssuerCreateCredentialAsync(_issuerWallet, claimReq, claimJson, _userRevocIndex);

            //8. Prover store received Claim
            // TODO await AnonCreds.ProverStoreClaimAsync(_issuerWallet, _claimResult.CredentialJson, _claimResult.RevocRegDeltaJson);
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
            var revocRegUpdateJson = ""; // TODO await AnonCreds.IssuerRevokeCredentialAsync(
                //_issuerWallet,
                //issuerDid,
                //1,
                //1);

            //10. Prover gets Claims for Proof Request
            var claimsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(_issuerWallet, _proofReqJson);
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
            var claimsJson = await AnonCreds.ProverGetCredentialsForProofReqAsync(_issuerWallet, _proofReqJson);
            var claims = JObject.Parse(claimsJson);
            var claimsForAttr1 = claims["attrs"]["attr1_referent"];
            var claim = claimsForAttr1[0];
            var claimUuid = claim.Value<string>("referent");

            //10. Prover create Proof
            var requestedClaimsJson = string.Format(_requestedClaimsJsonTemplate, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDefJson);
            var revocRegsJsons = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimResult.RevocRegDeltaJson);

            var proofJson = await AnonCreds.ProverCreateProofAsync(
                _issuerWallet,
                _proofReqJson,
                requestedClaimsJson,
                schemasJson,
                masterSecretName,
                claimDefsJson,
                revocRegsJsons);

            //11. Issuer revoke prover claim
            // TODO
            //var revocRegUpdateJson = await AnonCreds.IssuerRevokeCredentialAsync(
            //    _issuerWallet,
            //    issuerDid,
            //    1,
            //    1);

            ////12. Verifier verify proof
            //var updatedRevocRegsJsons = string.Format("{{\"{0}\":{1}}}", claimUuid, revocRegUpdateJson);

            var valid = false; // TODO await AnonCreds.VerifierVerifyProofAsync(
                //_proofReqJson,
                //proofJson,
                //schemasJson,
                //claimDefsJson,
                //updatedRevocRegsJsons);

            Assert.IsFalse(valid);
        }
    }
}
