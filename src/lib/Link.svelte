<script lang="ts">
  enum OpenIn {
    NewTab = "_blank",
    SameTab = "_self",
  }

  export let href: string;
  export let openIn: OpenIn = OpenIn.NewTab;
</script>

{#if openIn === OpenIn.NewTab}
  <a {href} target="_blank" rel="noopener noreferrer">
    <slot />
  </a>
{:else}
  <a {href}>
    <slot />
  </a>
{/if}
