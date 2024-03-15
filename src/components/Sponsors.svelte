<script lang="ts">
  import Section from "../lib/Section.svelte";
  import Sponsor from "../lib/Sponsor.svelte";
</script>

<Section title="Sponsors">
  <p>
    Looking to sponsor? Contact us at <a href="mailto:team@ridgehacks.us"
      >team@ridgehacks.us</a
    >.
  </p>

  <div class="sponsors">
    <Sponsor
      name="Bernards Township PBA"
      image="bernards-pba.png"
      link="https://www.facebook.com/bernardstownship.pba"
    />
    <Sponsor name="Car Tech Deals" image="car-tech-deals.png" />
    <Sponsor name="gen.xyz" image="gen-xyz.png" link="https://gen.xyz" />
    <Sponsor
      name="Goliath Automation & Robotics"
      image="goliath-automation-robotics.png"
      link="https://automationroboticsarduino.com"
    />
    <Sponsor
      name="Hoke Capital"
      image="hoke-capital.png"
      link="https://www.hokecapital.com"
    />
    <Sponsor name="Nord VPN" image="nord-vpn.png" link="https://nordvpn.com" />
    <Sponsor
      name="Perrotti"
      image="perrotti.png"
      link="https://www.perrottis.com"
    />
    <Sponsor
      name="Ridge Marketing"
      image="ridge-marketing.png"
      link="https://ridgemarketing.com"
      filter="brightness(0)"
    />
    <Sponsor
      name="Stop & Shop"
      image="stop-and-shop.png"
      link="https://stopandshop.com"
    />
    <Sponsor
      name="Wavelength Audio Video"
      image="wavelength-audio-video.png"
      link="https://wavelengthaudiovideo.com"
    />
    <Sponsor
      name="Wolfram"
      image="wolfram.png"
      link="https://www.wolfram.com"
    />
  </div>
</Section>

<style lang="scss">
  .sponsors {
    // 3 columns
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 24px;
    justify-items: center;
    padding-bottom: 10px;
  }
</style>
