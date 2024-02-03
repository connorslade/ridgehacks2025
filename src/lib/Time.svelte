<script lang="ts">
  export let hour: number;
  export let minute: number = 0;

  function pad(num: number): string {
    return num.toString().padStart(2, "0");
  }

  function to12Hour(hour: number, minute: number): string {
    hour %= 12;
    if (hour === 0) hour = 12;
    return `${hour}:${pad(minute)} ${hour < 12 ? "AM" : "PM"}`;
  }

  function to24Hour(hour: number, minute: number): string {
    return `${pad(hour)}:${pad(minute)}`;
  }
</script>

{to12Hour(hour, minute)}
