using System;
using Bitwarden.Sdk;

var identityUrl = Environment.GetEnvironmentVariable("BW_IDENTITY_URL");
var apiUrl = Environment.GetEnvironmentVariable("BW_API_URL");
var organizationId = Guid.Parse(Environment.GetEnvironmentVariable("BW_ORGANIZATION_ID"));
var accessToken = Environment.GetEnvironmentVariable("BW_ACCESS_TOKEN");
var projectId = Guid.Parse(Environment.GetEnvironmentVariable("BW_PROJECT_ID"));

BitwardenSettings? settings = new BitwardenSettings
{
    ApiUrl = apiUrl,
    IdentityUrl = identityUrl,
};

using var bitwardenClient = new BitwardenClient(settings);
bitwardenClient.AccessTokenLogin(accessToken.ToString());

var key = "Secret Key";
var value = "Secret Value";
var note = "Secret Note";

var secret = bitwardenClient.Secrets.Create(key, value, note, organizationId, new[] { projectId });

Console.Write($"Secret Created!\n\n  Key: {secret.Key}\nValue: {secret.Value}\n Note: {secret.Note}");
