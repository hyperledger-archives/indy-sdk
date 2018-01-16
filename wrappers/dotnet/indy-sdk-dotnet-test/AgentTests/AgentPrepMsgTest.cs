using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentPrepMsgTest : IndyIntegrationTestWithSingleWallet
    {
        private async Task CheckMessage(string senderVk, byte[] encryptedMsg)
        {
            await Wallet.CreateWalletAsync(POOL, "walletForCheck", TYPE, null, null);
            var localWallet = await Wallet.OpenWalletAsync("walletForCheck", null, null);

            var didJson = string.Format("{{\"seed\":\"{0}\",\"cid\":true}}", MY2_SEED);
            var result = await Signus.CreateAndStoreMyDidAsync(localWallet, didJson);
            var recipientDid = result.Did;

            var decryptedMessageBytes = await Signus.DecryptSealedAsync(localWallet, recipientDid, encryptedMsg);
            var decryptedMessageJson = Encoding.UTF8.GetString(decryptedMessageBytes);
            var decryptedMsg = JObject.Parse(decryptedMessageJson);


            Assert.IsTrue(decryptedMsg.Value<bool>("auth"));
            Assert.AreEqual(senderVk, decryptedMsg.Value<string>("sender"));
            Assert.IsNotNull(decryptedMsg.Value<string>("nonce"));
            Assert.IsNotNull(decryptedMsg.Value<string>("msg"));
           
            await localWallet.CloseAsync();
            await Wallet.DeleteWalletAsync("walletForCheck", null);
        }

        [TestMethod]
        public async Task TestPrepMsgWorksForCreatedKey()
        {
            var paramJson = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);

            var senderVk = await Crypto.CreateKeyAsync(wallet, paramJson);
            var encryptedMsg = await Agent.PrepMsgAsync(wallet, senderVk, VERKEY_MY2, MESSAGE);

            await CheckMessage(senderVk, encryptedMsg);
        }

        [TestMethod]
        public async Task TestPrepMsgWorksForCreatedDid()
        {
            var didJson = string.Format("{{\"seed\":\"{0}\",\"cid\":false}}", MY1_SEED);

            var result = await Signus.CreateAndStoreMyDidAsync(wallet, didJson);
            var senderVk = result.VerKey;

            var encryptedMsg = await Agent.PrepMsgAsync(wallet, senderVk, VERKEY_MY2, MESSAGE);

            await CheckMessage(senderVk, encryptedMsg);
        }


        [TestMethod]
        public async Task TestPrepMsgWorksForCreatedDidAsCid()
        {
            var didJson = string.Format("{{\"seed\":\"{0}\",\"cid\":true}}", MY1_SEED);

            var result = await Signus.CreateAndStoreMyDidAsync(wallet, didJson);
            var senderVk = result.VerKey;

            var encryptedMsg = await Agent.PrepMsgAsync(wallet, senderVk, VERKEY_MY2, MESSAGE);

            await CheckMessage(senderVk, encryptedMsg);
        }

        [TestMethod]
        public async Task TestPrepMsgWorksForUnknownSenderVerkey()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Agent.PrepMsgAsync(wallet, VERKEY, VERKEY_MY2, MESSAGE)
            );
        }

        [TestMethod]
        public async Task TestPrepMsgWorksForInvalidRecipientVk()
        {
            var paramJson = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);
            var senderVk = await Crypto.CreateKeyAsync(wallet, paramJson);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Agent.PrepMsgAsync(wallet, senderVk, INVALID_VERKEY, MESSAGE)
            );
        }
    }
}
