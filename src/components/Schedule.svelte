<script lang="ts">
  import Time from "../lib/Time.svelte";

  import rawData from "../assets/schedule.json";

  interface ScheduleData {
    [key: string]: {
      [key: string]: string;
    };
  }

  const data: ScheduleData = rawData;

  class EventTime {
    hour: number;
    minute: number;

    constructor(hour: number, minute: number) {
      this.hour = hour;
      this.minute = minute;
    }

    static fromString(time: string): EventTime {
      let [hour, minute] = time.split(":").map(Number);
      return new EventTime(hour, minute);
    }

    compareTo(other: EventTime): number {
      if (this.hour === other.hour) return this.minute - other.minute;
      return this.hour - other.hour;
    }

    toString(): string {
      return `${this.hour}:${this.minute.toString().padStart(2, "0")}`;
    }
  }

  let rooms = Object.keys(data);
  let times: EventTime[] = [];

  for (let key in data) {
    for (let time in data[key as keyof typeof data]) {
      times.push(EventTime.fromString(time));
    }
  }

  times.sort((a, b) => a.compareTo(b));

  times = times.filter(
    (time, index) =>
      times.findIndex((other) => time.compareTo(other) === 0) === index
  );
</script>

<table class="schedule">
  <thead>
    <th class="time">Time</th>
    <th class="activity">Main Areas</th>
    <th class="activity">Room 701</th>
    <th class="activity">PAC</th>
  </thead>

  {#each times as time}
    <tr>
      <td><Time hour={time.hour} minute={time.minute} /></td>
      {#each rooms as room}
        {#if data[room][time.toString()]}
          <td>{@html data[room][time.toString()]}</td>
        {:else}
          <td></td>
        {/if}
      {/each}
    </tr>
  {/each}
</table>

<style lang="scss">
  @use "sass:color";

  .schedule {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;

    & th,
    & td {
      padding: 10px;
      text-align: center;
    }

    & tr,
    & th {
      border-bottom: 1px solid color.adjust(white, $alpha: -0.5);
    }

    .activity {
      width: 33%;
    }

    .time {
      width: 100px;
    }
  }
</style>
