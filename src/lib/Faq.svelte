<script lang="ts">
  export let question: string;
</script>

<h3 class="question">{question}</h3>

<p class="answer">
  <slot />
</p>

<style lang="scss">
  .answer {
    margin-left: 1em;
  }
</style>
