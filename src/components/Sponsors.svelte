<script lang="ts">
    import Section from "../lib/Section.svelte";
    import Sponsor from "../lib/Sponsor.svelte";

    import sponsors from "../assets/sponsors.json";
</script>

<Section title="Sponsors" dark={true}>
    <p>
        Looking to sponsor? Contact us at <a href="mailto:team@ridgehacks.us"
            >team@ridgehacks.us</a
        >.
    </p>

    <div class="sponsors">
        {#each sponsors as { name, image, link }}
            <Sponsor {name} {image} {link} />
        {/each}
    </div>
</Section>

<style lang="scss">
    .sponsors {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
        gap: 24px;
        justify-items: center;
        padding-bottom: 10px;
    }
</style>
