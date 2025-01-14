<script lang="ts">
    import Section from "../lib/Section.svelte";
    import Sponsor from "../lib/Sponsor.svelte";
</script>

<Section title="Sponsors" dark={true}>
    <p>
        Looking to sponsor? Contact us at <a href="mailto:team@ridgehacks.us"
            >team@ridgehacks.us</a
        >.
    </p>

    <div class="sponsors"></div>
</Section>

<style lang="scss">
    .sponsors {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
        gap: 24px;
        justify-items: center;
        padding-bottom: 10px;
    }

    @media (max-width: 583px) {
        .sponsors {
            grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
        }
    }
</style>
