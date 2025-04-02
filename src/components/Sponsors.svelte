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

    <div class="sponsors">
        <Sponsor
            name="Balsamiq"
            image="balsamiq.png"
            link="https://balsamiq.com"
        />
        <Sponsor
            name="Buona Pizza"
            image="buona-pizza.svg"
            link="https://www.buonapizzanj.com"
        />
        <Sponsor name="CoCalc" image="cocalc.png" link="https://cocalc.com" />
        <Sponsor name="Gar" image="gar.png" />
        <Sponsor
            name="Hack United"
            image="hackunited.png"
            link="https://hackunited.org"
        />
        <Sponsor
            name="Hoke Capital"
            image="hoke-capital.png"
            link="https://www.hokecapital.com"
        />
        <Sponsor
            name="Incogni"
            image="incogni.png"
            link="https://incogni.com"
        />
        <Sponsor
            name="Nord Pass"
            image="nord-pass.png"
            link="https://nordpass.com"
        />
        <Sponsor
            name="Nord VPN"
            image="nord-vpn.png"
            link="https://nordvpn.com"
        />
        <Sponsor
            name="Perrottis"
            image="perrottis.svg"
            link="https://www.perrottis.com"
        />
        <Sponsor
            name="Bernards Township Police Department"
            image="police-department.png"
            link="https://www.bernards.org/departments/police"
        />
        <Sponsor
            name="Ridge Marketing"
            image="ridge-marketing.png"
            link="https://ridgemarketing.com"
        />
        <Sponsor name="Saily" image="saily.png" link="https://saily.com" />
        <Sponsor
            name="Verizon"
            image="verizon.png"
            link="https://www.verizon.com"
        />
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
