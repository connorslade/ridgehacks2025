self.addEventListener("push", (event) => {
  console.log("[Service Worker] Push Received.");
  console.log(`[Service Worker] Push had this data: "${event.data.text()}"`);

  event.waitUntil(
    self.registration.showNotification("Ridgehacks Update", {
      body: event.data.text(),
    })
  );

  //   event.notification.close();
  //   event.waitUntil(clients.openWindow("https://developers.google.com/web"));
});
