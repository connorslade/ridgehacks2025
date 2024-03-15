<script lang="ts">
  import Link from "./Link.svelte";

  export let name: string;
  export let image: string;
  export let link: string | null = null;

  export let filter: string | null = null;
</script>

<div class="container">
  {#if link === null}
    <img
      src={`/sponsors/${image}`}
      alt={name}
      title={name}
      class="sponsor-logo"
      style:filter
    />
  {:else}
    <Link href={link}>
      <img
        src={`/sponsors/${image}`}
        alt={name}
        title={name}
        class="sponsor-logo"
        style:filter
      />
    </Link>
  {/if}
</div>

<style lang="scss">
  .container {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .sponsor-logo {
    display: flex;
    width: 100%;
    align-items: center;
    gap: 30px;

    object-fit: contain;
    aspect-ratio: 1/1;
  }
</style>
