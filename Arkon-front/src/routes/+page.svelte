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
            img: p.img?.[0] ?? "no-product.jpeg",
            store: p.store,
            href: "/produto",
            category: p.category,
        }));
        combos = gerarCombos(products_list, 3, 3);
    });
</script>

<main>
    <div class="carrossel-container">
        <div class="carrossel-wrapper">
            {#each imagens as img, i (i)}
                {#if i === atual}
                    <a href={img.href}>
                        <img src={img.img} alt="Carrossel" class="carrossel-img"
                        in:fly={{ x: direcao * 500, duration: 300 }}
                        out:fly={{ x: -direcao * 500, duration: 300 }}/>
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
        <hr />
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

        <hr />
        <h1>Produtos</h1>
        <div class="produtos-container">
            {#each products_list.slice(0, 8) as product}
                <div class="product-card">
                    <a href={product.href}>
                        <img
                            src={product.img}
                            alt="Avatar"
                            class="product-img"
                        />
                    </a>
                    <div>
                        <a href={product.href}>
                            <p>{product.name}</p>
                        </a>

                        <div>
                            <div>
                                <p>{formatarPreco(product.old_price)}</p>
                                <p>{formatarPreco(product.price)}</p>
                            </div>
                            <p>Quantidade: {product.stock}</p>
                        </div>

                        <hr />

                        <div class="buttons-container">
                            <button class="botao-comprar">Comprar</button>
                            <button class="botao-comprar">
                                <img src="cart.png" alt="cart" /> Colocar na cesta
                            </button>
                        </div>
                    </div>
                </div>
            {/each}
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
                        <img class="produto-principal" src={combo[0].img} alt="Produto principal"/>
                    </div>

                    <div>
                        {#each combo.slice(1) as produto}
                            <div class="produto-combo">
                                <img class="produto-adicional" src={produto.img} alt="Produto adicional"/>
                            </div>
                        {/each}
                    </div>
                </div>

                <div class="combo-div">
                    <p class="combo-p">Combo {i + 1}</p>
                    <p>
                        <s>{formatarPreco(calcularTotal(combo) * 1.2)}</s>
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
</main>

<style>
    main {
     background-color: #141420;
     color: white;
}
 p {
     margin: 0;
     padding: 0;
}
 s {
     color: gray;
     margin-right: 1rem;
}
 .carrossel-container {
     width: 100%;
     margin: auto;
     overflow: hidden;
}
 .carrossel-wrapper {
     width: 100%;
     aspect-ratio: 24 / 9;
     position: relative;
}
 .carrossel-img {
     width: 100%;
     height: 100%;
     object-fit: cover;
     position: absolute;
     top: 0;
     left: 0;
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
     z-index: 1;
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
     box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}
 .item-carrossel-container {
     display: flex;
     overflow-x: auto;
     gap: 1rem;
     padding: 1rem 5rem;
     scroll-behavior: smooth;
     scrollbar-width: none;
}
 .item-carrossel-container::-webkit-scrollbar {
     display: none;
}
 .produtos-container {
    width: 100%;
    height: auto;
    min-height: 40rem;
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
     padding: 0.5rem;
}
 .item-img {
     margin-left: 0.5rem;
     width: 5rem;
     height: 5rem;
     border-radius: 7px;
     margin-right: 0.5rem;
}
 .veja-mais:hover {
     background-color: #5142fc;
}
 .botao-comprar {
     margin-top: 0.1rem;
     border: none;
     border-radius: 7px;
     background-color: #5142fc;
     color: white;
     width: 5rem;
     height: 3rem;
}
 .combo-p {
     font-size: 1.3rem;
     font-weight: bold;
}
 .produto-combo {
     background-color: #fff;
     border-radius: 12px;
     padding: 0.5rem;
     display: flex;
     align-items: center;
     justify-content: center;
     height: 100px;
     width: 100px;
}
 .produto-principal {
     width: 100%;
     height: 100%;
     border-radius: 15px;
}
 .produto-adicional {
     max-height: 80px;
     max-width: 80px;
}
 .preco-combo-produto {
     color: #32a852;
     font-weight: bold;
}
 .combo-div {
     margin-top: 1rem;
     text-align: center;
}
 .ultimos-comprados {
     margin-top: 3rem;
}
 .ultimos-comprados h1 {
     margin-left: 5rem;
}
 .ultimos-comprados hr {
     width: 90%;
     margin-left: 5rem;
}
 .carrossel-botoes {
     display: flex;
     align-items: center;
     padding: 0 5rem;
}
 .item-carrossel-container {
     overflow-x: scroll;
}
 .item-card a {
     text-decoration: none;
     color: inherit;
}
 .item-card p {
     font-size: 1.3rem;
}
 .ultimos-comprados p {
     color: #32a852;
}
 .product-card {
     background-color: #1a1a2e;
     padding: 18px;
     border: 1px solid #4d4c5a;
     border-radius: 10px;
     width: 19rem;
     align-items: center;
     justify-content: center;
}
 .product-card img {
     width: 100%;
     height: 12rem;
     display: block;
     margin: 0 auto;
     border-radius: 10px;
}
 .product-card p {
     font-size: 1.3rem;
     margin-top: 1rem;
}
 .product-card div {
     display: flex;
     justify-content: space-between;
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
     transition: all 0.3s ease-in-out;
}
 .veja-mais-container {
     width: 100%;
     display: flex;
     justify-content: center;
     margin-top: 2rem;
}
  .rerrol-combos {
    width: 3rem;
    height: 3rem;
    color: white;
    border: 3px solid white;
    background-color: transparent;
    border-radius: 7px;
    transition: background-color 0.3s ease;
}
  .rerrol-combos:hover {
    background-color: #5142fc;
}
 .combos {
     padding: 3rem;
     display: flex;
     flex-wrap: wrap;
     justify-content: center;
     gap: 2rem;
}
 .combo-card {
     display: flex;
     flex-direction: column;
     align-items: center;
     margin-bottom: 3rem;
}
 .combo-layout {
     display: flex;
     background-color: #1a1a2e;
     border-radius: 15px;
     padding: 1rem;
     gap: 1rem;
     width: 35rem;
     height: 25rem;
}
 .refresh-combo {
     width: 2rem;
     height: 2rem;
}
</style>
