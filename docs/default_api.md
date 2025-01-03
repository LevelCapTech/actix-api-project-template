# default_api

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
**getIndex**](default_api.md#getIndex) | **GET** / | メインエンドポイント
****](default_api.md#) | **GET** /items | アイテム一覧取得
****](default_api.md#) | **POST** /items | 新規アイテム作成
****](default_api.md#) | **DELETE** /items/{id} | アイテム削除
****](default_api.md#) | **GET** /items/{id} | アイテム取得
****](default_api.md#) | **PUT** /items/{id} | アイテム更新


# **getIndex**
> models::GetIndex200Response getIndex()
メインエンドポイント

シンプルなJSONレスポンスを返します。

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::GetIndex200Response**](getIndex_200_response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::ItemsGet200ResponseInner> ()
アイテム一覧取得

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::ItemsGet200ResponseInner>**](_items_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::ItemsPost201Response (items_post_request)
新規アイテム作成

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **items_post_request** | [**ItemsPostRequest**](ItemsPostRequest.md)|  | 

### Return type

[**models::ItemsPost201Response**](_items_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> (id)
アイテム削除

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i32**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::ItemsGet200ResponseInner (id)
アイテム取得

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i32**|  | 

### Return type

[**models::ItemsGet200ResponseInner**](_items_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> models::ItemsIdPut200Response (id, items_id_put_request)
アイテム更新

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i32**|  | 
  **items_id_put_request** | [**ItemsIdPutRequest**](ItemsIdPutRequest.md)|  | 

### Return type

[**models::ItemsIdPut200Response**](_items__id__put_200_response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

