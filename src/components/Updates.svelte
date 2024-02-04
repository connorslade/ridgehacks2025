<script lang="ts">
  import { serviceWorker, subscribed } from "../main";
  import { urlBase64ToUint8Array } from "../misc";
  import { debug, eventDay } from "../assets/data.json";
  import ListItem from "../lib/ListItem.svelte";
  import Section from "../lib/Section.svelte";

  // If it is more than a day before the event, hide the updates section
  let hidden = true;

  const updateHidden = () =>
    (hidden = new Date().getTime() / 1000 < eventDay - 24 * 60 * 60);
  updateHidden();
  setInterval(updateHidden, 10);

  let processing = false;
  const publicKey =
    "BJbCtPkzTVAuzV1mptTaCYQcZr5Nok42qNgN7sTu2RI_ZBL0tYmq2MLaeI7K3khfUXFFAEl3-RxOZkrujijb7G8";
  // Private key: TLLJmdpccdYruWA90CuyTZqVLLnjwQpL7gaZ798e4Cg

  async function updateServer(endpoint: string, subscribe: boolean) {
    const response = await fetch(
      `/api/${subscribe ? "subscribe" : "unsubscribe"}`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ endpoint }),
      }
    );
    if (!response.ok) console.error("Failed to update server:", response);
  }

  async function subscribe() {
    if (!serviceWorker) return;

    const key = urlBase64ToUint8Array(publicKey);
    let subscription = await serviceWorker.pushManager.subscribe({
      userVisibleOnly: true,
      applicationServerKey: key,
    });
    await updateServer(subscription.endpoint, true);
    subscribed.update(() => true);
  }

  async function unsubscribe() {
    if (!serviceWorker) return;

    let subscription = await serviceWorker.pushManager.getSubscription();
    if (subscription) {
      await updateServer(subscription.endpoint, false);
      await subscription.unsubscribe();
      subscribed.update(() => false);
    }
  }
</script>

<Section title="Updates" hidden={hidden && !debug}>
  {#if "serviceWorker" in navigator && "PushManager" in window}
    {#if $subscribed}
      <p>
        <strong>You are subscribed to push notifications.</strong> If you would
        like to unsubscribe, click
        <span
          on:click={unsubscribe}
          on:keydown={unsubscribe}
          class:processing
          role="button"
          class:clickable={true}
          tabindex="0">here</span
        >.
      </p>
    {:else}
      <p>
        <strong>You are not subscribed to push notifications.</strong> If you
        would like to subscribe, click
        <span
          on:click={subscribe}
          on:keydown={unsubscribe}
          role="button"
          class="clickable"
          tabindex="0">here</span
        >.
      </p>
    {/if}
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

<style lang="scss">
  .clickable {
    cursor: pointer;
    text-decoration: underline wavy;
  }

  .processing {
    pointer-events: none;
    cursor: not-allowed;
  }
</style>
