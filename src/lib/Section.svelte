<script lang="ts">
  export let title: string;
  let id = title.toLowerCase().replace(/\s/g, "-");
</script>

<div {id} class="section">
  <h1>{title}</h1>
  <slot />
</div>

<style>
  .section {
    border-radius: 8px;
    background-color: var(--foreground-color);
    padding: 20px;
    padding-top: 10px;
    margin-bottom: 20px;
    scroll-margin-top: 20px;
  }

  :global(.section:nth-child(odd)) {
    background-color: #49808a;
    color: #fff;
  }
</style>
