using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class DecryptSealedTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private String trusteeDid;
        private String trusteeVerkey;
        private String myDid;
        private String myVerkey;

        [TestInitialize]
        public async Task Before()
        {
            var trusteeNym = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            trusteeDid = trusteeNym.Did;
            trusteeVerkey = trusteeNym.VerKey;

            var myNym = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            myDid = myNym.Did;
            myVerkey = myNym.VerKey;

            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);
        }

        [TestMethod]
        public async Task TestDecryptSealedWorks()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, trusteeDid, trusteeVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptedMessage = await Signus.EncryptSealedAsync(wallet, pool, trusteeDid, MESSAGE);
            var decryptedMessage = await Signus.DecryptSealedAsync(wallet, trusteeDid, encryptedMessage);

            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
        }

        [TestMethod]
        public async Task TestSealedDecryptSealedWorksForOtherCoder()
        {
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, myDid, myVerkey);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var encryptResult = await Signus.EncryptSealedAsync(wallet, pool, myDid, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                    Signus.DecryptSealedAsync(wallet, trusteeDid, encryptResult)
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
