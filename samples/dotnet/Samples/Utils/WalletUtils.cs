using Hyperledger.Indy.WalletApi;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples.Utils
{
    static class WalletUtils
    {
        public static async Task CreateWalletAsync(string config, string credentials)
        {
            try
            {
                await Wallet.CreateWalletAsync(config, credentials);
            }
            catch (WalletExistsException)
            {
                //Swallow expected exception if it happens.
            }
        }

        public static async Task DeleteWalletAsync(string config, string credentials)
        {
            try
            {
                await Wallet.DeleteWalletAsync(config, credentials);
            }
            catch (IOException) //TODO: This should be a more specific error when implemented
            {
                //Swallow expected exception if it happens.

            }
        }

    }
}
