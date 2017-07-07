using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Wrapper
{
    public class CreateAndStoreMyDidResult
    {
        public CreateAndStoreMyDidResult(string did, string verKey, string pk)
        {
            Did = did;
            VerKey = verKey;
            Pk = pk;
        }

        public string Did { get; }
        public string VerKey { get; }
        public string Pk { get; }

    }

    public class ReplaceKeysResult
    {
        public ReplaceKeysResult(string verKey, string pk)
        {
            VerKey = verKey;
            Pk = pk;
        }
        public string VerKey { get; }
        public string Pk { get; }

    }

    public class EncryptResult
    {
        public EncryptResult(string encryptedMsg, string nonce)
        {
            EncryptedMsg = encryptedMsg;
            Nonce = nonce;
        }
        public string EncryptedMsg { get; }
        public string Nonce { get; }

    }
}
