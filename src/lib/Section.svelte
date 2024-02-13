<script lang="ts">
  export let title: string;
  export let hidden: boolean = false;
  let id = title.toLowerCase().replace(/\s/g, "-");
</script>

<div {id} style:display={hidden ? "none" : "block"} class="section">
  <h1>{title}</h1>
  <slot />
</div>

<style lang="scss">
  @use "sass:color";

  .section {
    border-radius: 8px;
    background-color: var(--foreground-color);
    padding: 20px;
    padding-top: 10px;
    margin-bottom: 20px;
    scroll-margin-top: 20px;
    color: #000;
    overflow-x: auto;

    &:nth-child(even) {
      background-color: #49808a;
      color: #fff;
    }
  }

  @media (max-width: 600px) {
    .section {
      margin-bottom: 10px;
    }
  }
</style>
