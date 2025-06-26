<script>
    import { onMount } from 'svelte';
    import * as jwt_decode from 'jwt-decode';

    let user = "";
    let navItems = [
        { label: "Camisetas", href: "/camisetas" },
        { label: "Calçados", href: "/calcados" },
        { label: "Acessórios", href: "/acessorios" },
        { label: "Bonés", href: "/bones" },
        { label: "Outlet", href: "/outlet" },
        { label: "Outros", href: "/outros" }
    ];

    onMount(() => {
        const token = localStorage.getItem('token');
        if (token) {
            const decoded = jwt_decode.jwtDecode(token);
            user = decoded.username;
        }
    });
</script>

<main>
    <div class="logo-container">
        <a href="/">
            <img src="/Arkon.png" alt="Arkon Logo">
            <span>Arkon</span>
        </a>
    </div>

    <div class="search-container">
        <form class="search-form">
            <input type="text" placeholder="Pesquisar">
            <button type="submit">Buscar</button>
        </form>

        <div class="nav-bar">
            <a href="/" class="nav-link">Home</a>
            <a href="Explore" class="nav-link">Explore</a>
            {#each navItems as item}
                <a href={item.href} class="nav-link">{item.label}</a>
            {/each}
        </div>
    </div>

    {#if user}
    <div class="user-info">
        <img class="user-img" src="/no-user-img.webp" alt="User Avatar">
        <span>Bem Vindo, {user}!</span>
    </div>
    {/if}

    {#if !user}
    <div class="login">
        <a href="/login">Faça o login</a>
    </div>
    {/if}
</main>
<div class="bottom-bar"></div>

<style>
    main {
    position: fixed;
    width: 100%;
    height: 6rem;
    z-index: 2;
    background-color: transparent;
    color: white;
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #4d4c5a;
    overflow: hidden;
}

main::before {
    content: "";
    position: absolute;
    top: 0; left: 0;
    width: 100%;
    height: 100%;
    z-index: -1;
    backdrop-filter: blur(10px);
    background-color: rgba(20, 20, 32, 0);
}

.logo-container {
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    width: 10rem;
    height: auto;
}

.logo-container a {
    display: flex;
    align-items: center;
    text-decoration: none;
    color: inherit;
}

.logo-container img {
    width: 3rem;
    height: 3rem;
}

.logo-container span {
    margin-left: 0.2rem;
    margin-top: 0.2rem;
    font-size: 1.5rem;
    font-weight: bold;
}

.search-form {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
}

.search-form input {
    padding: 0.5rem;
    border-radius: 0.5rem;
    border: none;
    font-size: 1rem;
    width: 100%;
}

.search-form button {
    padding: 0.5rem;
    border-radius: 0.5rem;
    border: none;
    background-color: #007bff;
    color: #fff;
    font-size: 1rem;
}

.nav-bar {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
}

.nav-link {
    font-size: 1.2rem;
}

.user-info {
    margin-right: 1rem;
    color: white;
}

.user-img {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    margin-right: 0.5rem;
}

.login {
    margin-right: 1rem;
    color: white;
    border: 3px solid white;
    padding: 0.5rem;
    border-radius: 0.5rem;
    transition: all 0.3s ease-in-out;
}

.login:hover {
    background-color: #5142fc;
    border: 3px solid #5142fc;
}

.bottom-bar {
    width: 100%;
    height: 6rem;
    background-color: #141420;
}
    </style>