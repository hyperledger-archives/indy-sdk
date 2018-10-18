using System;

namespace Hyperledger.Indy.Test
{
    class WalletUtils
    {
        const string TYPE = "default";

        public static string GetWalletConfig()
        {
            return string.Format("{{\"id\":\"{0}\", \"storage_type\":\"{1}\"}}", GetWalletId(), TYPE);
        }

        public static string GetWalletId()
        {
            return Guid.NewGuid().ToString();

        }
          
    }
}

