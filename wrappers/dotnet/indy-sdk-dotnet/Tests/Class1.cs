using Indy.Sdk.Dotnet.Api;
using System;

namespace Indy.Sdk.Dotnet
{
    class Program
    {
        private const string TRUSTEE_DID = "V4SGRU86Z58d6TV7PBUe6f";
        private const string TRUSTEE_VERKEY = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
        private const string TRUSTEE_SEED = "000000000000000000000000Trustee1";
        
        static void Main(string[] args)
        {
            try
            {
                Ledger.CreateConfigAsync("11347-04.txn", "{\"genesis_txn\":\"11347-04.txn\"}").Wait();
            }
            catch (AggregateException e)
            {
                e.Handle((x) =>
                {
                    if (x is SovrinException) // This we know how to handle.
                    {
                        if (((SovrinException)x).ErrorCode == (int)ErrorCode.PoolLedgerNotCreatedError)
                            return true;
                    }
                    return false; // Let anything else stop the application.
                });
            }

            var ledger = Ledger.OpenAsync("11347-04", "{\"refreshOnOpen\":true}").Result;

            try
            {
                Wallet.CreateAsync("11347-04", "trusteewallet", "default", null, null).Wait();
            }
            catch (AggregateException e)
            {
                e.Handle((x) =>
                {
                    if (x is SovrinException) // This we know how to handle.
                    {
                        if (((SovrinException)x).ErrorCode == (int)ErrorCode.WalletAlreadyExistsError)
                            return true;
                    }
                    return false; // Let anything else stop the application.
                });
            }

            var wallet = Wallet.OpenAsync("trusteewallet", null, null).Result;
            var storeDidResult = wallet.CreateAndStoreMyDidAsync("{\"seed\":\"" + TRUSTEE_SEED + "\"}").Result;

            var nymRequest = Ledger.BuildNymRequestAsync(TRUSTEE_VERKEY, TRUSTEE_DID, TRUSTEE_VERKEY, null, SovrinConstants.ROLE_TRUSTEE).Result;
            var signedNymRequest = wallet.SignAsync(TRUSTEE_DID, nymRequest).Result;
            var result = ledger.SubmitRequestAsync(signedNymRequest).Result;

            wallet.CloseAsync().Wait();
            ledger.CloseAsync().Wait();
        }
    }
}
