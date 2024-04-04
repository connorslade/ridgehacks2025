<script lang="ts">
  import { onMount } from "svelte";
  import mapboxgl from "mapbox-gl";
  import "mapbox-gl/dist/mapbox-gl.css";

  import { mapToken } from "../assets/data.json";
  import Section from "../lib/Section.svelte";

  let mapContainer: any;

  mapboxgl.accessToken = mapToken;
  const navigation = new mapboxgl.NavigationControl();
  const scale = new mapboxgl.ScaleControl({
    unit: "imperial",
  });
  const geolocate = new mapboxgl.GeolocateControl({
    positionOptions: {
      enableHighAccuracy: true,
    },
    trackUserLocation: true,
    showUserHeading: true,
  });

  onMount(() => {
    const map = new mapboxgl.Map({
      attributionControl: false,
      container: mapContainer,
      style: "mapbox://styles/alphonse5/cleqda8m0003y01qlfhr22bgh",
      center: [-74.5465931, 40.6943],
      zoom: 16,
    });

    map.addControl(scale);
    map.addControl(geolocate);
    map.addControl(navigation);
    map.scrollZoom.disable();
  });
</script>

<Section title="Map">
  <div class="map" bind:this={mapContainer} />
</Section>

<style>
  .map {
    height: 400px;
    border-radius: 10px;
  }

  :global(#map) {
    padding: 0;

    & h1 {
      position: absolute;
      left: 20px;
      z-index: 1;
      pointer-events: none;

      color: #fff;
      filter: drop-shadow(0px 0px 10px #000);
    }
  }
</style>
