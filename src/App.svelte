<script type="ts">
  import logo from "./assets/logo.png";
</script>

<main>
  <img src={logo} alt="Ridgehacks 2024 logo" class="logo" />

  <div class="navbar">
    <a href="/">Schedule</a>
    <a href="/">Event Details</a>
    <a href="/">FAQ</a>
    <a href="/">Map</a>
    <a href="/">Sponsors</a>
    <a href="/">Organizers</a>
    <a href="/">Register</a>
  </div>

  <div class="sections">
    <div class="section">
      <h1>Ridgehacks 2024</h1>
      <!-- TODO: This was just copied from 2023 ridgehacks -->
      <p>
        Ridgehacks 2023 is Ridge High School's premier annual hackathon led by
        the <a href="ridgecompsci.club">Ridge Computer Science Club</a>.
        Following a single day, 12-hour hacking schedule, Ridgehacks offers a
        free hackathon for Middle and High School Students from 8:00 AM until
        10:00 PM.
      </p>
    </div>

    <div class="section">
      <h1>Schedule</h1>

      <p>todo&hellip;</p>
    </div>

    <div class="section">
      <h1>Event Details</h1>

      <p>todo&hellip;</p>
    </div>
  </div>
</main>

<style lang="scss">
  main {
    background-color: var(--background-color);
    font-family: sans-serif;
  }

  .logo {
    position: relative;
    left: 50%;
    transform: translateX(-50%);
    width: 500px;
    margin-bottom: -20px;
  }

  .navbar {
    display: flex;
    justify-content: center;
    gap: 20px;
    margin-top: 20px;
    margin-bottom: 30px;

    & a {
      text-decoration: none;
      color: #fff;
      font-size: 20px;
    }
  }

  .sections {
    margin: 0;
    max-width: 1200px;

    position: relative;
    left: 50%;
    transform: translateX(-50%);

    & .section {
      border-radius: 8px;
      background-color: var(--foreground-color);
      padding: 20px;
      padding-top: 10px;
      margin-bottom: 20px;
    }
  }
</style>
