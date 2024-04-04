<script type="ts">
  // import logo from "./assets/logo.png";
  import logo from "./assets/logo.svg";
  import footerLogo from "./assets/ridgehacks.svg";

  import InnerLink from "./lib/InnerLink.svelte";
  import Section from "./lib/Section.svelte";
  import ListItem from "./lib/ListItem.svelte";
  import Link from "./lib/Link.svelte";
  import Schedule from "./components/Schedule.svelte";
  import Map from "./components/Map.svelte";
  import Time from "./lib/Time.svelte";
  import Organizers from "./components/Organizers.svelte";
  import Sponsors from "./components/Sponsors.svelte";
  import Faq from "./lib/Faq.svelte";
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
    <Link
      href="https://forms.gle/mHgRcivPSz6Xdpxb9"
      style="color: var(--accent-color)">Register</Link
    >
  </div>

  <div class="sections">
    <Section title="Ridgehacks 2024">
      <h2>
        Registration is currently open. Fill out <Link
          href="https://forms.gle/mHgRcivPSz6Xdpxb9">this form</Link
        > to sign up.
      </h2>

      <p>
        Ridgehacks 2024 is Ridge High School's premier annual hackathon led by
        the <Link href="https://ridgecompsci.club"
          >Ridge Computer Science Club</Link
        >. Following a single day, 12-hour hacking schedule, Ridgehacks offers a
        free hackathon for Middle, High School, and College Students from <Time
          hour={8}
        /> until <Time hour={22} />.
      </p>
    </Section>

    <Schedule />

    <Section title="Event Details">
      <ListItem title="Date">April 6th, 2024</ListItem>
      <ListItem title="Who can attend">
        Any high school or college students student who's down to make something
        awesome. Or anything tech, or programming related really.
      </ListItem>
      <ListItem title="Cost to attend">Free!</ListItem>
      <ListItem title="Location">
        Ridge High School (268 South Finley Avenue, Basking Ridge, NJ)
      </ListItem>
    </Section>

    <Section title="What is a Hackathon" dark={true}>
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

    <Map />

    <Section title="FAQ">
      <Faq question="Where can I park/where do I enter the building?">
        Please consult the map below. Make sure to enter the building through
        the Performing Arts Center (PAC).
      </Faq>

      <Faq question="Where do we get updates/communicate with y'all?">
        Discord! Make sure to join our server here.
      </Faq>

      <Faq question="How will teams work?">
        You can work in teams of 2 to 4. If you don't have a team, don't worry!
        You'll be able to form a team at RidgeHacks with other people looking
        for teams or teams that want more members before hacking begins.
      </Faq>

      <Faq question="How long is the event? When is it? Where is it?">
        It's on April 6th, 2024 from <Time hour={8} /> to <Time hour={22} />.
        It's at Ridge High School. You can find the address for our hackathon
        here.
      </Faq>

      <Faq question="What can I make?">
        Anything. Software, hardware, middleware, malware, anything that ends
        with “ware” and starts with you typing lines of code. And even more than
        that!
      </Faq>

      <Faq question="What's in it for me?">
        Well, the clout and prestige of winning Ridgehacks(college). And of
        course, we do have prizes. You'll have to show up to find out what they
        are though.
      </Faq>
    </Section>

    <Section title="Judges" dark={true}>
      <p>More information coming soon.</p>
    </Section>

    <Sponsors />

    <Organizers />
  </div>

  <div class="footer">
    <img src={footerLogo} class="logo" alt="Ridgehacks 2024" />

    <p>Ridge High School Hackathon, April 6th</p>
    <p>268 S Finley Ave, Basking Ridge, NJ 07920</p>
    <a href="mailto:team@ridgehacks.us">team@ridgehacks.us</a>
  </div>
</main>

<style lang="scss">
  @use "style/typefaces.scss";

  main {
    padding: 0 20px 0 20px;
  }

  .logo {
    position: relative;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    max-width: 600px;
    margin-bottom: -20px;
  }

  .navbar {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    gap: 20px;
    margin: 20px;
    margin-bottom: 30px;
    line-height: 1em;

    & :global(a) {
      color: #fff;
      text-decoration: none;
      font-size: 20px;
      transition: transform 0.2s;

      &:hover {
        transform: scale(0.9);
      }
    }
  }

  .sections {
    margin: 0;
    max-width: 1000px;

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

  @media (max-width: 600px) {
    main {
      padding: 0 10px 0 10px;
    }
  }
</style>
