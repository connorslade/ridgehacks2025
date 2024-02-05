self.addEventListener("push", (event) => {
  event.waitUntil(
    self.registration.showNotification("Ridgehacks Update", {
      body: event.data.text(),
    })
  );
});

self.addEventListener("notificationclick", (event) => {
  event.notification.close();
  event.waitUntil(clients.openWindow("http://localhost:5173"));
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    // caches.match(event.request).then((response) => {
    //   return response || fetch(event.request);
    // })
    fetch(event.request)
  );
});
