<script lang="ts">
  export let title: string | null;
</script>

<p>
  <strong>
    {#if title}
      {title}:
    {:else}
      <slot name="title" />:
    {/if}
  </strong>
  <slot />
</p>
