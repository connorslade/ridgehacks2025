<script lang="ts">
    import Section from "../lib/Section.svelte";
    import Faq from "../lib/Faq.svelte";
    import Time from "../lib/Time.svelte";
    import InnerLink from "../lib/InnerLink.svelte";
</script>

<Section title="FAQ">
    <Faq question="Where can I park/where do I enter the building?">
        Please consult the map above. Make sure to enter the building through
        the Performing Arts Center (PAC).
    </Faq>

    <Faq question="How will teams work?">
        You can work in teams of 2 to 4. If you don't have a team, don't worry!
        You'll be able to form a team at RidgeHacks with other people looking
        for teams or teams that want more members before hacking begins.
    </Faq>

    <Faq question="How long is the event? When is it? Where is it?">
        It's on April 26th, 2025 from <Time hour={8} /> to <Time hour={22} />.
        It's at Ridge High School. You can find the address for our hackathon
        under
        <InnerLink href="#event-details">Event Details</InnerLink>.
    </Faq>

    <Faq question="What can I make?">
        Anything. Software, hardware, middleware, anything that ends with “ware”
        and starts with you typing lines of code. And even more than that!
    </Faq>

    <Faq question="What's in it for me?">
        Well, the clout and prestige of winning Ridgehacks (college). And of
        course, we do have prizes. You'll have to show up to find out what they
        are though.
    </Faq>
</Section>
