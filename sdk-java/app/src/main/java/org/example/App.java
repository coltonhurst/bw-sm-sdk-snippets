package org.example;

import java.lang.System;
import java.util.UUID;

import com.bitwarden.sdk.*;
import com.bitwarden.sdk.schema.*;

public class App {
    public static void main(String[] args) {
        String identityUrl = System.getenv("BW_IDENTITY_URL");
        String apiUrl = System.getenv("BW_API_URL");
        UUID organizationId = UUID.fromString(System.getenv("BW_ORGANIZATION_ID"));
        String accessToken = System.getenv("BW_ACCESS_TOKEN");
        UUID projectId = UUID.fromString(System.getenv("BW_PROJECT_ID"));
        
        BitwardenSettings bitwardenSettings = new BitwardenSettings();
        bitwardenSettings.setApiUrl(apiUrl);
        bitwardenSettings.setIdentityUrl(identityUrl);

        BitwardenClient client = new BitwardenClient(bitwardenSettings);
        client.accessTokenLogin(accessToken);

        SecretResponse secret = client.secrets().create("Secret Key", "Secret Value", "Secret Note", organizationId, new UUID[] { projectId });

        System.out.println(String.format("Secret Created!\n\n  Key: %s\nValue: %s\n Note: %s\n", secret.getKey(), secret.getValue(), secret.getNote()));
    }
}
