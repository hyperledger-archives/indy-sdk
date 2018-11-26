using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Hyperledger.Indy.Samples.Utils;
using Hyperledger.Indy.WalletApi;

namespace Hyperledger.Indy.Samples.WalletStorage
{
        public class InMemoryWalletStorage : IWalletStorage
    {
        // Permanent storage
        public Dictionary<string, string> StoredMetadata { get; set; } = new Dictionary<string, string>();
        public List<StorageRecord> StoredRecords { get; set; } = new List<StorageRecord>();

        // In memory handles
        public Dictionary<int, string> MetadataHandles { get; set; } = new Dictionary<int, string>();
        public Dictionary<int, string> StorageHandles { get; set; } = new Dictionary<int, string>();
        public Dictionary<int, StorageRecord> RecordHandles { get; set; } = new Dictionary<int, StorageRecord>();

        public Task CreateAsync(string name, string config, string credentialsJson, string metadata)
        {
            StoredMetadata[name] = metadata;

            return Task.CompletedTask;
        }

        public Task<int> OpenAsync(string name, string config, string credentialsJson)
        {
            var nextStorageHandle = StorageHandles.Keys.MaxOrDefault() + 1;
            StorageHandles[nextStorageHandle] = name;
            return Task.FromResult(nextStorageHandle);
        }

        public Task CloseAsync(int storageHandle)
        {
            StorageHandles.Remove(storageHandle);
            return Task.CompletedTask;
        }

        public Task DeleteAsync(string name, string config, string credentialsJson)
        {
            StoredMetadata.Remove(name);
            return Task.CompletedTask;
        }

        public Task AddRecordAsync(int storageHandle, string type, string id, byte[] value, string tagsJson)
        {
            var record = new StorageRecord
            {
                Id = id,
                Type = type,
                Value = value,
                Tags = tagsJson
            };
            StoredRecords.Add(record);
            return Task.CompletedTask;
        }

        public Task UpdateRecordValueAsync(int storageHandle, string type, string id, byte[] value)
        {
            var record = StoredRecords.Single(x => x.Id == id && x.Type == type);
            record.Value = value;
            return Task.CompletedTask;
        }

        public Task UpdateRecordTagsAsync(int storageHandle, string type, string id, string tagsJson)
        {
            var record = StoredRecords.Single(x => x.Id == id && x.Type == type);
            record.Tags = tagsJson;
            return Task.CompletedTask;
        }

        public Task AddRecordTagsAsync(int storageHandle, string type, string id, string tagsJson)
        {
            return UpdateRecordTagsAsync(storageHandle, type, id, tagsJson);
        }

        public Task DeleteRecordTagsAsync(int storageHandle, string type, string id, string tagNamesJson)
        {
            return UpdateRecordTagsAsync(storageHandle, type, id, null);
        }

        public Task DeleteRecordAsync(int storageHandle, string type, string id)
        {
            var record = StoredRecords.SingleOrDefault(x => x.Id == id && x.Type == type);
            StoredRecords.Remove(record);
            return Task.CompletedTask;
        }

        public Task<int> GetRecordAsync(int storageHandle, string type, string id, string optionsJson)
        {
            var record = StoredRecords.Single(x => x.Id == id && x.Type == type);

            var nextRecordHandle = RecordHandles.Keys.MaxOrDefault() + 1;
            RecordHandles.Add(nextRecordHandle, record);

            return Task.FromResult(nextRecordHandle);
        }

        public Task<string> GetRecordIdAsync(int storageHandle, int recordHandle)
        {
            var id = RecordHandles[recordHandle].Id;
            return Task.FromResult(id);
        }

        public Task<string> GetRecordTypeAsync(int storageHandle, int recordHandle)
        {
            var type = RecordHandles[recordHandle].Type;
            return Task.FromResult(type);
        }

        public Task<byte[]> GetRecordValueAsync(int storageHandle, int recordHandle)
        {
            var value = RecordHandles[recordHandle].Value;
            return Task.FromResult(value);
        }

        public Task<string> GetRecordTagsAsync(int storageHandle, int recordHandle)
        {
            var tags = RecordHandles[recordHandle].Tags;
            return Task.FromResult(tags);
        }

        public Task FreeRecordAsync(int storageHandle, int recordHandle)
        {
            RecordHandles.Remove(recordHandle);
            return Task.CompletedTask;
        }

        public Task<Tuple<string, int>> GetStorageMetadataAsync(int storageHandle)
        {
            var metadata = StoredMetadata[StorageHandles[storageHandle]];

            var nextMetadataHandle = MetadataHandles.Keys.MaxOrDefault() + 1;
            MetadataHandles[nextMetadataHandle] = metadata;

            return Task.FromResult(new Tuple<string, int>(metadata, nextMetadataHandle));
        }

        public Task SetStorageMetadataAsync(int storageHandle, string metadataP)
        {
            var storageName = StorageHandles[storageHandle];
            StoredMetadata[storageName] = metadataP;
            return Task.CompletedTask;
        }

        public Task FreeStorageMetadataAsync(int stroageHandle, int metadataHandle)
        {
            MetadataHandles.Remove(metadataHandle);
            return Task.CompletedTask;
        }

        public Task<int> SearchRecordsAsync(int storageHandle, string type, string queryJson, string optionsJson)
        {
            throw new NotImplementedException();
        }

        public Task<int> SearchAllRecordsAsync(int storageHandle)
        {
            throw new NotImplementedException();
        }

        public Task<int> GetSearchTotalCountAsync(int storageHandle, int searchHandle)
        {
            throw new NotImplementedException();
        }

        public Task<int> FetchSearchNextRecordAsync(int storageHandle, int searchHandle)
        {
            throw new NotImplementedException();
        }

        public Task FreeSearchAsync(int storageHandle, int searchHandle)
        {
            throw new NotImplementedException();
        }
    }
}
