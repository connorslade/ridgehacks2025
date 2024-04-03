<script lang="ts">
  import { settings } from "../settings";
  import Time from "../lib/Time.svelte";
  import Section from "../lib/Section.svelte";

  import { eventTimezone, eventDay } from "../assets/data.json";
  import rawData from "../assets/schedule.json";

  interface ScheduleData {
    [key: string]: {
      [key: string]: string;
    };
  }

  function correctTimeZone(date: Date): Date {
    return new Date(date.getTime() + eventTimezone * 3600 * 1000);
  }

  function addDateHours(date: Date, hours: number): Date {
    return new Date(date.getTime() + hours * 60 * 60 * 1000);
  }

  function toggleTimeFormat() {
    settings.update((s) => {
      s.militaryTime = !s.militaryTime;
      return s;
    });
  }

  class EventTime {
    hour: number;
    minute: number;

    constructor(hour: number, minute: number) {
      this.hour = hour;
      this.minute = minute;
    }

    static fromDate(date: Date): EventTime {
      date = correctTimeZone(date);
      return new EventTime(date.getUTCHours(), date.getUTCMinutes());
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
      return `${this.hour.toString().padStart(2, "0")}:${this.minute.toString().padStart(2, "0")}`;
    }
  }

  const data: ScheduleData = rawData;
  const eventDate = new Date(eventDay); // 1707841550 * 1000

  let rooms = Object.keys(data);
  let times: EventTime[] = [];

  for (let key in data)
    for (let time in data[key as keyof typeof data])
      times.push(EventTime.fromString(time));

  times.sort((a, b) => a.compareTo(b));

  times = times.filter(
    (time, index) =>
      times.findIndex((other) => time.compareTo(other) === 0) === index
  );

  let currentActivity = -1;
  refreshActivity();
  setTimeout(refreshActivity, 10);

  function refreshActivity() {
    let nowRaw = new Date();
    let now = EventTime.fromDate(nowRaw);
    if (nowRaw < eventDate || nowRaw > addDateHours(eventDate, 12)) {
      currentActivity = -1;
      return;
    }

    for (let i = times.length - 1; i >= 0; i--) {
      if (times[i].compareTo(now) < 0) {
        currentActivity = i;
        break;
      }
    }
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-missing-attribute -->
<Section title="Schedule" dark={true}>
  <p>Schedule has not yet been finalized and may still change.</p>

  <table class="schedule">
    <thead>
      <th class="time">Time</th>
      {#each rooms as room}
        <th class="activity">{room}</th>
      {/each}
    </thead>

    {#each times as time, idx}
      <tr class:current-time={idx == currentActivity}>
        <td>
          <Time hour={time.hour} minute={time.minute} />
        </td>
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

  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <a class="format-toggle" on:click={toggleTimeFormat}>
    Switch to {$settings.militaryTime ? "12" : "24"} hour time.
  </a>
</Section>

<style lang="scss">
  @use "sass:color";

  .schedule {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;
    min-width: 600px;

    & th,
    & td {
      padding: 10px;
      text-align: center;
    }

    & tr,
    & th {
      border-bottom: 1px solid color.adjust(white, $alpha: -0.5);
    }

    .current-time {
      background-color: color.adjust(white, $alpha: -0.8);
    }

    .activity {
      width: auto;
    }

    .time {
      width: 100px;
    }
  }

  .format-toggle {
    display: block;
    font-size: 0.8em;
    color: #ffffff8c;
    margin-bottom: 0;
    margin-top: 12.8px;
    width: max-content;
    cursor: pointer;
  }
</style>
