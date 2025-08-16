<webui-data data-page-title="CLI Tools & Setup" data-page-subtitle=""></webui-data>

## Local CLI Setup for Static Site Development

For local development of the static site, you can use the following steps to set up a CLI environment.

```terminal:Download and install Chocolatey
powershell -c "irm https://community.chocolatey.org/install.ps1|iex"
```

> Restart terminal

```terminal:Download and install Node.js:
choco install nodejs --version="22.18.0"
```

> Restart terminal

```terminal:Verify versions:
node -v
npm -v
```

> [success] Then, you can run the following command to run your static website locally.

```terminal:Run the static site:
npx browser-sync start --server . --watch --single --host 127.0.0.1 --port 3210 --no-ui
```
