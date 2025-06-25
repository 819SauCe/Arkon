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
    <div style="display: flex; align-items: center; justify-content: center; text-align: center; width: 10rem; height: auto;">
        <a href="/" style="display: flex; align-items: center; text-decoration: none; color: inherit;">
            <img src="/Arkon.png" alt="Arkon Logo" style="width: 3rem; height: 3rem;">
            <span style="margin-left: 0.2rem; margin-top: 0.2rem; font-size: 1.5rem; font-weight: bold;">Arkon</span>
        </a>
    </div>


    <div>
        <form class="search-form" style="display: flex; align-items: center; justify-content: center; gap: 0.5rem;">
            <input type="text" placeholder="Pesquisar" style="padding: 0.5rem; border-radius: 0.5rem; border: none; font-size: 1rem; width: 100%;">
            <button type="submit" style="padding: 0.5rem; border-radius: 0.5rem; border: none; background-color: #007bff; color: #fff; font-size: 1rem;">Buscar</button>
        </form>

        <div class="nav-bar" style="display: flex; align-items: center; justify-content: center; gap: 1rem;">
            <a href="/" class="nav-link" style="font-size: 1.2rem;">Home</a>
            <a href="Explore" class="nav-link" style="font-size: 1.2rem;">Explore</a>
            {#each navItems as item}
                    <a href={item.href} class="nav-link" style="font-size: 1.2rem;">{item.label}</a>
            {/each}
        </div>

    </div>

    {#if user}
    <div class="user-info" style="margin-right: 1rem; color: white;">
        <img class="user-img" src="/no-user-img.webp" alt="User Avatar" style="width: 2.5rem; height: 2.5rem; border-radius: 50%; margin-right: 0.5rem;">
        <span>Bem Vindo, {user}!</span>
    </div>
    {/if}
    {#if !user}
    <div class="login" style="margin-right: 1rem; color: white; border: 3px solid white; padding: 0.5rem; border-radius: 0.5rem; transition: all 0.3s ease-in-out;">
        <a href="/login" style="text-decoration: none; color: inherit;">Faça o login</a>
    </div>
    {/if}


</main>
<div style="width: 100%; height: 6rem; background-color: #141420;"></div>

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
    .login:hover {
        background-color: #5142fc;
        border: 3px solid #5142fc;

    }
    </style>