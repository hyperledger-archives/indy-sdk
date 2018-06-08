﻿using Hyperledger.Indy.PoolApi;
using System.IO;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Samples.Utils
{
    static class PoolUtils
    {
        public const string DEFAULT_POOL_NAME = "default_pool";

        public static string CreateGenesisTxnFile(string filename)
        {
            var file = EnvironmentUtils.GetTmpPath(filename);
            var testPoolIp = EnvironmentUtils.GetTestPoolIP();

            var defaultTxns = new string[]{
                string.Format("{{\"reqSignature\":{{}},\"txn\":{{\"data\":{{\"data\":{{\"alias\":\"Node1\",\"blskey\":\"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba\",\"client_ip\":\"{0}\",\"client_port\":9702,\"node_ip\":\"{0}\",\"node_port\":9701,\"services\":[\"VALIDATOR\"]}},\"dest\":\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\"}},\"metadata\":{{\"from\":\"Th7MpTaRZVRYnPiabds81Y\"}},\"type\":\"0\"}},\"txnMetadata\":{{\"seqNo\":1,\"txnId\":\"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62\"}},\"ver\":\"1\"}}", testPoolIp),
                string.Format("{{\"reqSignature\":{{}},\"txn\":{{\"data\":{{\"data\":{{\"alias\":\"Node2\",\"blskey\":\"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk\",\"client_ip\":\"{0}\",\"client_port\":9704,\"node_ip\":\"{0}\",\"node_port\":9703,\"services\":[\"VALIDATOR\"]}},\"dest\":\"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\"}},\"metadata\":{{\"from\":\"EbP4aYNeTHL6q385GuVpRV\"}},\"type\":\"0\"}},\"txnMetadata\":{{\"seqNo\":2,\"txnId\":\"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc\"}},\"ver\":\"1\"}}", testPoolIp),
                string.Format("{{\"reqSignature\":{{}},\"txn\":{{\"data\":{{\"data\":{{\"alias\":\"Node3\",\"blskey\":\"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5\",\"client_ip\":\"{0}\",\"client_port\":9706,\"node_ip\":\"{0}\",\"node_port\":9705,\"services\":[\"VALIDATOR\"]}},\"dest\":\"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\"}},\"metadata\":{{\"from\":\"4cU41vWW82ArfxJxHkzXPG\"}},\"type\":\"0\"}},\"txnMetadata\":{{\"seqNo\":3,\"txnId\":\"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4\"}},\"ver\":\"1\"}}", testPoolIp),
                string.Format("{{\"reqSignature\":{{}},\"txn\":{{\"data\":{{\"data\":{{\"alias\":\"Node4\",\"blskey\":\"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw\",\"client_ip\":\"{0}\",\"client_port\":9708,\"node_ip\":\"{0}\",\"node_port\":9707,\"services\":[\"VALIDATOR\"]}},\"dest\":\"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\"}},\"metadata\":{{\"from\":\"TWwCRQRZ2ZHMJFn9TzLp7W\"}},\"type\":\"0\"}},\"txnMetadata\":{{\"seqNo\":4,\"txnId\":\"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008\"}},\"ver\":\"1\"}}", testPoolIp)
             };

            Directory.CreateDirectory(Path.GetDirectoryName(file));
            var stream = new StreamWriter(file);

            foreach(var defaultTxn in defaultTxns)
            {
                stream.WriteLine(defaultTxn);
            }

            stream.Close();

            return file;
        }

        public static async Task CreatePoolLedgerConfig()
        {
            var genesisTxnFile = CreateGenesisTxnFile("temp.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');
            var createPoolLedgerConfig = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            try
            {
                await Pool.CreatePoolLedgerConfigAsync(DEFAULT_POOL_NAME, createPoolLedgerConfig);
            }
            catch (PoolLedgerConfigExistsException)
            {
                //Swallow expected exception if it happens.
            }
        }

        public static async Task DeletePoolLedgerConfigAsync(string name)
        {
            try
            {
                await Pool.DeletePoolLedgerConfigAsync(name);
            }
            catch(IOException) //TODO: This should be a more specific error when implemented
            {
                //Swallow expected exception if it happens.                
            }
        }
    }
}
