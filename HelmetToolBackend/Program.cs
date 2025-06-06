using HelmetToolBackend.Auth;
using HelmetToolBackend.ChallengeAnswers;
using HelmetToolBackend.Challenges;
using HelmetToolBackend.Library;
using HelmetToolBackend.Shared;
using Microsoft.Azure.Cosmos;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Azure.Monitor.OpenTelemetry.AspNetCore;
using HelmetToolBackend.ChallengeSolution;

var host = new HostBuilder()
    .ConfigureFunctionsWebApplication()
    .ConfigureServices(services =>
    {
        // Register your services here
        services.AddOpenTelemetry().UseAzureMonitor();

        services.AddSingleton<IAuthClient, AuthClient>();
        services.AddSingleton<IJwtHandler, JwtHandler>();
        services.AddSingleton<ILibraryStorage, LibraryStorage>();
        services.AddSingleton<IChallengeStorage, ChallengeStorage>();
        services.AddSingleton<IAnswerStorage, AnswerStorage>();
        services.AddSingleton<ISolutionStorage, SolutionStorage>();
        services.AddSingleton<Config>();
        services.AddSingleton((services) =>
        {
            var config = services.GetRequiredService<Config>();
            return new CosmosClient(config.CosmosDbConnectionString, new CosmosClientOptions
            {
                SerializerOptions = new CosmosSerializationOptions
                {
                    PropertyNamingPolicy = CosmosPropertyNamingPolicy.CamelCase,
                    IgnoreNullValues = true
                }
            });
        });

    })
    .Build();

host.Run();
