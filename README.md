## Firstrade MCP Server

This tool aims to provide a layer between your Firstrade API and LLMs. 
You can build your custom Firstrade API server with [morristai/firstrade](https://github.com/morristai/firstrade) or [MaxxRK/firstrade-api](https://github.com/MaxxRK/firstrade-api).

### Configuration

```json
{
  "mcpServers": {
    "firstrade-server": {
      "command": "PATH-TO-BINARY/firstrade-mcp",
      "env": {
        "API_HOST": "http://localhost",
        "API_PORT": "8080"
      }
    }
  },
  "globalShortcut": ""
}
```

## Disclaimer

This project is not affiliated with Firstrade or any of its subsidiaries. 
It is an independent implementation intended for educational and personal use only. Use at your own risk.