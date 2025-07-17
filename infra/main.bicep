param appName string

@secure()
param authClientId string
@secure()
param authClientSecret string

param openIdConnectWellKnownConfiguration string = 'https://haasteikko.ciamlogin.com/haasteikko.onmicrosoft.com/v2.0/.well-known/openid-configuration'

var uniqueSuffix = uniqueString(resourceGroup().id)
var location = resourceGroup().location
var hostingAccountName = substring('${appName}hs${uniqueSuffix}', 0, 23)

resource keyVault 'Microsoft.KeyVault/vaults@2024-12-01-preview' = {
  name: substring('kv${appName}${uniqueSuffix}', 0, 23)
  location: location
  properties: {
    sku: {
      family: 'A'
      name: 'standard'
    }
    tenantId: subscription().tenantId
    accessPolicies: []
    enableRbacAuthorization: true
  }

  resource clientSecretKey 'secrets' = {
    name: 'ClientSecret'
    properties: {
      contentType: 'text/plain'
      value: authClientSecret
    }
  }
}

resource databaseAccount 'Microsoft.DocumentDB/databaseAccounts@2024-12-01-preview' = {
  name: '${appName}-db-${uniqueSuffix}'
  location: location
  kind: 'GlobalDocumentDB'
  properties: {
    publicNetworkAccess: 'Enabled'
    enableAutomaticFailover: false
    enableMultipleWriteLocations: false
    isVirtualNetworkFilterEnabled: false
    virtualNetworkRules: []
    disableKeyBasedMetadataWriteAccess: false
    enableFreeTier: false
    enableAnalyticalStorage: false
    analyticalStorageConfiguration: {
      schemaType: 'WellDefined'
    }
    createMode: 'Default'
    databaseAccountOfferType: 'Standard'
    enableMaterializedViews: false
    capacityMode: 'Serverless'
    defaultIdentity: 'FirstPartyIdentity'
    networkAclBypass: 'None'
    disableLocalAuth: false
    enablePartitionMerge: false
    enablePerRegionPerPartitionAutoscale: false
    enableBurstCapacity: false
    enablePriorityBasedExecution: false
    minimalTlsVersion: 'Tls12'
    consistencyPolicy: {
      defaultConsistencyLevel: 'Session'
      maxIntervalInSeconds: 5
      maxStalenessPrefix: 100
    }
    locations: [
      {
        locationName: location
        failoverPriority: 0
        isZoneRedundant: false
      }
    ]
    cors: []
    capabilities: []
    ipRules: []
    backupPolicy: {
      type: 'Continuous'
      continuousModeProperties: {
        tier: 'Continuous7Days'
      }
    }
    networkAclBypassResourceIds: []
    diagnosticLogSettings: {
      enableFullTextQuery: 'None'
    }
    capacity: {
      totalThroughputLimit: 4000
    }
  }
}

resource databaseStorage 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases@2024-12-01-preview' = {
  parent: databaseAccount
  name: '${appName}-db'
  properties: {
    resource: {
      id: '${appName}-db'
    }
  }
}

resource userContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-12-01-preview' = {
  parent: databaseStorage
  name: 'users'
  properties: {
    resource: {
      id: 'users'
      indexingPolicy: {
        automatic: true
        indexingMode: 'consistent'
        includedPaths: [
          {
            path: '/*'
          }
        ]
        excludedPaths: [
          {
            path: '/"_etag"/?'
          }
        ]
      }
      partitionKey: {
        paths: [
          '/id'
        ]
        kind: 'Hash'
        version: 2
      }
    }
  }
}

resource answersCosmosContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-12-01-preview' = {
  parent: databaseStorage
  name: 'answers'
  properties: {
    resource: {
      id: 'answers'
      indexingPolicy: {
        indexingMode: 'consistent'
        automatic: true
        includedPaths: [
          {
            path: '/*'
          }
        ]
        excludedPaths: [
          {
            path: '/"_etag"/?'
          }
        ]
      }
      partitionKey: {
        paths: [
          '/userId'
        ]
        kind: 'Hash'
        version: 2
      }
    }
  }
}

resource challengesCosmosContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-12-01-preview' = {
  parent: databaseStorage
  name: 'challenges'
  properties: {
    resource: {
      id: 'challenges'
      indexingPolicy: {
        indexingMode: 'consistent'
        automatic: true
        includedPaths: [
          {
            path: '/*'
          }
        ]
        excludedPaths: [
          {
            path: '/"_etag"/?'
          }
        ]
      }
      partitionKey: {
        paths: [
          '/kind'
        ]
        kind: 'Hash'
        version: 2
      }
    }
  }
}

resource libraryCosmosContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-12-01-preview' = {
  parent: databaseStorage
  name: 'library'
  properties: {
    resource: {
      id: 'library'
      indexingPolicy: {
        indexingMode: 'consistent'
        automatic: true
        includedPaths: [
          {
            path: '/*'
          }
        ]
        excludedPaths: [
          {
            path: '/"_etag"/?'
          }
        ]
      }
      partitionKey: {
        paths: [
          '/userId'
        ]
        kind: 'Hash'
        version: 2
      }
    }
  }
}

resource solutionsCosmosContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-12-01-preview' = {
  parent: databaseStorage
  name: 'solutions'
  properties: {
    resource: {
      id: 'solutions'
      indexingPolicy: {
        indexingMode: 'consistent'
        automatic: true
        includedPaths: [
          {
            path: '/*'
          }
        ]
        excludedPaths: [
          {
            path: '/"_etag"/?'
          }
        ]
      }
      partitionKey: {
        paths: [
          '/userId'
        ]
        kind: 'Hash'
        version: 2
      }
    }
  }
}

resource hostingStorageAccount 'Microsoft.Storage/storageAccounts@2024-01-01' = {
  name: hostingAccountName
  location: location
  kind: 'StorageV2'
  sku: {
    name: 'Standard_LRS'
  }
  properties: {
    supportsHttpsTrafficOnly: true
  }
}

resource hostingStorageAccountBlobService 'Microsoft.Storage/storageAccounts/blobServices@2024-01-01' = {
  parent: hostingStorageAccount
  name: 'default'
}

resource flexBackendContainer 'Microsoft.Storage/storageAccounts/blobServices/containers@2024-01-01' = {
  parent: hostingStorageAccountBlobService
  name: 'flexbackend'
  properties: {
    publicAccess: 'None'
  }
}

resource webContainer 'Microsoft.Storage/storageAccounts/blobServices/containers@2024-01-01' = {
  parent: hostingStorageAccountBlobService
  name: '$web'
}

resource logAnalytics 'Microsoft.OperationalInsights/workspaces@2025-02-01' = {
  name: '${appName}-la-${uniqueSuffix}'
  location: location
  properties: {
    retentionInDays: 30
    sku: {
      name: 'Standalone'
    }
  }
}

resource applicationInsights 'Microsoft.Insights/components@2020-02-02' = {
  name: '${appName}-ai-${uniqueSuffix}'
  location: location
  kind: 'web'
  properties: {
    Application_Type: 'web'
    WorkspaceResourceId: logAnalytics.id
    DisableLocalAuth: true
  }
}

resource keyVaultRoleDefinition 'Microsoft.Authorization/roleDefinitions@2022-04-01' existing = {
  name: '4633458b-17de-408a-b874-0445c86b69e6'
  scope: keyVault
}

resource flexPlan 'Microsoft.Web/serverfarms@2024-04-01' = {
  name: '${appName}-plan-flex-${uniqueSuffix}'
  location: location
  kind: 'functionapp,linux'
  sku: {
    tier: 'FlexConsumption'
    name: 'FC1'
  }
  properties: {
    reserved: true
  }
}

resource flexFunctionApp 'Microsoft.Web/sites@2024-04-01' = {
  name: '${appName}-flex-api-${uniqueSuffix}'
  location: location
  kind: 'functionapp,linux'
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    reserved: true
    serverFarmId: flexPlan.id
    httpsOnly: true
    siteConfig: {
      appSettings: [
        {
          name: 'AzureWebJobsStorage__accountName'
          value: hostingStorageAccount.name
        }
        {
          name: 'COSMOS_CONNECTION_STRING'
          value: listConnectionStrings(databaseAccount.id, '2019-12-12').connectionStrings[0].connectionString
        }
        { name: 'DATABASE_NAME', value: databaseStorage.name }
        {
          name: 'SECRET_KEY'
          value: '@Microsoft.KeyVault(VaultName=${keyVault.name};SecretName=${keyVault::clientSecretKey.name})'
        }
        { name: 'APPLICATIONINSIGHTS_CONNECTION_STRING', value: applicationInsights.properties.ConnectionString }
      ]
      cors: {
        #disable-next-line BCP329
        allowedOrigins: [
          substring(
            hostingStorageAccount.properties.primaryEndpoints.web,
            0,
            #disable-next-line BCP329
            length(hostingStorageAccount.properties.primaryEndpoints.web) - 1
          )
        ]
      }
    }
    functionAppConfig: {
      scaleAndConcurrency: {
        maximumInstanceCount: 40
        instanceMemoryMB: 2048
      }
      runtime: {
        name: 'custom'
        version: '1.0'
      }
      deployment: {
        storage: {
          type: 'blobContainer'
          value: '${hostingStorageAccount.properties.primaryEndpoints.blob}${flexBackendContainer.name}'
          authentication: { type: 'SystemAssignedIdentity' }
        }
      }
    }
  }
}

resource roleAssignmentAppInsights 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(subscription().id, applicationInsights.id, flexFunctionApp.id, 'Monitoring Metrics Publisher')
  scope: applicationInsights
  properties: {
    roleDefinitionId: subscriptionResourceId(
      'Microsoft.Authorization/roleDefinitions',
      '3913510d-42f4-4e42-8a64-420c390055eb'
    )
    principalId: flexFunctionApp.identity.principalId
    principalType: 'ServicePrincipal'
  }
}

resource flexAppToKeyVault 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(keyVault.id, flexFunctionApp.id, 'Key Vault Secrets User')
  scope: keyVault
  properties: {
    principalId: flexFunctionApp.identity.principalId
    principalType: 'ServicePrincipal'
    roleDefinitionId: keyVaultRoleDefinition.id
  }
}

var storageRoleDefinitionId = 'b7e6dc6d-f1e8-4753-8033-0f276bb0955b' //Storage Blob Data Owner role

resource flexAppToStorageAccount 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  scope: hostingStorageAccount
  name: guid(flexFunctionApp.id, hostingStorageAccount.id, 'Storage Blob Data Owner')
  properties: {
    principalId: flexFunctionApp.identity.principalId
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', storageRoleDefinitionId)
    principalType: 'ServicePrincipal'
  }
}

resource flexFunctionAppAuthSettings 'Microsoft.Web/sites/config@2022-09-01' = {
  parent: flexFunctionApp
  name: 'authsettingsV2'
  properties: {
    platform: {
      enabled: true
      runtimeVersion: '~1'
    }
    globalValidation: {
      requireAuthentication: true
      unauthenticatedClientAction: 'Return401'
    }
    identityProviders: {
      customOpenIdConnectProviders: {
        registration: {
          enabled: true
          registration: {
            clientId: authClientId
            clientCredential: { clientSecretSettingName: 'SECRET_KEY' }
            openIdConnectConfiguration: { wellKnownOpenIdConfiguration: openIdConnectWellKnownConfiguration }
          }
        }
      }
    }
  }
}
