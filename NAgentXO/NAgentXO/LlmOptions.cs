using System;
using System.Collections.Generic;
using System.Text;

namespace NAgentXO
{
    public sealed class LlmOptions
    {
        public const string SectionName = "Llm";

        public string DefaultProvider { get; set; } = "";
        public Dictionary<string, LlmProviderOptions> Providers { get; set; } = [];
    }

    public sealed class LlmProviderOptions
    {
        public string BaseUrl { get; set; } = "";
        public string ApiKey { get; set; } = "";
        public string DefaultModel { get; set; } = "";
        public Dictionary<string, string> Models { get; set; } = [];
    }
}
