using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples
{
    static class AgentDemo
    {
        private readonly static byte[] MESSAGE = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
        private const string ALICE_WALLET = "alice_wallet";
        private const string BOB_WALLET = "bob_wallet";
        private const string POOL_NAME = "no pool";

        public static async Task Execute()
        {
            Console.WriteLine("Agent sample -> started");

            Wallet aliceWallet = null;
            Wallet bobWallet = null;

            try
            {
                // 1. Create and open wallets for Alice and Bob
                await Wallet.CreateWalletAsync(POOL_NAME, ALICE_WALLET, null, null, null);
                aliceWallet = await Wallet.OpenWalletAsync(ALICE_WALLET, null, null);
                await Wallet.CreateWalletAsync(POOL_NAME, BOB_WALLET, null, null, null);
                bobWallet = await Wallet.OpenWalletAsync(BOB_WALLET, null, null);

                // 2. Create keys for Alice and Bob
                var aliceKey = await Crypto.CreateKeyAsync(aliceWallet, "{}");
                var bobKey = await Crypto.CreateKeyAsync(bobWallet, "{}");

                // 3. Prepare authenticated message from Alice to Bob
                var encryptedAuthMsg = await Agent.PrepMsgAsync(aliceWallet, aliceKey, bobKey, MESSAGE);

                // 4. Parse authenticated message on Bob's side
                {
                    var decryptedAuth = await Agent.ParseMsgAsync(bobWallet, bobKey, encryptedAuthMsg);
                    Debug.Assert(aliceKey.Equals(decryptedAuth.SenderKey));
                    Debug.Assert(MESSAGE.SequenceEqual(decryptedAuth.Msg));
                }

                // 5. Prepare anonymous message from Bob to Alice
                var encryptedAnonMsg = await Agent.PrepAnonymousMsgAsync(aliceKey, MESSAGE);

                // 6. Parse anonymous message on Alice's side
                {
                    var decryptedAnon = await Agent.ParseMsgAsync(aliceWallet, aliceKey, encryptedAnonMsg);
                    Debug.Assert(decryptedAnon.SenderKey == null);
                    Debug.Assert(MESSAGE.SequenceEqual(decryptedAnon.Msg));
                }
            }
            finally
            {
                //7. Close and delete wallets
                if (aliceWallet != null)
                    await aliceWallet.CloseAsync();
                
                await WalletUtils.DeleteWalletAsync(ALICE_WALLET, null);

                if (bobWallet != null)
                    await bobWallet.CloseAsync();

                await WalletUtils.DeleteWalletAsync(BOB_WALLET, null);
             }

            Console.WriteLine("Agent sample -> completed");
        }
    }
}
