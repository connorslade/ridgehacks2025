<script lang="ts">
  import { onMount } from "svelte";
  import mapboxgl from "mapbox-gl";
  import "mapbox-gl/dist/mapbox-gl.css";

  import { mapToken } from "../assets/data.json";

  let mapContainer: any;

  mapboxgl.accessToken = mapToken;
  const navigation = new mapboxgl.NavigationControl();
  const geolocateControls = new mapboxgl.GeolocateControl({
    positionOptions: {
      enableHighAccuracy: true,
    },
    trackUserLocation: true,
    showUserHeading: true,
  });

  onMount(() => {
    const map = new mapboxgl.Map({
      container: mapContainer,
      style: "mapbox://styles/alphonse5/cleqda8m0003y01qlfhr22bgh",
      center: [-74.5465931, 40.6948376],
      zoom: 18,
    });

    map.addControl(geolocateControls);
    map.addControl(navigation);
    map.scrollZoom.disable();
  });
</script>

<div class="map" bind:this={mapContainer} />

<style>
  .map {
    height: 400px;
    border-radius: 10px;
  }
</style>
