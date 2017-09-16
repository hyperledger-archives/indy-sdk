using Hyperledger.Indy.WalletApi;
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

        public static async Task DeleteWalletAsync(string name, string credentials)
        {
            try
            {
                await Wallet.DeleteWalletAsync(name, credentials);
            }
            catch (IndyException e)
            {
                if (e.ErrorCode != ErrorCode.CommonIOError) //TODO: This should be a more specific error when implemented
                    throw;
            }
        }

    }
}
