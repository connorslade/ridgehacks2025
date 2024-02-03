<script type="ts">
  import logo from "./assets/logo.png";
  import footerLogo from "./assets/ridgehacks.svg";

  import InnerLink from "./lib/InnerLink.svelte";
  import Section from "./lib/Section.svelte";
  import ListItem from "./lib/ListItem.svelte";
  import Link from "./lib/Link.svelte";
  import Time from "./lib/Time.svelte";
</script>

<main>
  <a href="/">
    <img src={logo} alt="Ridgehacks 2024 logo" class="logo" />
  </a>

  <div class="navbar">
    <InnerLink href="#schedule">Schedule</InnerLink>
    <InnerLink href="#event-details">Event Details</InnerLink>
    <InnerLink href="#what-is-a-hackathon">What is a Hackathon</InnerLink>
    <InnerLink href="#map">Map</InnerLink>
    <InnerLink href="#sponsors">Sponsors</InnerLink>
    <InnerLink href="#organizers">Organizers</InnerLink>
    <a href="/" class="register">Register</a>
  </div>

  <div class="sections">
    <Section title="Ridgehacks 2024">
      <p>
        Ridgehacks 2023 is Ridge High School's premier annual hackathon led by
        the <Link href="https://ridgecompsci.club"
          >Ridge Computer Science Club</Link
        >. Following a single day, 12-hour hacking schedule, Ridgehacks offers a
        free hackathon for Middle and High School Students from 8:00 AM until
        10:00 PM.
      </p>
    </Section>

    <Section title="Schedule">
      <table class="schedule">
        <thead>
          <th class="time">Time</th>
          <th class="activity">Main Areas</th>
          <th class="activity">Room 701</th>
          <th class="activity">PAC</th>
        </thead>
        <tr>
          <td><Time hour={8} /></td>
          <td>Check-in</td>
          <td></td>
          <td></td>
        </tr>
        <tr>
          <td><Time hour={8} minute={15} /></td>
          <td></td>
          <td></td>
          <td>Opening Ceremony</td>
        </tr>
        <tr>
          <td><Time hour={9} /></td>
          <td>Hacking Begins</td>
          <td></td>
          <td>EEC Speaker</td>
        </tr>
        <tr>
          <td><Time hour={10} /></td>
          <td>Breakfast</td>
          <td>Consumer Repair Workshop sponsored by Tulls Automotive</td>
          <td></td>
        </tr>
        <tr>
          <td><Time hour={11} /></td>
          <td></td>
          <td>Open CV Workshop sponsored by Ridge Marketing</td>
          <td></td>
        </tr>
      </table>
    </Section>

    <Section title="Event Details">
      <ListItem title="Date">April 1st, 2023</ListItem>
      <ListItem title="Who can attend">
        Any middle school or high school student (grades 6-12) who's down to
        make something awesome. Or anything tech, or programming related really
        (please reach out if you are an elementary student interested in
        attending).
      </ListItem>
      <ListItem title="Cost to attend">Free!</ListItem>
      <ListItem title="Location">
        Ridge High School (268 South Finley Avenue, Basking Ridge, NJ)
      </ListItem>
    </Section>

    <Section title="What is a Hackathon">
      <p>
        A hackathon is an event where students individually or collaboratively
        create innovative solutions with technology, often using hardware,
        software, or both! Students present and share their ideas with other
        participants and can be awarded with prizes for their work. In addition,
        due to many workshops and experienced mentors, a hackathon is a great
        learning opportunity and experience. Overall, hackathons are an
        excellent event for any student with an interest in computer science and
        programming to attend, no matter the skill level.
      </p>
    </Section>

    <Section title="Map">
      <p>todo&hellip;</p>
    </Section>

    <Section title="Sponsors">
      <p>todo&hellip;</p>
    </Section>

    <Section title="Organizers">
      <p>todo&hellip;</p>
    </Section>
  </div>

  <div class="footer">
    <img src={footerLogo} class="logo" alt="Ridgehacks 2024" />

    <p>Ridge High School Hackathon, April 1st</p>
    <p>268 S Finley Ave, Basking Ridge, NJ 07920</p>
    <a href="mailto:team@ridgehacks.us">team@ridgehacks.us</a>
  </div>
</main>

<style lang="scss">
  @use "sass:color";
  @use "style/typefaces.scss";

  .logo {
    position: relative;
    left: 50%;
    transform: translateX(-50%);
    max-width: 600px;
    width: 75vw;
    margin-bottom: -20px;
  }

  .navbar {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 20px;
    margin-top: 20px;
    margin-bottom: 30px;

    & :global(a) {
      text-decoration: none;
      color: #fff;
      font-size: 20px;
    }

    & .register {
      color: var(--accent-color);
    }
  }

  .sections {
    margin: 0;
    padding: 10px;
    max-width: 1200px;

    position: relative;
    left: 50%;
    transform: translateX(-50%);
  }

  .footer {
    width: 300px;
    position: relative;
    left: 50%;
    transform: translateX(-50%);

    font-size: 0.7em;
    margin-bottom: 40px;
    line-height: 0.2em;
    text-align: center;

    & a,
    & p {
      font-family: typefaces.$degular;
      font-feature-settings: "c2sc";
      color: #fff;
    }

    & .logo {
      width: 300px;
      margin-bottom: 10px;
    }
  }

  .schedule {
    width: 100%;
    border-collapse: collapse;
    table-layout: fixed;

    & th,
    & td {
      padding: 10px;
      text-align: center;
    }

    & tr,
    & th {
      border-bottom: 1px solid color.adjust(white, $alpha: -0.5);
    }

    .activity {
      width: 33%;
    }

    .time {
      width: 100px;
    }
  }
</style>
