<script>
	import { onMount } from 'svelte';
	import { page } from '$app/stores';

	let user = null;
	let id = null;
	let admin = false;
	let userMiniPanel = false;
	$: currentPath = $page.url.pathname;

	onMount(() => {
		user = localStorage.getItem('username');
		id = localStorage.getItem('user_id');
		admin = localStorage.getItem('role') === 'admin';
	});

	function miniPanel() {
		userMiniPanel = !userMiniPanel;
	}
</script>

<header>
    <a href="/">
        <img class="logo" src="/svelte-icon.svg" alt="logo" />
    </a>

    <div class="nav">
        <a href="/" class:active={currentPath === '/'}>Home</a>
        <a>ERP</a>
        <a>CRM</a>
        <a>Comunidade</a>
        <a>Preço</a>
        <a>Contato</a>
        {#if admin }
            <a href="/admin" class:active=
            {currentPath === '/admin'||
            currentPath === '/admin/webstores' ||
            currentPath === '/admin/bank'}>admin-painel</a>
        {/if}
    </div>

    {#if !user}
    <div class="login">
        <a href="/login" id="default-btn" class="login-btn">Sing in</a>
        <a href="/register" id="default-btn" class="register-btn">Register</a>
    </div>
    {/if}
    {#if user}
        <div class="user" on:click={miniPanel}>
            <img class="user-img" src="/no-profile.svg" alt="user-icon">
            <p>{user}</p>
        </div>
    {/if}
    {#if userMiniPanel}
        <div class="mini-panel">
            <a href="/profile/{id}">Perfil</a>
            <a href="/settings">Configurações</a>
            <a href="/logout">Sair</a>
        </div>
    {/if}

</header>

<style>
    @import url("../styles/all.css");

    header {
        display: flex;
        width: 100%;
        height: 4rem;
        border-bottom: var(--border-default);
        align-items: center;
    }
    .logo {
        width: 2.4rem;
        height: 2.4rem;
        margin-left: 1rem;
    }
    .nav {
        display: flex;
        margin-left: auto;
        gap: 1rem;
    }
    .login {
        display: flex;
        gap: 1rem;
        margin-left: 1rem;
        margin-right: 1rem;
    }
    .login-btn {
        width: 5rem;
        background-color: #ebebeba6;
    }
    .register-btn {
        width: 5rem;
        background-color: black;
        color: white;
    }
    a {
        text-align: center;
        padding: 0.3rem 0.5rem 0.3rem 0.5rem;
        cursor: pointer;
    }
    a.active {
        text-align: center;
        padding: 0.3rem 0.5rem 0.3rem 0.5rem;
        border-radius: 8px;
        cursor: pointer;
        background-color: #e4e4e4;
        color: black;
        text-decoration: none;
    }
    .user-img {
        width: 2rem;
        height: 2rem;
        border-radius: 50%;
        margin-bottom: 3px;
    }
    .user {
        display: flex;
        gap: 0.5rem;
        margin-left: 1.5rem;
        margin-right: 1rem;
        align-items: center;
        justify-content: center;
    }
    .user {
        cursor: pointer;
    }
    /* mini panel */
    .mini-panel {
        position: absolute;
        top: 4rem;
        right: 1rem;
        background-color: white;
        border: var(--border-default);
        border-radius: 8px;
        box-shadow: 0px 4px 8px rgba(0,0,0,0.1);
        padding: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        z-index: 10;
    }
    .mini-panel a {
        padding: 0.3rem 0.5rem;
        text-decoration: none;
        color: black;
        border-radius: 4px;
    }
    .mini-panel a:hover {
        background-color: #f0f0f0;
    }
</style>