// Modified from https://gist.github.com/Klerith/80abd742d726dd587f4bd5d6a0ab26b6
export function urlBase64ToUint8Array(base64: string) {
  var padding = "=".repeat((4 - (base64.length % 4)) % 4);
  var base64 = (base64 + padding).replace(/\-/g, "+").replace(/_/g, "/");

  var rawData = window.atob(base64);
  var outputArray = new Uint8Array(rawData.length);

  for (var i = 0; i < rawData.length; ++i)
    outputArray[i] = rawData.charCodeAt(i);
  return outputArray;
}
