<script>
    import Sidebar from "$lib/components/Sidebar.svelte";
    let balance = "0,00"
</script>

<div class="side-config">
<Sidebar />
<header>
    <p class="balance">R${balance}</p>
</header>
</div>

<style>
    @import url("../../../lib/styles/all.css");

    header {
        width: 100%;
        height: 3rem;
        border-bottom: var(--border-default);
        display: flex;
        justify-content: end;
        align-items: center;
    }

    .balance {
        margin-right: 1rem;
    }

    .side-config {
        display: flex;
    }
</style>