<webui-data data-page-title="Routing" data-page-subtitle=""></webui-data>

Web UI provides a simple and effective routing system to manage navigation within your application.

## Managed Navigation

All local navigation is handled through the routing system defined by the `loader.js` file. Users get animated transitions between pages which can be customized through CSS rules.

Page loading first checks if the requested page is already in the cache. If it is, the cached version is used, providing a faster loading experience. If not, the page is fetched from the server and added to the cache for future use.

If an expected page is not found, a 404 error page is displayed, informing the user that the requested resource could not be found. This page can also be customized to provide additional information or navigation options.
