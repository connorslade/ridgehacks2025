<script lang="ts">
    import ListItem from "../lib/ListItem.svelte";
    import Section from "../lib/Section.svelte";

    import judges from "../assets/judges.json";
</script>

<Section title="Judges" dark={true}>
    {#each judges as judge}
        <ListItem title={judge.title}>
            {judge.description}
        </ListItem>
    {/each}
</Section>
