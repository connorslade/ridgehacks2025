import { writable } from "svelte/store";

class Settings {
  militaryTime: boolean = false;

  constructor() {
    try {
      const settings = JSON.parse(localStorage.getItem("settings") || "");
      this.militaryTime = settings.militaryTime;
    } catch (error) {}
  }
}

export const settings = writable(new Settings());

settings.subscribe((value) => {
  localStorage.setItem("settings", JSON.stringify(value));
});
