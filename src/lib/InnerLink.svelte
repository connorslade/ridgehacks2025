<script lang="ts">
  export let href: string;

  function handleClick(event: MouseEvent) {
    event.preventDefault();

    let target = event.target as HTMLAnchorElement;
    let anchorId = new URL(target.href).hash.slice(1);
    let anchor = document.getElementById(anchorId);
    if (anchor) anchor.scrollIntoView({ behavior: "smooth" });
  }
</script>

<a {href} on:click={handleClick}>
  <slot />
</a>
