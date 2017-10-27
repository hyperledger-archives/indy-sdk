using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class DecryptSealedTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private string _trusteeDid;
        private string _trusteeVerkey;
        private string _myDid;
        private string _myVerkey;

        [TestInitialize]
        public async Task Before()
        {
            var trusteeNym = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            _trusteeDid = trusteeNym.Did;
            _trusteeVerkey = trusteeNym.VerKey;

            var myNym = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            _myDid = myNym.Did;
            _myVerkey = myNym.VerKey;

            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);
        }

        [TestMethod]
        public async Task TestDecryptSealedWorks()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _trusteeDid, _trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptedMessage = await Signus.EncryptSealedAsync(wallet, pool, _trusteeDid, MESSAGE);
            var decryptedMessage = await Signus.DecryptSealedAsync(wallet, _trusteeDid, encryptedMessage);

            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
        }

        [TestMethod]
        public async Task TestSealedDecryptSealedWorksForOtherCoder()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, _myDid, _myVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, _myDid, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                    Signus.DecryptSealedAsync(wallet, _trusteeDid, encryptResult)
                );
        }

        [TestMethod]
        public async Task TestDecryptSealedWorksForUnknownMyDid()
        {
            byte[] encryptedMessage = (byte[])(Array)new sbyte[] { -105, 30, 89, 75, 76, 28, -59, -45, 105, -46, 20 };

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.DecryptSealedAsync(wallet, "unknowDid", encryptedMessage)
            );

        }
    }
}
