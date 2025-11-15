# Quick Start Guide

Get started with Bazzounquester in 5 minutes.

## Installation (One-Liner)

If you have Rust installed:

```bash
git clone https://github.com/yourusername/bazzounquester.git && cd bazzounquester && ./install.sh
```

Don't have Rust? Install it first:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## First Steps

### 1. Verify Installation

```bash
bazzounquester --version
```

If you get "command not found", try:
```bash
~/.cargo/bin/bazzounquester --version
```

### 2. Start Interactive Mode

```bash
bazzounquester
```

You'll see:
```
╔═══════════════════════════════════════════════════════════════╗
║              Bazzounquester - Interactive Mode                ║
║          Your Postman in the Terminal - HTTP CLI Tool        ║
╚═══════════════════════════════════════════════════════════════╝
```

### 3. Your First Request

Inside interactive mode:
```bash
bazzounquester> get https://httpbin.org/get
```

### 4. Try More Commands

```bash
# GET with query parameters
bazzounquester> get https://httpbin.org/get -q "name=John" -q "age=30"

# POST with JSON
bazzounquester> post https://httpbin.org/post -b '{"message":"Hello"}'

# With authentication
bazzounquester> get https://httpbin.org/bearer -H "Authorization:Bearer mytoken"

# Get help
bazzounquester> help

# Exit
bazzounquester> exit
```

## Command-Line Mode

You can also use it without interactive mode:

```bash
# Simple GET
bazzounquester get https://api.github.com/users/octocat

# POST with body and headers
bazzounquester post https://httpbin.org/post \
  -H "Content-Type:application/json" \
  -b '{"name":"test","value":123}'
```

## Common Tasks

### Test a REST API

```bash
# Get all resources
bazzounquester get https://jsonplaceholder.typicode.com/posts

# Get specific resource
bazzounquester get https://jsonplaceholder.typicode.com/posts/1

# Create new resource
bazzounquester post https://jsonplaceholder.typicode.com/posts \
  -H "Content-Type:application/json" \
  -b '{"title":"My Post","body":"Content","userId":1}'
```

### Debug API Endpoints

```bash
# Check headers and response
bazzounquester get https://httpbin.org/headers

# Test authentication
bazzounquester get https://httpbin.org/bearer \
  -H "Authorization:Bearer YOUR_TOKEN"

# Check response time
bazzounquester get https://httpbin.org/delay/2
```

## Essential Commands

| Command | Description |
|---------|-------------|
| `bazzounquester` | Start interactive mode |
| `bazzounquester --help` | Show help |
| `bazzounquester --version` | Show version |
| `bazzounquester get URL` | Make GET request |
| `bazzounquester post URL -b "data"` | Make POST request |

## Options

| Option | Description | Example |
|--------|-------------|---------|
| `-H` | Add header | `-H "Authorization:Bearer token"` |
| `-q` | Add query parameter | `-q "page=1"` |
| `-b` | Add request body | `-b '{"key":"value"}'` |

## Interactive Mode Commands

Inside interactive mode:

| Command | Description |
|---------|-------------|
| `help` | Show all commands |
| `version` | Show version info |
| `clear` | Clear screen |
| `exit` | Quit |

## Tips

1. **Use quotes for JSON**: Always use single quotes for JSON bodies
   ```bash
   -b '{"name":"value"}'
   ```

2. **Multiple headers**: Add `-H` multiple times
   ```bash
   -H "Content-Type:application/json" -H "Authorization:Bearer token"
   ```

3. **Arrow keys work**: Use ↑ and ↓ to navigate command history in interactive mode

4. **Create an alias**: Make it shorter
   ```bash
   alias req='bazzounquester'
   ```

## Troubleshooting

**Command not found?**
- Add to PATH: `export PATH="$HOME/.cargo/bin:$PATH"`
- Or use full path: `~/.cargo/bin/bazzounquester`

**Need help?**
- Run `bazzounquester --help`
- Type `help` in interactive mode
- See README.md for full documentation

## Next Steps

- Read [README.md](README.md) for detailed documentation
- See [INSTALL.md](INSTALL.md) for installation troubleshooting
- Try the examples in interactive mode
- Create your own aliases and shortcuts

---

**Happy requesting!**

Hassan Bazzoun | hassan.bazzoundev@gmail.com
