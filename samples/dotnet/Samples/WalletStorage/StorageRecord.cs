using System;
using System.Collections.Generic;
using System.Text;

namespace Hyperledger.Indy.Samples.WalletStorage
{
    public class StorageRecord
    {
        public string Id { get; set; }

        public string Type { get; set; }

        public byte[] Value { get; set; }

        public string Tags { get; set; }
    }
}
