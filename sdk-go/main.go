package main

import (
	"fmt"
	"os"

	sdk "github.com/bitwarden/sdk-go"
)

func main() {
	identityUrl := os.Getenv("BW_IDENTITY_URL")
	apiUrl := os.Getenv("BW_API_URL")
	accessToken := os.Getenv("BW_ACCESS_TOKEN")
	organizationId := os.Getenv("BW_ORGANIZATION_ID")
	projectId := os.Getenv("BW_PROJECT_ID")

	bitwardenClient, err := sdk.NewBitwardenClient(&apiUrl, &identityUrl)
	if err != nil {
		panic(err)
	}

	err = bitwardenClient.AccessTokenLogin(accessToken, nil)
	if err != nil {
		panic(err)
	}

	secret, err := bitwardenClient.Secrets().Create("Secret Key", "Secret Value", "Secret Note", organizationId, []string{projectId})
	if err != nil {
		panic(err)
	}

	fmt.Printf("Secret Created!\n\n  Key: %v\nValue: %v\n Note: %v\n", secret.Key, secret.Value, secret.Note)

	defer bitwardenClient.Close()
}
