import "./style/main.scss";
import App from "./App.svelte";
import { writable } from "svelte/store";

export let subscribed = writable(false);
export let serviceWorker: ServiceWorkerRegistration | null = null;

const app = new App({
  target: document.getElementById("app") as HTMLElement,
});

serviceWorker = await navigator.serviceWorker.register("service-worker.js");
serviceWorker.pushManager.getSubscription().then((subscription) => {
  subscribed.set(subscription !== null);
});

export default app;
