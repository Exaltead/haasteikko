package shared

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"

	"github.com/Azure/azure-sdk-for-go/sdk/azcore"
	"github.com/Azure/azure-sdk-for-go/sdk/azcore/runtime"
	"github.com/Azure/azure-sdk-for-go/sdk/azidentity"

	"github.com/Azure/azure-sdk-for-go/sdk/data/azcosmos"
)

func GetClient() (*azcosmos.Client, error) {
	endpoint := os.Getenv("COSMOS_ENDPOINT")
	if endpoint == "" {
		return nil, errors.New("COSMOS_ENDPOINT environment variable is not set")
	}
	cred, err := azidentity.NewDefaultAzureCredential(nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create a credential: %v", err)
	}
	return azcosmos.NewClient(endpoint, cred, nil)
}

func ConnectToCosmosContainer(containerName string) (*azcosmos.ContainerClient, error) {

	databaseName := os.Getenv("DATABASE_NAME")

	client, err := GetClient()

	if err != nil {
		return nil, fmt.Errorf("failed to create a client from connection string %v", err)
	}
	database, err := client.NewDatabase(databaseName)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to database %v", err)
	}

	container, err := database.NewContainer(containerName)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to container %v", err)
	}

	return container, nil
}

func LoadDataFromPager[T any](pager *runtime.Pager[azcosmos.QueryItemsResponse]) ([]T, error) {
	items := []T{}

	for pager.More() {
		response, err := pager.NextPage(context.TODO())
		if err != nil {
			return nil, err
		}

		for _, bytes := range response.Items {
			item := new(T)
			err := json.Unmarshal(bytes, &item)
			if err != nil {
				return nil, err
			}
			items = append(items, *item)
		}
	}

	return items, nil
}

func QueryAll[T any](containerName string, partitionKey string, query string, queryOptions azcosmos.QueryOptions) ([]T, error) {
	container, err := ConnectToCosmosContainer(containerName)
	if err != nil {
		return nil, err
	}

	var cosmosPartitionKey azcosmos.PartitionKey
	if partitionKey == "" {
		cosmosPartitionKey = azcosmos.NewPartitionKey()
	} else {
		cosmosPartitionKey = azcosmos.NewPartitionKeyString(partitionKey)
	}

	pager := container.NewQueryItemsPager(query, cosmosPartitionKey, &queryOptions)
	if contents, err := LoadDataFromPager[T](pager); err == nil {
		return contents, err
	} else {
		return nil, err
	}
}

type SingleItemResponse[T any] struct {
	Documents []T `json:"Documents"`
}

func QuerySingle[T any](containerName string, id string, partitionKey string) (T, error) {
	container, err := ConnectToCosmosContainer(containerName)

	if err != nil {
		return *new(T), err
	}

	partitionKeyValue := azcosmos.NewPartitionKeyString(partitionKey)

	resp, err := container.ReadItem(context.TODO(), partitionKeyValue, id, nil)

	if err != nil {
		var err2 *azcore.ResponseError
		if errors.As(err, &err2) {
			return *new(T), errors.New("item not found")
		}

		log.Default().Println("Failed to read item", err)
		return *new(T), fmt.Errorf("failed to read a item %v", err)
	}

	if resp.RawResponse.StatusCode == 404 {
		return *new(T), errors.New("item not found")
	}

	item := new(T)
	if err := json.Unmarshal(resp.Value, &item); err != nil {
		return *new(T), fmt.Errorf("failed to unmarshal item %v", err)
	}

	return *item, nil
}

func DeleteItem(containerName string, id string, partitionKey string) error {
	container, err := ConnectToCosmosContainer(containerName)
	if err != nil {
		return err
	}

	partitionKeyValue := azcosmos.NewPartitionKeyString(partitionKey)

	resp, err := container.DeleteItem(context.TODO(), partitionKeyValue, id, nil)

	if err != nil || resp.RawResponse.StatusCode != 204 {
		return fmt.Errorf("failed to delete item %v", err)
	}
	return nil
}

func UpsertItem[T any](containerName string, item T, partitionKey string) error {
	container, err := ConnectToCosmosContainer(containerName)
	if err != nil {
		return err
	}
	partitionKeyValue := azcosmos.NewPartitionKeyString(partitionKey)

	itemBytes, err := json.Marshal(item)
	if err != nil {
		return fmt.Errorf("failed to marshal item %v", err)
	}

	_, err = container.UpsertItem(context.TODO(), partitionKeyValue, itemBytes, nil)

	if err != nil {
		return fmt.Errorf("failed to upsert item %v", err)
	}

	return nil
}
