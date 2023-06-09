# Rust API client for index_service

defaultDescription


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `index_service` and add the following to `Cargo.toml` under `[dependencies]`:

```
index_service = { path = "./index_service" }
```

## Documentation for API Endpoints

All URIs are relative to *https://controller..pinecone.io*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*IndexOperationsApi* | [**configure_index**](docs/IndexOperationsApi.md#configure_index) | **PATCH** /databases/{indexName} | 
*IndexOperationsApi* | [**create_collection**](docs/IndexOperationsApi.md#create_collection) | **POST** /collections | 
*IndexOperationsApi* | [**create_index**](docs/IndexOperationsApi.md#create_index) | **POST** /databases | 
*IndexOperationsApi* | [**delete_collection**](docs/IndexOperationsApi.md#delete_collection) | **DELETE** /collections/{collectionName} | 
*IndexOperationsApi* | [**delete_index**](docs/IndexOperationsApi.md#delete_index) | **DELETE** /databases/{indexName} | 
*IndexOperationsApi* | [**describe_collection**](docs/IndexOperationsApi.md#describe_collection) | **GET** /collections/{collectionName} | 
*IndexOperationsApi* | [**describe_index**](docs/IndexOperationsApi.md#describe_index) | **GET** /databases/{indexName} | 
*IndexOperationsApi* | [**list_collections**](docs/IndexOperationsApi.md#list_collections) | **GET** /collections | 
*IndexOperationsApi* | [**list_indexes**](docs/IndexOperationsApi.md#list_indexes) | **GET** /databases | 


## Documentation For Models

 - [ApproximatedConfig](docs/ApproximatedConfig.md)
 - [CollectionMeta](docs/CollectionMeta.md)
 - [CreateCollectionRequest](docs/CreateCollectionRequest.md)
 - [CreateRequest](docs/CreateRequest.md)
 - [CreateRequestIndexConfig](docs/CreateRequestIndexConfig.md)
 - [CreateRequestMetadataConfig](docs/CreateRequestMetadataConfig.md)
 - [HnswConfig](docs/HnswConfig.md)
 - [IndexMeta](docs/IndexMeta.md)
 - [IndexMetaDatabase](docs/IndexMetaDatabase.md)
 - [IndexMetaDatabaseIndexConfig](docs/IndexMetaDatabaseIndexConfig.md)
 - [IndexMetaDatabaseMetadataConfig](docs/IndexMetaDatabaseMetadataConfig.md)
 - [IndexMetaStatus](docs/IndexMetaStatus.md)
 - [PatchRequest](docs/PatchRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



