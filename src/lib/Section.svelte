<script lang="ts">
  export let title: string;
  let id = title.toLowerCase().replace(/\s/g, "-");
</script>

<main {id}>
  <h1>{title}</h1>
  <slot />
</main>

<style>
  main {
    border-radius: 8px;
    background-color: var(--foreground-color);
    padding: 20px;
    padding-top: 10px;
    margin-bottom: 20px;
    scroll-margin-top: 20px;

    &:nth-child(even) {
      background-color: #49808a;
      color: #fff;
    }
  }
</style>
