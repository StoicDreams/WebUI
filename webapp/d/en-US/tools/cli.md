<webui-data data-page-title="CLI Tools & Setup" data-page-subtitle=""></webui-data>

## Local CLI Setup for Static Site Development

For local development of the static site, you can use the following steps to set up a CLI environment. The main thing needed is the npx command from Node.js. There are several ways of installing Node.js.

> We use PowerShell on both Windows and Mac so our scripts can be consistent. If you're not using powershell you might need to modify these scripts or use different commands to suit your shell.

<webui-tabs pad="var(--padding)" transition-timing="200" data-subscribe="session-home-tab-index:setTab">
    <webui-button align="left" hash="welcome" slot="tabs">Windows</webui-button>
    <webui-content slot="content">
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
    </webui-content>
    <webui-button align="left" hash="welcome" slot="tabs">Mac</webui-button>
    <webui-content slot="content">
        For our Mac installation, we will be using the [Brew](https://brew.sh/) package installer.

        ```terminal:Update and Verify Brew
        brew update
        brew -v
        ```

        ```terminal:Install Node
        brew install node@22
        ```

        ```terminal:Verify versions
        node -v
        npm -v
        ```
    </webui-content>
</webui-tabs>

> [success] Then, you can run the following command to run your static website locally.

```terminal:Run the static site:
npx browser-sync start --server . --watch --single --host 127.0.0.1 --port 3210 --no-ui
```
