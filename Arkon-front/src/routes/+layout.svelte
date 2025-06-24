<script>
    import Header from "./Header.svelte"
    import Footer from "./Footer.svelte"
</script>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap');
    :root {
        --background: rgb(35, 33, 51);
        --foreground: rgb(51, 50, 55);
        --font: #ffffff;
    }

    :global(*) {
        padding: 0%;
        margin: 0%;
        border: 0%;
        font-family: "Nunito", sans-serif;
    }
</style>


<Header />
<slot />
<Footer />