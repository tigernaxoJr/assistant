using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using NAgentXO;

// 1. 建立 Configuration 讀取 appsettings.json
IConfiguration configuration = new ConfigurationBuilder()
    .SetBasePath(Directory.GetCurrentDirectory())
    .AddJsonFile("appsettings.json", optional: false, reloadOnChange: true)
    .Build();

// 2. 建立 DI 容器
var services = new ServiceCollection();

// 3. 註冊服務
services.AddSingleton<IConfigService, ConfigService>();
services.AddTransient<ITaskWorker, TaskWorker>();
services.AddTransient<App>(); // 將主要 App 類別註冊進容器
// 讀取 LLM 配置並註冊到 DI 容器
var llmOptions = configuration
    .GetRequiredSection(LlmOptions.SectionName)
    .Get<LlmOptions>()
    ?? throw new InvalidOperationException("Missing LLM configuration.");
services.AddSingleton(llmOptions);

// 4. 建立 ServiceProvider 並啟動應用程式
using var serviceProvider = services.BuildServiceProvider();
var app = serviceProvider.GetRequiredService<App>();
app.Run(args); // args 在頂層陳述式中會自動存在，可以直接使用

