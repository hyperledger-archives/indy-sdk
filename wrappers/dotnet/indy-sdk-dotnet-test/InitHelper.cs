using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    class InitHelper
    {
        private static bool _isInitialized = false;

        public static async Task InitAsync()
        {
            if (_isInitialized)
                return;

            _isInitialized = true;
        }

    }
}
