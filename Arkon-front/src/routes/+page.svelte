<script>
	import { fly } from 'svelte/transition';

    let atual = 0;
	let direcao = 1;
    let itemContainer;


    let item = [
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"},
        {img: "no-product.jpeg", name: "sem item", price: 0, href: "/teste"}
    ]

    let products_list = [
        {img: "no-product.jpeg", name: "camisa nike", price: 19999, old_price: 23999, cartegory: "camisa", stock: 234, href: "/teste"},
        {img: "no-product.jpeg", name: "tenis nike", price: 129999, old_price: 203999, cartegory: "tenis", stock: 2, href: "/teste"},
        {img: "no-product.jpeg", name: "tenis adidas", price: 15999, old_price: 33899, cartegory: "tenis", stock: 3, href: "/teste"},
        {img: "no-product.jpeg", name: "calça adidas", price: 10000, old_price: 20099, cartegory: "calca", stock: 700, href: "/teste"},
        {img: "no-product.jpeg", name: "camisa abidas", price: 5999, old_price: 20099, cartegory: "camisa", stock: 4, href: "/teste"},
        {img: "no-product.jpeg", name: "camisa flamengo", price: 5999, old_price: 20099, cartegory: "camisa", stock: 234, href: "/teste"},
        {img: "no-product.jpeg", name: "Funko pop", price: 19999, old_price: 23999, cartegory: "funko", stock: 234, href: "/teste"},
        {img: "no-product.jpeg", name: "Funko pop", price: 12999, old_price: 23999, cartegory: "funko", stock: 123, href: "/teste"}
    ]

	let imagens = [
        {img: "https://t3.ftcdn.net/jpg/02/62/18/46/360_F_262184611_bXhmboL9oE6k2ILu4qXxNWFhNJCEbTn2.jpg", href: "/teste"},
        {img: "https://t4.ftcdn.net/jpg/03/06/69/49/360_F_306694930_S3Z8H9Qk1MN79ZUe7bEWqTFuonRZdemw.jpg", href: "/teste"},
        {img: "https://cdn.vectorstock.com/i/500p/57/56/shopping-cart-banner-online-store-vector-42935756.jpg", href: "/teste"}
    ];

    const formatarPreco = valor =>
    (valor / 100).toLocaleString('pt-BR', {
      style: 'currency',
      currency: 'BRL'
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
        itemContainer.scrollBy({ left: 300, behavior: 'smooth' });
    }   

    function voltarItem() {
        itemContainer.scrollBy({ left: -300, behavior: 'smooth' });
    }

	proxima();
	setInterval(proxima, 5000);
</script>

<main style="background-color: #141420; color: white;">
<div class="carrossel-container">
	<div class="carrossel-wrapper">
		{#each imagens as img, i (i)}
			{#if i === atual}
                <a href={img.href}>
                    <img src={img.img} alt="Carrossel" class="carrossel-img" in:fly={{ x: direcao * 500, duration: 300 }} out:fly={{ x: -direcao * 500, duration: 300 }}/>
                </a>
			{/if}
		{/each}
		<div class="carrossel-controles">
			<button on:click={anterior} class="botao-carrossel">◀</button>
			<button on:click={proxima} class="botao-carrossel">▶</button>
		</div>
	</div>
</div>

<div class="ultimos-comprados" style="margin-top: 3rem;">
    <h1 style="margin-left: 5rem;">Últimos items Comprados</h1>
    <hr style="width: 90%; margin-left: 5rem;">
    <div style="display: flex; align-items: center; padding: 0 5rem;">
    <button on:click={voltarItem} class="botao-carrossel">◀</button>
    <button on:click={avancarItem} class="botao-carrossel" style="margin-left: 1rem;">▶</button>
</div>

    <div class="item-carrossel-container" bind:this={itemContainer} style="overflow-x: scroll;">
    {#each item as item}
        <div class="item-card">
            <a href={item.href} style="text-decoration: none; color: inherit;">
                <img src={item.img} alt="Avatar" class="item-img">
            </a>
            <div style="display: flex; flex-direction: column;">
                <a href={item.href} style="text-decoration: none; color: inherit;">
                    <p style="font-size: 1.3rem;">{item.name}</p>
                </a>
                <p style="color: #32a852;">R$ {formatarPreco(item.price)}</p>
            </div>
        </div>
    {/each}
    </div>

    <hr style="width: 90%; margin-left: 5rem; margin-top: 3rem;">
    <h1 style="margin-left: 5rem; margin-bottom: 1rem;">Produtos</h1>

    <div style="display: flex; flex-wrap: wrap; justify-content: center; gap: 2rem;">
        {#each products_list.slice(0, 8) as product}
            <div class="product-card" style="background-color: #1a1a2e; padding: 18px; border: 1px solid #4d4c5a; border-radius: 10px; width: 19rem; align-items: center; justify-content: center;">
                <a href={product.href} style="text-decoration: none; color: inherit;">
                    <img src={product.img} alt="Avatar" class="product-img" style="width: 100%; height: 12rem; display: block; margin: 0 auto; border-radius: 10px;">
                </a>
                <div style="display: flex; flex-direction: column;">
                    <a href={product.href} style="text-decoration: none; color: inherit;">
                        <p style="font-size: 1.3rem; margin-top: 1rem;">{product.name}</p>
                    </a>
                    
                    <div style="display: flex; justify-content: space-between;">
                        <div style="display: flex; flex-direction: column;">
                            <p style="text-decoration: line-through; font-size: 0.9rem; color: #b8b8b8;">{formatarPreco(product.old_price)}</p>
                            <p style="color: #32a852;">{formatarPreco(product.price)}</p>
                        </div>
                        <p style="color: white;">Quantidade: {product.stock}</p>
                    </div>

                    <hr>

                    <div style="display: flex; flex-direction: row; align-items: center; justify-content: center; gap: 1rem;">
                    <button class="botao-comprar" style="margin-top: 0.1rem; border: none; border-radius: 7px; background-color: #5142fc; color: white; width: 5rem; height: 3rem;">Comprar</button>
                    <button class="botao-comprar" style="margin-top: 0.1rem; background-color: transparent; color: white; border: none;"><img src="cart.png" alt="cart" style="width: 1.3rem; height: 1.3rem;"> Colocar na cesta</button>
                    </div>
                </div>
            </div>
        {/each}
    </div>

    <div style="width: 100%; display: flex; justify-content: center; margin-top: 2rem;">
        <button class="veja-mais" style="margin-bottom: 5rem; margin-top: 0.1rem; border: 3px solid white; border-radius: 7px; background-color: transparent; color: white; width: 7rem; height: 3rem; transition: all 0.3s ease-in-out;">Veja mais!</button>
    </div>

</div>

<hr>
<h1 style="margin-left: 5rem; margin-bottom: 1rem;">Combos Especiais</h1>
<button on:click={() => combos = gerarCombos(products_list, 3, 3)} class="veja-mais">
      <img src="refresh.png" alt="refresh" style="width: 1.3rem; height: 1.3rem;" />
</button>

<div class="combos" style="padding: 3rem; display: flex; flex-wrap: wrap; justify-content: center; gap: 2rem;">
  <div style="text-align: center; margin-bottom: 2rem;">
  </div>

  {#each combos as combo, i}
    <div class="combo-card" style="display: flex; flex-direction: column; align-items: center; margin-bottom: 3rem;">
      
      <div class="combo-layout" style="display: flex; background-color: #1a1a2e; border-radius: 15px; padding: 1rem; gap: 1rem; width: 35rem; height: 25rem;">
        <div style="flex: 1; display: flex; align-items: center; justify-content: center; background-color: #b6f5c8; border-radius: 12px; min-height: 260px;">
          <img src={combo[0].img} alt="Produto principal" style="width: 100%; height: 100%; border-radius: 15px;" />
        </div>

        <div style="display: flex; flex-direction: column; gap: 1rem;">
          {#each combo.slice(1) as produto}
            <div style="background-color: #fff; border-radius: 12px; padding: 0.5rem; display: flex; align-items: center; justify-content: center; height: 100px; width: 100px;">
              <img src={produto.img} alt="Produto adicional" style="max-height: 80px; max-width: 80px;" />
            </div>
          {/each}
        </div>
      </div>

      <div style="margin-top: 1rem; text-align: center;">
        <p style="font-size: 1.3rem; font-weight: bold;">Combo {i + 1}</p>
        <p style="font-size: 1.2rem;">
          <s style="color: gray; margin-right: 1rem;">{formatarPreco(calcularTotal(combo) * 1.2)}</s>
          <span style="color: #32a852; font-weight: bold;">{formatarPreco(calcularTotal(combo))}</span>
        </p>
        <button class="botao-comprar" style="margin-top: 0.5rem; background-color: #5142fc; color: white; border: none; padding: 0.6rem 1.5rem; border-radius: 6px;">Comprar Combo</button>
      </div>
    </div>
  {/each}

</div>

</main>

<style>
    p {
        margin: 0;
        padding: 0;
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
		box-shadow: 0 2px 4px rgba(0,0,0,0.2);
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
</style>
