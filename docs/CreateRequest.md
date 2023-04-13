# CreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the index to be created. The maximum length is 45 characters. | 
**dimension** | **i32** | The dimensions of the vectors to be inserted in the index | 
**index_type** | Option<**String**> | The type of vector index. Pinecone supports 'approximated'. | [optional][default to approximated]
**metric** | Option<**String**> | The distance metric to be used for similarity search. You can use 'euclidean', 'cosine', or 'dotproduct'. | [optional][default to cosine]
**pods** | Option<**i32**> | The number of pods for the index to use,including replicas. | [optional][default to 1]
**replicas** | Option<**i32**> | The number of replicas. Replicas duplicate your index. They provide higher availability and throughput. | [optional][default to 1]
**shards** | Option<**i32**> | The number of shards to be used in the index. | [optional][default to 1]
**pod_type** | Option<**String**> | The type of pod to use. One of `s1`, `p1`, or `p2` appended with `.` and one of `x1`, `x2`, `x4`, or `x8`. | [optional][default to p1.x1]
**index_config** | Option<[**crate::models::CreateRequestIndexConfig**](createRequest_index_config.md)> |  | [optional]
**metadata_config** | Option<[**crate::models::CreateRequestMetadataConfig**](createRequest_metadata_config.md)> |  | [optional]
**source_collection** | Option<**String**> | The name of the collection to create an index from | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


