using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class RegisterWalletTypeTest : IndyIntegrationTestBase
    {
        protected static string _masterSecretName = "master_secret_name";
        protected static string _issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        protected static string _issuerDid2 = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected static string _proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected static string _claimOfferTemplate = "{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}";
        protected static string _schema = "{\"seqNo\":1,\"data\": {\"name\":\"gvt\",\"version\":\"1.0\",\"keys\":[\"age\",\"sex\",\"height\",\"name\"]}}";
        protected static string _claimRequestTemplate =
            "{{\"blinded_ms\":" +
            "{{\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"," +
            "\"u\":\"54172737564529332710724213139048941083013176891644677117322321823630308734620627329227591845094100636256829761959157314784293939045176621327154990908459072821826818718739696323299787928173535529024556540323709578850706993294234966440826690899266872682790228513973999212370574548239877108511283629423807338632435431097339875665075453785141722989098387895970395982432709011505864533727415552566715069675346220752584449560407261446567731711814188836703337365986725429656195275616846543535707364215498980750860746440672050640048215761507774996460985293327604627646056062013419674090094698841792968543317468164175921100038\"," +
            "\"ur\":null}}," +
            "\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}";

        [TestMethod]
        [Ignore]
        public async Task TestExerciseCustomWallet()
        {
            await Wallet.RegisterWalletTypeAsync("inmem", new InMemWalletType());

            var walletName = "registerWalletTypeWorks";

            await Wallet.CreateWalletAsync("default", walletName, "inmem", null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);
            Assert.IsNotNull(wallet);

            var claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(wallet, _issuerDid, _schema, null, false);

            await AnonCreds.ProverStoreClaimOfferAsync(wallet, string.Format(_claimOfferTemplate, _issuerDid, 1));
            await AnonCreds.ProverStoreClaimOfferAsync(wallet, string.Format(_claimOfferTemplate, _issuerDid, 2));
            await AnonCreds.ProverStoreClaimOfferAsync(wallet, string.Format(_claimOfferTemplate, _issuerDid2, 2));

            await AnonCreds.ProverCreateMasterSecretAsync(wallet, _masterSecretName);

            var claimOffer = string.Format("{{\"issuer_did\":\"{0}\",\"schema_seq_no\":{1}}}", _issuerDid, 1);

            var claimRequest = await AnonCreds.ProverCreateAndStoreClaimReqAsync(wallet, "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", claimOffer, claimDef, _masterSecretName);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "                 \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "                 \"height\":[\"175\",\"175\"],\n" +
                    "                 \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateClaimAsync(wallet, claimRequest, claim, -1);
            var claimJson = createClaimResult.ClaimJson;

            await AnonCreds.ProverStoreClaimAsync(wallet, claimJson);

            var filter = string.Format("{{\"issuer_did\":\"{0}\"}}", _issuerDid);

            var claimOffers = await AnonCreds.ProverGetClaimOffersAsync(wallet, filter);

            await wallet.CloseAsync();
        }

        [TestMethod]
        [Ignore] //Not a valid test since the wallet type is registered on init!
        public async Task TestRegisterWalletTypeWorks()
        {
            await Wallet.RegisterWalletTypeAsync("inmem", new InMemWalletType());
        }

        [TestMethod]
        public async Task TestRegisterWalletTypeDoesNotWorkForTwiceWithSameName()
        {
            var type = "inmem";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.RegisterWalletTypeAsync(type, new InMemWalletType())
            );

            Assert.AreEqual(ErrorCode.WalletTypeAlreadyRegisteredError, ex.ErrorCode);
        }
    }
}
