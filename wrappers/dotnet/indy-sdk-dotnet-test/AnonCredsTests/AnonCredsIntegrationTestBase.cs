using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{

    public abstract class AnonCredsIntegrationTestBase 
    {
        protected static Wallet commonWallet;
        protected static string claimDef;
        protected static string masterSecretName = "master_secret_name";
        protected static string issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        protected static string issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected static string proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected static string claimOfferTemplate = "{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}";
        protected static string schema = "{\"seqNo\":1,\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]}}";
        protected static string claimRequestTemplate =
            "{{\"blinded_ms\":" +
            "{{\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"," +
            "\"u\":\"54172737564529332710724213139048941083013176891644677117322321823630308734620627329227591845094100636256829761959157314784293939045176621327154990908459072821826818718739696323299787928173535529024556540323709578850706993294234966440826690899266872682790228513973999212370574548239877108511283629423807338632435431097339875665075453785141722989098387895970395982432709011505864533727415552566715069675346220752584449560407261446567731711814188836703337365986725429656195275616846543535707364215498980750860746440672050640048215761507774996460985293327604627646056062013419674090094698841792968543317468164175921100038\"," +
            "\"ur\":null}}," +
            "\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}";

        private static bool _walletOpened = false;

        [TestInitialize]
        public async Task SetUp() 
        {
            await InitHelper.InitAsync();
        }

        [ClassCleanup]
        public static async Task CloseCommonWallet()
        {
            if (_walletOpened)
            {
                await commonWallet.CloseAsync();
                _walletOpened = false;
            }

        }

        protected async Task InitCommonWallet()
        {
            if (_walletOpened)
                return;

            StorageUtils.CleanupStorage();

            var walletName = "anoncredsCommonWallet";

            await Wallet.CreateWalletAsync("default", walletName, "default", null, null);
            commonWallet = await Wallet.OpenWalletAsync(walletName, null, null);

            claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(commonWallet, issuerDid, schema, null, false);

            await AnonCreds.ProverStoreClaimOfferAsync(commonWallet, string.Format(claimOfferTemplate, issuerDid, 1));
            await AnonCreds.ProverStoreClaimOfferAsync(commonWallet, string.Format(claimOfferTemplate, issuerDid, 2));
            await AnonCreds.ProverStoreClaimOfferAsync(commonWallet, string.Format(claimOfferTemplate, issuerDid2, 2));

            await AnonCreds.ProverCreateMasterSecretAsync(commonWallet, masterSecretName);

            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}", issuerDid, 1);

            var claimRequest = await AnonCreds.ProverCreateAndStoreClaimReqAsync(commonWallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer, claimDef, masterSecretName);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "                 \"height\":[\"175\",\"175\"],\n" +
                    "                 \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateClaimAsync(commonWallet, claimRequest, claim, -1);
            var claimJson = createClaimResult.ClaimJson;

            await AnonCreds.ProverStoreClaimAsync(commonWallet, claimJson, createClaimResult.RevocRegUpdateJson);

            _walletOpened = true;
        }
    }
}
