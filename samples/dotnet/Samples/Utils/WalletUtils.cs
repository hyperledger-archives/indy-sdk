using Hyperledger.Indy.WalletApi;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples.Utils
{
    static class WalletUtils
    {
        public static async Task CreateWalleatAsync(string poolName, string name, string type, string config, string credentials)
        {
            try
            {
                await Wallet.CreateWalletAsync(poolName, name, type, config, credentials);
            }
            catch (IndyException e)
            {
                if (e.ErrorCode != ErrorCode.WalletAlreadyExistsError)
                    throw;
            }
        }
    }
}
