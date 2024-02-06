<script lang="ts">
  import { settings } from "../settings";

  export let hour: number;
  export let minute: number = 0;

  function pad(num: number): string {
    return num.toString().padStart(2, "0");
  }

  function to12Hour(hour: number, minute: number): string {
    let newHour = hour % 12;
    if (newHour === 0) newHour = 12;
    return `${newHour}:${pad(minute)} ${hour < 12 ? "AM" : "PM"}`;
  }

  function to24Hour(hour: number, minute: number): string {
    return `${pad(hour)}:${pad(minute)}`;
  }
</script>

{#if $settings.militaryTime}
  {to24Hour(hour, minute)}
{:else}
  {to12Hour(hour, minute)}
{/if}
