# Rust API client for DaDaPushClient

DaDaPush: Real-time Notifications App Send real-time notifications through our API without coding and maintaining your own app for iOS or Android devices.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen
For more information, please visit [https://www.dadapush.com](https://www.dadapush.com)

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://www.dadapush.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DaDaPushMessageApi* | [**create_message**](docs/DaDaPushMessageApi.md#create_message) | **Post** /api/v1/message | push Message to a Channel
*DaDaPushMessageApi* | [**delete_message**](docs/DaDaPushMessageApi.md#delete_message) | **Delete** /api/v1/message/{messageId} | delete a Channel Message
*DaDaPushMessageApi* | [**get_message**](docs/DaDaPushMessageApi.md#get_message) | **Get** /api/v1/message/{messageId} | get a Channel Message
*DaDaPushMessageApi* | [**get_messages**](docs/DaDaPushMessageApi.md#get_messages) | **Get** /api/v1/messages | get Message List


## Documentation For Models

 - [Action](docs/Action.md)
 - [MessageObject](docs/MessageObject.md)
 - [MessagePushRequest](docs/MessagePushRequest.md)
 - [MessagePushResponse](docs/MessagePushResponse.md)
 - [PageResponseOfMessageObject](docs/PageResponseOfMessageObject.md)
 - [Result](docs/Result.md)
 - [ResultOfMessageObject](docs/ResultOfMessageObject.md)
 - [ResultOfMessagePushResponse](docs/ResultOfMessagePushResponse.md)
 - [ResultOfPageResponseOfMessageObject](docs/ResultOfPageResponseOfMessageObject.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

contacts@dadapush.com
