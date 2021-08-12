

let CACHE_NAME = 'pandadoodle-resources';
let urlsToCache = [];

self.addEventListener('install', function(event) {
// Perform install steps
    event.waitUntil(
        caches.open(CACHE_NAME)
        .then(function(cache) {
            console.log('Opened cache');
        return cache.addAll(urlsToCache);
        })
    );
});

self.addEventListener('fetch', function(event) {
  let request = event.request;

  if (request.method == "POST" || request.url.indexOf("statsig-prod-web-sdk.min.js") != -1) {
    event.respondWith(fetch(request));
    return;
  }

  // Try to fetch fresher non-static (index.html, index.js and 1.js) before using cache.
  if (request.url.indexOf("static") == -1) {
    event.respondWith(fetch(request).then(
      (response) => {
        console.log("fetching: " + request.url);
        return caches.open(CACHE_NAME).then((cache) => {
          cache.put(request, response.clone());
          return response;
        });
      },
      (error) => {
        console.log("trying to find in cache: " + request.url);
        return caches.match(request).then((response) => {
          if (response) {
            console.log("falling back to cache: " + request.url);
            return response;
          }
          return error;
        });
      }
    ));
    return;
  }

  // Remove "Range" header from audio requests so that they can be cached. 
  // Otherwise they return status 206 which can't be cached.
  if (event.request.headers.get("Range")) {
    let new_headers = new Headers();
    for (var pair of event.request.headers.entries()) {
      if (pair[0].toLowerCase() != "range") {
        new_headers.append(pair[0], pair[1]);
      }
    }
    request = new Request(
      request.url, {
        method: request.method,
        headers: new_headers,
        mode: 'same-origin', // need to set this properly
        credentials: request.credentials,
        redirect: request.redirect   // let browser handle redirects
      }
    );
  }

  // console.log("requesting: " + request.url + ", headers: " + request.headers.get("Range"));
  event.respondWith(
    caches.match(request).then((response) => {
      if (response) {
        // console.log("found in cache: " + request.url);
        return response;
      }
      return fetch(request).then((response) => {
        return caches.open(CACHE_NAME).then((cache) => {
          cache.put(request, response.clone());
          return response;
        });
      });
    })
  );
});