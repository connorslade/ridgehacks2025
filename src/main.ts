import "./style/main.scss";
import App from "./App.svelte";
import { inject } from "@vercel/analytics";
import { writable } from "svelte/store";

export let subscribed = writable(false);
export let serviceWorker: ServiceWorkerRegistration | null = null;

const app = new App({
  target: document.getElementById("app") as HTMLElement,
});

inject();

export default app;
