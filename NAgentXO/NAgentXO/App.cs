using System;
using System.Collections.Generic;
using System.Text;

namespace NAgentXO
{
    internal class App
    {
        private readonly IConfigService _config;
        private readonly ITaskWorker _worker;
        private readonly LlmOptions llmOptions;

        // DI 容器會自動注入所需要的依賴
        public App(IConfigService config, ITaskWorker worker, LlmOptions llmOptions)
        {
            _config = config;
            _worker = worker;
            this.llmOptions = llmOptions;
        }

        public void Run(string[] args)
        {
            Console.WriteLine($"[App 啟動] 目前版本：{_config.GetVersion()}");
            _worker.DoWork();
            var providerName = "OpenAI";
            var provider = llmOptions.Providers[providerName];
            var model = provider.Models[provider.DefaultModel];

            Console.WriteLine($"{provider.BaseUrl} / {model}");
            Console.ReadLine();
        }
    }
    public interface IConfigService { string GetVersion(); }
    public class ConfigService : IConfigService
    {
        public string GetVersion() => "v1.0.0";
    }

    public interface ITaskWorker { void DoWork(); }
    public class TaskWorker : ITaskWorker
    {
        public void DoWork() => Console.WriteLine("TaskWorker 正在執行 Agent 任務...");
    }
}
