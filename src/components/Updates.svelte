<script lang="ts">
  import { debug, eventDay } from "../assets/data.json";
  import ListItem from "../lib/ListItem.svelte";
  import Section from "../lib/Section.svelte";

  // If it is more than a day before the event, hide the updates section
  let hidden = true;

  function updateHidden() {
    hidden = new Date().getTime() / 1000 < eventDay - 24 * 60 * 60;
  }

  updateHidden();
  setInterval(updateHidden, 10);
</script>

<Section title="Updates" hidden={hidden && !debug}>
  {#if "serviceWorker" in navigator && "PushManager" in window}
    <p>
      Click <a href="/">here</a> to subscribe to push notifications for future updates.
    </p>
  {:else}
    <p>
      Your browser does not support push notifications. This may be because it
      does not support service workers or the Push API.
    </p>
  {/if}

  <hr />

  <ListItem title="6:14 PM">
    Make sure you all have your team cards (the one with the numbers)! They are
    used for judging and are really important. If you need a card see Rachel in
    the Cafe.
  </ListItem>
  <ListItem title="10:12 AM">
    Have a song request for the cafeteria playlist? Submit them <a href="/"
      >here</a
    >!
  </ListItem>
  <ListItem title="8:47 AM">
    Find the opening ceremony slides <a href="/">here</a>.
  </ListItem>
  <ListItem title="6:39 AM">
    Please make sure you have filled out the <a href="/">MLH Form</a> and our
    <a href="/">Tally Form</a> in order to participate.
  </ListItem>
</Section>
