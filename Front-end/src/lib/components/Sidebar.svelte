<script>
    import { page } from '$app/stores';
    $: currentPath = $page.url.pathname;
</script>

<side>
    <h1>Admin - Arkon</h1>
    <div class="admin-paths" class:active={currentPath === '/admin/webstores'}>
        <img class="icons-side" src="/container-icon.png" alt="webstore_icon"/>
        <a href="/admin/webstores">WebStores</a>
    </div>
    <div class="admin-paths" class:active={currentPath === '/admin/bank'}>
        <img class="icons-side" src="/container-icon.png" alt="webstore_icon"/>
        <a href="/admin/bank">Bank</a>
    </div>
</side>

<style>
    @import url("../../lib/styles/all.css");

    side {
        width: 25%;
        height: 100vh;
        display: flex;
        flex-direction: column;
        border-right: var(--border-default);
        padding-left: 1rem;
        padding-right: 1rem;
        gap: 1rem;
    }
    side h1 {
        margin-top: 31px;
    }
    .admin-paths {
        display: flex;
        align-items: center;
        gap: 1rem;
        text-align: center;
        padding: 0.3rem 0.5rem 0.3rem 0.5rem;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        font-size: 1.2rem;
    }
    .admin-paths:hover {
        background-color: #e4e4e4;
    }
    .admin-paths.active {
        background-color: #e4e4e4;
    }

    .icons-side {
        width: 1.5rem;
        height: 1.5rem;
    }
</style>