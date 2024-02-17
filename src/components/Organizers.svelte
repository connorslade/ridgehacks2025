<script lang="ts">
  import Section from "../lib/Section.svelte";

  import organizers from "../assets/organizers.json";
</script>

<Section title="Organizers">
  <div class="organizers">
    {#each organizers as organizer}
      <div class="organizer">
        <!-- <img class="image" src={`/organizers/${organizer.image}`} alt="" /> -->
        <div>
          <p class="name">{organizer.name}</p>
          <p class="role">{organizer.role}</p>
        </div>
      </div>
    {/each}
  </div>
</Section>

<style lang="scss">
  .organizers {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 24px;
    justify-items: center;
    padding-bottom: 10px;
  }

  .organizer {
    display: flex;
    width: 100%;
    align-items: center;
    gap: 30px;

    & .image {
      height: 100px;
      border-radius: 100%;
      object-fit: cover;
      aspect-ratio: 1/1;
    }

    & .name {
      font-size: 1.6em;
      font-weight: bold;
      margin: 0;
    }

    & .role {
      margin: 0;
    }
  }
</style>
