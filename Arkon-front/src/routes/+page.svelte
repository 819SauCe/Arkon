<script>
    import { fly } from "svelte/transition";
    import { onMount } from "svelte";

    let atual = 0;
    let direcao = 1;
    let itemContainer;
    let products_list = [];

    let item = [
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
        { img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste" },
    ];

    let imagens = [
        {
            img: "https://t3.ftcdn.net/jpg/02/62/18/46/360_F_262184611_bXhmboL9oE6k2ILu4qXxNWFhNJCEbTn2.jpg",
            href: "/teste",
        },
        {
            img: "https://t4.ftcdn.net/jpg/03/06/69/49/360_F_306694930_S3Z8H9Qk1MN79ZUe7bEWqTFuonRZdemw.jpg",
            href: "/teste",
        },
        {
            img: "https://cdn.vectorstock.com/i/500p/57/56/shopping-cart-banner-online-store-vector-42935756.jpg",
            href: "/teste",
        },
    ];

    const formatarPreco = (valor) =>
        (valor / 100).toLocaleString("pt-BR", {
            style: "currency",
            currency: "BRL",
        });

    function handleFiles(event) {
        const files = event.target.files;
        images = [];

        for (let i = 0; i < files.length; i++) {
            const reader = new FileReader();
            reader.onload = (e) => {
                images = [...images, e.target.result];
            };
            reader.readAsDataURL(files[i]);
        }
    }

    function gerarCombos(lista, tamanhoCombo = 3, quantidadeCombos = 3) {
        const combos = [];
        const usados = new Set();

        const maxCombosPossiveis = Math.floor(lista.length / tamanhoCombo);
        const combosParaGerar = Math.min(quantidadeCombos, maxCombosPossiveis);

        for (let i = 0; i < combosParaGerar; i++) {
            const combo = [];
            while (combo.length < tamanhoCombo) {
                const index = Math.floor(Math.random() * lista.length);
                if (!usados.has(index)) {
                    combo.push(lista[index]);
                    usados.add(index);
                }

                if (usados.size >= lista.length) break;
            }
            combos.push(combo);
        }

        return combos;
    }

    function calcularTotal(combo) {
        return combo.reduce((acc, p) => acc + p.price, 0);
    }

    let combos = gerarCombos(products_list, 3, 3);

    function proxima() {
        direcao = 1;
        atual = (atual + 1) % imagens.length;
    }

    function anterior() {
        direcao = -1;
        atual = (atual - 1 + imagens.length) % imagens.length;
    }

    function avancarItem() {
        itemContainer.scrollBy({ left: 300, behavior: "smooth" });
    }

    function voltarItem() {
        itemContainer.scrollBy({ left: -300, behavior: "smooth" });
    }

    function toggleFavorito(prod) {
        prod.favorito = !prod.favorito;
    }

    proxima();
    setInterval(proxima, 5000);

    onMount(async () => {
        const res = await fetch("http://localhost:3000/products");
        const data = await res.json();
        products_list = data.map((p) => ({
            name: p.name,
            price: p.price,
            old_price: p.old_price,
            discount: p.discount,
            stock: p.stock,
            img: p.img?.[0]?.startsWith("data:")
                ? p.img[0]
                : `http://localhost:3000/avatars/${p.img[0]}`,
            store: p.store,
            href: "/produto",
            category: p.category,
        }));
        combos = gerarCombos(products_list, 3, 3);
        console.log(data);
    });
</script>

<main>
    <div class="carrossel-container">
        <div class="carrossel-wrapper">
            {#each imagens as img, i (i)}
                {#if i === atual}
                    <a href={img.href}>
                        <img
                            src={img.img}
                            alt="Carrossel"
                            class="carrossel-img"
                            in:fly={{ x: direcao * 500, duration: 300 }}
                            out:fly={{ x: -direcao * 500, duration: 300 }}
                        />
                    </a>
                {/if}
            {/each}
            <div class="carrossel-controles">
                <button on:click={anterior} class="botao-carrossel">◀</button>
                <button on:click={proxima} class="botao-carrossel">▶</button>
            </div>
        </div>
    </div>

    <div class="ultimos-comprados">
        <h1>Últimos items Comprados</h1>
        <hr style="margin-left: 5rem;" />
        <div class="carrossel-botoes">
            <button on:click={voltarItem} class="botao-carrossel">◀</button>
            <button on:click={avancarItem} class="botao-carrossel">▶</button>
        </div>

        <div class="item-carrossel-container" bind:this={itemContainer}>
            {#each item as item}
                <div class="item-card">
                    <a href={item.href}>
                        <img src={item.img} alt="Avatar" class="item-img" />
                    </a>
                    <div>
                        <a href={item.href}>
                            <p>{item.name}</p>
                        </a>
                        <p>{formatarPreco(item.price)}</p>
                    </div>
                </div>
            {/each}
        </div>

        <hr style="margin-left: 5rem;" />
        <h1>Produtos</h1>
        <div class="container">
            <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 2.5rem; padding: 2rem;">
                {#if products_list.length}
                    {#each products_list.slice(0, 8) as product}
                        <div class="product-card fade-in" style="position: relative; background-color: #1a1a2e; padding: 1.5rem; border: 1px solid #2e2e3e; border-radius: 1rem; width: 18rem; display: flex; flex-direction: column; align-items: center;">
                            <button class="fav-btn" on:click={() => toggleFavorito(product)}>
                                {#if product.favorito}
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="#ff3e6c" viewBox="0 0 24 24" width="24" height="24">
                                        <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 6 4 4 6.5 4c1.74 0 3.41 1.01 4.13 2.44h.74C14.09 5.01 15.76 4 17.5 4 20 4 22 6 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
                                    </svg>
                                {:else}
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="#fff" stroke-width="2" viewBox="0 0 24 24" width="24" height="24">
                                        <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 6 4 4 6.5 4c1.74 0 3.41 1.01 4.13 2.44h.74C14.09 5.01 15.76 4 17.5 4 20 4 22 6 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
                                    </svg>
                                {/if}
                            </button>
                            <a href={product.href} style="text-decoration: none; color: inherit;">
                                <img src={product.img} alt="Avatar" style="width: 100%; height: 16rem; object-fit: cover; border-radius: 0.75rem;"/>
                            </a>
                            <a href={product.href} style="text-decoration: none; color: white; margin-top: 1rem; font-size: 1.2rem;">
                                {product.name}
                            </a>
                            <div style="width: 100%; margin-top: 1rem; display: flex; justify-content: space-between;">
                                <div>
                                    <p style="text-decoration: line-through; font-size: 0.9rem; color: #aaa;">
                                        {formatarPreco(product.old_price)}
                                    </p>
                                    <p style="color: #32a852; font-size: 1rem;">
                                        {formatarPreco(product.price)}
                                    </p>
                                </div>
                                <p style="color: white; font-size: 0.9rem;">
                                    Qtd: {product.stock}
                                </p>
                            </div>
                            <hr style="border-color: #444; width: 100%; margin: 1rem 0;"/>
                            <div style="display: flex; gap: 0.75rem; width: 100%; justify-content: center;">
                                <button style="background-color: #5142fc; color: white; border: none; padding: 0.5rem 1rem; border-radius: 0.5rem;">
                                    Comprar
                                </button>
                                <button style="background: none; border: none; color: white; display: flex; align-items: center; gap: 0.5rem;">
                                    <img src="cart.png" alt="cart" style="width: 1.2rem;"/>
                                    Cesta
                                </button>
                            </div>
                        </div>
                    {/each}
                {:else}
                    <h1 style="color: white;">Sem produtos :(</h1>
                {/if}
            </div>

            {#if products_list.length > 8}
                <div class="veja-mais-container">
                    <button class="veja-mais">Veja mais!</button>
                </div>
            {/if}
        </div>

        <hr />
        {#if !products_list}
            <h1>Combos Especiais</h1>
            <button class="rerrol-combos" on:click={() => (combos = gerarCombos(products_list, 3, 3))}>
                <img class="refresh-combo" src="refresh.png" alt="refresh" />
            </button>

            <div class="combos">
                {#each combos as combo, i}
                    <div class="combo-card">
                        <div class="combo-layout">
                            <div>
                                <img
                                    class="produto-principal"
                                    src={combo[0].img}
                                    alt="Produto principal"
                                />
                            </div>

                            <div>
                                {#each combo.slice(1) as produto}
                                    <div class="produto-combo">
                                        <img
                                            class="produto-adicional"
                                            src={produto.img}
                                            alt="Produto adicional"
                                        />
                                    </div>
                                {/each}
                            </div>
                        </div>

                        <div class="combo-div">
                            <p class="combo-p">Combo {i + 1}</p>
                            <p>
                                <s
                                    >{formatarPreco(
                                        calcularTotal(combo) * 1.2,
                                    )}</s
                                >
                                <span class="preco-combo-produto">
                                    {formatarPreco(calcularTotal(combo))}
                                </span>
                            </p>
                            <button class="botao-comprar">Comprar Combo</button>
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</main>

<style>
main {
 background-color: #141420;
 color: white
 }
 p {
 margin: 0;
 padding: 0
 }
 s {
 color: gray;
 margin-right: 1rem
 }
 .carrossel-container {
 width: 100%;
 margin: auto;
 overflow: hidden
 }
 .carrossel-wrapper {
 width: 100%;
 aspect-ratio: 24 / 9;
 position: relative
 }
 .carrossel-img {
 width: 100%;
 height: 100%;
 object-fit: cover;
 position: absolute;
 top: 0;
 left: 0
 }
 .carrossel-controles {
 position: absolute;
 top: 50%;
 left: 0;
 right: 0;
 display: flex;
 justify-content: space-between;
 transform: translateY(-50%);
 padding: 0 1rem;
 z-index: 1
 }
 .botao-carrossel {
 background-color: #5142fc;
 color: #fff;
 width: 2.5rem;
 height: 2.5rem;
 border-radius: 15px;
 display: flex;
 align-items: center;
 justify-content: center;
 font-size: 1.25rem;
 border: none;
 line-height: 1;
 transition: background-color 0.3s ease;
 box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2)
 }
 .item-carrossel-container {
 display: flex;
 overflow-x: auto;
 gap: 1rem;
 padding: 1rem 5rem;
 scroll-behavior: smooth;
 scrollbar-width: none
 }
 .item-carrossel-container::-webkit-scrollbar {
 display: none
 }

 .item-card {
 min-width: 20rem;
 height: 7rem;
 flex-shrink: 0;
 display: flex;
 align-items: center;
 border: 1px solid #4d4c5a;
 border-radius: 10px;
 background-color: #1a1a2e;
 padding: 0.5rem
 }
 .item-img {
 margin-left: 0.5rem;
 width: 5rem;
 height: 5rem;
 border-radius: 7px;
 margin-right: 0.5rem
 }
 .veja-mais:hover {
 background-color: #5142fc
 }
 .botao-comprar {
 margin-top: 0.1rem;
 border: none;
 border-radius: 7px;
 background-color: #5142fc;
 color: white;
 width: 5rem;
 height: 3rem
 }
 .combo-p {
 font-size: 1.3rem;
 font-weight: bold
 }
 .produto-combo {
 background-color: #fff;
 border-radius: 12px;
 padding: 0.5rem;
 display: flex;
 align-items: center;
 justify-content: center;
 height: 100px;
 width: 100px
 }
 .produto-principal {
 width: 100%;
 height: 100%;
 border-radius: 15px
 }
 .produto-adicional {
 max-height: 80px;
 max-width: 80px
 }
 .preco-combo-produto {
 color: #32a852;
 font-weight: bold
 }
 .combo-div {
 margin-top: 1rem;
 text-align: center
 }
 .ultimos-comprados {
 margin-top: 3rem
 }
 .ultimos-comprados h1 {
 margin-left: 5rem
 }
 .ultimos-comprados hr {
 width: 90%
 }
 .carrossel-botoes {
 display: flex;
 align-items: center;
 padding: 0 5rem
 }
 .item-carrossel-container {
 overflow-x: scroll
 }
 .item-card a {
 text-decoration: none;
 color: inherit
 }
 .item-card p {
 font-size: 1.3rem
 }
 .ultimos-comprados p {
 color: #32a852
 }
 .veja-mais {
 margin-bottom: 5rem;
 margin-top: 0.1rem;
 border: 3px solid white;
 border-radius: 7px;
 background-color: transparent;
 color: white;
 width: 7rem;
 height: 3rem;
 transition: all 0.3s ease-in-out
 }
 .veja-mais-container {
 width: 100%;
 display: flex;
 justify-content: center;
 margin-top: 2rem
 }
 .rerrol-combos {
 width: 3rem;
 height: 3rem;
 color: white;
 border: 3px solid white;
 background-color: transparent;
 border-radius: 7px;
 transition: background-color 0.3s ease
 }
 .rerrol-combos:hover {
 background-color: #5142fc
 }
 .combos {
 padding: 3rem;
 display: flex;
 flex-wrap: wrap;
 justify-content: center;
 gap: 2rem
 }
 .combo-card {
 display: flex;
 flex-direction: column;
 align-items: center;
 margin-bottom: 3rem
 }
 .combo-layout {
 display: flex;
 background-color: #1a1a2e;
 border-radius: 15px;
 padding: 1rem;
 gap: 1rem;
 width: 35rem;
 height: 25rem
 }
 .refresh-combo {
 width: 2rem;
 height: 2rem
 }
 .product-card {
 transition:
 transform 0.3s ease,
 box-shadow 0.3s ease
 }
 .product-card:hover {
 transform: translateY(-6px);
 box-shadow: 0 10px 20px rgba(0, 0, 0, 0.4)
 }
 .fade-in {
 opacity: 0;
 transform: translateY(20px);
 animation: fadeInUp 0.6s ease forwards
 }
 @keyframes fadeInUp {
 to {
 opacity: 1;
 transform: translateY(0)
 }
 }

 .fav-btn {
 position: absolute;
 top: 1.75rem;
 right: 2rem;
 background-color: var(--background);
 border: none;
 border-radius: 50%;
 width: 2rem;
 height: 2rem;
 display: flex;
 align-items: center;
 justify-content: center;
 cursor: pointer;
 transition: transform 0.2s
 }

 .fav-btn:hover {
 transform: scale(1.2)
 }
 </style>
