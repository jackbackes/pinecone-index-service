# \IndexOperationsApi

All URIs are relative to *https://controller..pinecone.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**configure_index**](IndexOperationsApi.md#configure_index) | **PATCH** /databases/{indexName} | 
[**create_collection**](IndexOperationsApi.md#create_collection) | **POST** /collections | 
[**create_index**](IndexOperationsApi.md#create_index) | **POST** /databases | 
[**delete_collection**](IndexOperationsApi.md#delete_collection) | **DELETE** /collections/{collectionName} | 
[**delete_index**](IndexOperationsApi.md#delete_index) | **DELETE** /databases/{indexName} | 
[**describe_collection**](IndexOperationsApi.md#describe_collection) | **GET** /collections/{collectionName} | 
[**describe_index**](IndexOperationsApi.md#describe_index) | **GET** /databases/{indexName} | 
[**list_collections**](IndexOperationsApi.md#list_collections) | **GET** /collections | 
[**list_indexes**](IndexOperationsApi.md#list_indexes) | **GET** /databases | 



## configure_index

> String configure_index(index_name, patch_request)


This operation specifies the pod type and number of replicas for an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** | The name of the index | [required] |
**patch_request** | Option<[**PatchRequest**](PatchRequest.md)> | The desired pod type and replica configuration for the index. |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_collection

> String create_collection(create_collection_request)


This operation creates a Pinecone collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_collection_request** | Option<[**CreateCollectionRequest**](CreateCollectionRequest.md)> |  |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_index

> String create_index(create_request)


This operation creates a Pinecone index. You can use it to specify the measure of similarity, the dimension of vectors to be stored in the index, the numbers of shards and replicas to use, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_request** | Option<[**CreateRequest**](CreateRequest.md)> |  |  |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> String delete_collection(collection_name)


This operation deletes an existing collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_index

> String delete_index(index_name)


This operation deletes an existing index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** | The name of the index | [required] |

### Return type

**String**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## describe_collection

> crate::models::CollectionMeta describe_collection(collection_name)


Get a description of a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

[**crate::models::CollectionMeta**](collectionMeta.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## describe_index

> crate::models::IndexMeta describe_index(index_name)


Get a description of an index.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_name** | **String** | The name of the index | [required] |

### Return type

[**crate::models::IndexMeta**](indexMeta.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_collections

> Vec<String> list_collections()


This operation returns a list of your Pinecone collections.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexes

> Vec<String> list_indexes()


This operation returns a list of your Pinecone indexes.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

