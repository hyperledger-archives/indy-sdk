using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class VerifyTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _myDid;
        private string _myVerkey;
        private byte[] _signature = (byte[])(Array) new sbyte[] {- 87, - 41, 8, - 31, 7, 107, 110, 9, - 63, - 94, - 54, - 42, - 94, 66, - 18, - 45, 63, - 47, 12, - 60, 8, - 45, 55, 27, 120, 94, - 52, - 109, 53, 104,
            103, 61, 60, - 7, - 19, 127, 103, 46, - 36, - 33, 10, 95, 75, 53, - 11, - 46, - 15, - 105, - 65, 41, 48, 30, 9, 16, 78, - 4, - 99, - 50, - 46, - 111, 125, - 123, 109, 11};
        
        [TestInitialize]
        public async Task Before()
        {
            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            _trusteeDid = trusteeDidResult.Did;
            _trusteeVerkey = trusteeDidResult.VerKey;

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            _myDid = myDidResult.Did;
            _myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(_trusteeDid, _myDid, _myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, _trusteeDid, nymRequest);
        }        

        [TestMethod]
        public async Task TestVerifyWorksForVerkeyCachedInWallet()
        {
            var identityJson = string.Format("{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}", _myDid, _myVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var valid = await Signus.VerifySignatureAsync(wallet, pool, _myDid, MESSAGE, _signature);
            Assert.IsTrue(valid);
        }

        [TestMethod]
        public async Task TestVerifyWorksForGetNymFromLedger()
        {
            var valid = await Signus.VerifySignatureAsync(wallet, pool, _myDid, MESSAGE, _signature);
            Assert.IsTrue(valid);
        }
        
        [TestMethod]
        public async Task TestVerifyWorksForOtherSigner()
        {
            var trusteeIdentityJson = string.Format(IDENTITY_JSON_TEMPLATE, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, trusteeIdentityJson);

            var myIdentityJson = string.Format(IDENTITY_JSON_TEMPLATE, _myDid, _myVerkey);
            await Signus.StoreTheirDidAsync(wallet, myIdentityJson);

            var signature = await Signus.SignAsync(wallet, _trusteeDid, MESSAGE);

            var valid = await Signus.VerifySignatureAsync(wallet, pool, _myDid, MESSAGE, signature);
            Assert.IsFalse(valid);
        }
    }
}
