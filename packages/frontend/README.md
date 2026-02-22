# Frontend

A vue app for site created to make answering and managing helmet reading challenges easier. Hosting is easiest done by uploading to azure blob storage and enabling static website hosting.

## Required tools

* Node

## Setup

Run the following commands

```sh
npm run install
```

To start a development server

```sh
npm run dev
```

## Deployment instructions for frontend

Copy .env to .env.production and set API url to where your API is hosted. Build dist and upload to blob storage where static website hosting is enabled. Example commands given below.

Requires AzCopy locally installed. [See instructions at time of writing](https://learn.microsoft.com/en-us/azure/storage/common/storage-use-azcopy-v10?tabs=dnf)

```sh
npm run build
az storage blob sync -c \$web --account-name mystorageccount -s dist
```