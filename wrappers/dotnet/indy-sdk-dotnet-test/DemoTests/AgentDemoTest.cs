using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class AgentDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentDemo()
        {
            // 1. Create and open wallets for Alice and Bob
            await Wallet.CreateWalletAsync("no pool", "alice_wallet", null, null, null);
            var aliceWallet = await Wallet.OpenWalletAsync("alice_wallet", null, null);
            await Wallet.CreateWalletAsync("no pool", "bob_wallet", null, null, null);
            var bobWallet = await Wallet.OpenWalletAsync("bob_wallet", null, null);

            // 2. Create keys for Alice and Bob
            var aliceKey = await Crypto.CreateKeyAsync(aliceWallet, "{}");
            var bobKey = await Crypto.CreateKeyAsync(bobWallet, "{}");

            // 3. Prepare authenticated message from Alice to Bob
            var encryptedAuthMsg = await Agent.PrepMsgAsync(aliceWallet, aliceKey, bobKey, MESSAGE);

            // 4. Parse authenticated message on Bob's side
            {
                var decryptedAuth = await Agent.ParseMsgAsync(bobWallet, bobKey, encryptedAuthMsg);
                Assert.AreEqual(aliceKey, decryptedAuth.SenderKey);
                Assert.IsTrue(MESSAGE.SequenceEqual(decryptedAuth.Msg));
            }

            // 5. Prepare anonymous message from Bob to Alice
            var encryptedAnonMsg = await Agent.PrepAnonymousMsgAsync(aliceKey, MESSAGE);

            // 6. Parse anonymous message on Alice's side
            {
                var decryptedAnon = await Agent.ParseMsgAsync(aliceWallet, aliceKey, encryptedAnonMsg);
                Assert.IsNull(decryptedAnon.SenderKey);
                Assert.IsTrue(MESSAGE.SequenceEqual(decryptedAnon.Msg));
            }
        }
    }
}
