<script>
  import { onMount } from 'svelte';
  let step = 1;
  let name = '';
  let short_desc = '';
  let desc = "";
  let price;
  let old_price;
  let sku = '';
  let supplier = '';
  let stock;
  let width;
  let height;
  let weight;
  let category;
  let images = [];
  let currentPage = 1, productsPerPage = 16;
  let form = false;
  let products_list = [];
  let selectedCategory = "";  

  const formatarPreco = valor =>
    (valor / 100).toLocaleString('pt-BR', {
      style: 'currency',
      currency: 'BRL'
    });

  async function deletarProduto(id) {
    const token = localStorage.getItem("token");
    const res = await fetch("http://localhost:3000/d_product", {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
      },
      body: JSON.stringify({ id })
    });

    if (res.ok) {
      products_list = products_list.filter(p => p._id !== id);
      alert("Produto deletado com sucesso!");
    } else {
      alert("Erro ao deletar produto.");
    }
  }

  function changeStep(n) {
    step += n;
  }

  function paginate(page) {
    currentPage = page;
  }

  function handleFiles(event) {
    images = Array.from(event.target.files).map(file => URL.createObjectURL(file));
  }

  function toggleForm() {
    form = !form;
  }

  async function salvarProduto() {
  if (!name || !short_desc || !desc || !price || !sku || !supplier || !category) {
    alert("Por favor, preencha todos os campos obrigatórios!");
    return;
  }

  const token = localStorage.getItem("token");
  const productData = {
    name,
    short_desc,
    desc,
    price,
    old_price,
    sku,
    supplier,
    stock,
    width,
    height,
    weight,
    category,
    images
  };

  const res = await fetch('http://localhost:3000/s_product', {
    method: 'POST',
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${token}`
    },
    body: JSON.stringify(productData)
  });

  if (res.ok) {
    const newProduct = await res.json();
    products_list = [newProduct, ...products_list];
    alert("Produto registrado com sucesso!");
    toggleForm();
  } else {
    alert("Erro ao registrar produto.");
  }
}

  let totalPages = 0;

  $: filteredProducts = selectedCategory
    ? products_list.filter(product => product.category === selectedCategory)
    : products_list;

  $: paginatedProducts = filteredProducts.slice((currentPage - 1) * productsPerPage, currentPage * productsPerPage);

  $: totalPages = Math.ceil(filteredProducts.length / productsPerPage);

  onMount(async () => {
    const res = await fetch('http://localhost:3000/products');
    const data = await res.json();
    products_list = [...data.map(p => ({
      _id: p._id?.$oid ?? p._id,
      name: p.name,
      price: p.price,
      old_price: p.old_price,
      discount: p.discount,
      stock: p.stock,
      img: p.img?.[0] ?? "no-product.jpeg",
      store: p.store,
      href: "/produto",
      category: Array.isArray(p.category) ? p.category[0] ?? "Sem categoria" : p.category
    }))];

    products_list.forEach(p => { if (p.category == null || p.category == "") p.category = "Sem categoria"; });
  });

</script>

<main>
  {#if form}
    <div class="overlay"></div>
    <div class="modal-container">
      <div class="modal-header">
        <h2>Adicionar Produto (Etapa {step}/3)</h2>
        <button class="close-btn" on:click={toggleForm}>X</button>
      </div>

      <div class="modal-body">
        {#if step === 1}
          <input placeholder="Nome" bind:value={name}>
          <textarea placeholder="Descrição curta" bind:value={short_desc}></textarea>
          <textarea placeholder="Descrição" bind:value={desc}></textarea>
          <input type="number" placeholder="Preço Atual" bind:value={price}>
          <input type="number" placeholder="Preço Antigo" bind:value={old_price}>
        {/if}

        {#if step === 2}
          <input placeholder="SKU" bind:value={sku}>
          <input placeholder="Fornecedor" bind:value={supplier}>
          <input type="number" placeholder="Estoque" bind:value={stock}>
          <input type="number" placeholder="Largura" bind:value={width}>
          <input type="number" placeholder="Altura" bind:value={height}>
          <input type="number" placeholder="Peso" bind:value={weight}>
          <input placeholder="Categoria" bind:value={category}>
        {/if}

        {#if step === 3}
          <input type="file" multiple on:change={handleFiles}>
          <div class="image-preview">
            {#each images as img}
              <img src={img} alt="img-preview">
            {/each}
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        {#if step > 1}<button class="btn-secondary" on:click={() => changeStep(-1)}>Voltar</button>{/if}
        {#if step < 3}<button class="btn-primary" on:click={() => changeStep(1)}>Próximo</button>{/if}
        {#if step === 3}<button class="btn-success" on:click={salvarProduto}>Salvar Produto</button>{/if}
      </div>
    </div>
  {/if}

  <div class="container">
    <div class="top-menu">
      <h1>Novo produto</h1>
      <button class="btn-add-product" on:click={toggleForm}>+</button>
      <select class="form-select" bind:value={selectedCategory}>
        <option value="">Todas categorias</option>
        {#each Array.from(new Set(products_list.map(p => p.category))) as cat}
          <option value={cat}>{cat}</option>
        {/each}
      </select>
    </div>
    <hr>

    <div class="products-grid" style="display: flex; gap: 3rem;">
      {#each paginatedProducts as product (product._id)}
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
                          {#if product.old_price}
                            <p style="text-decoration: line-through; font-size: 0.9rem; color: #b8b8b8;">{formatarPreco(product.old_price)}</p>
                          {/if}
                            <p style="color: #32a852;">{formatarPreco(product.price)}</p>
                        </div>
                        <p style="color: white;">Quantidade: {product.stock}</p>
                    </div>

                    <hr>

                    <div style="display: flex; flex-direction: row; align-items: center; justify-content: center; gap: 1rem;">
                    <button class="botao-comprar" style="margin-top: 0.1rem; border: none; border-radius: 7px; background-color: #5142fc; color: white; width: 5rem; height: 3rem;">Editar</button>
                    <button class="botao-comprar" on:click={() => deletarProduto(product._id)} style="margin-top: 0.1rem; border: none; border-radius: 7px; background-color: #5142fc; color: white; width: 5rem; height: 3rem;">Excluir</button>
                    </div>
                </div>
              </div>
      {/each}
    </div>

    <nav class="pagination">
      {#each Array(totalPages) as _, index}
        <button class="{currentPage === index + 1 ? 'active' : ''}" on:click={() => paginate(index + 1)}>
          {index + 1}
        </button>
      {/each}
    </nav>
  </div>
</main>


<style>
    main {
    width: 100%;
    height: auto;
    min-height: 40rem;
    background-color: var(--background);
    color: var(--font);
    padding-bottom: 1rem;
  }

  .overlay {
    position: fixed;
    top: 0; left: 0;
    width: 100%; height: 100%;
    background-color: rgba(0,0,0,0.6);
    z-index: 998;
  }

  .modal-container {
    position: fixed;
    top: 50%; left: 50%;
    transform: translate(-50%, -50%);
    width: 40rem; height: 30rem;
    background-color: #212529;
    color: aliceblue;
    border-radius: 14px;
    padding: 1rem;
    z-index: 999;
    overflow-y: auto;
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .close-btn {
    cursor: pointer;
    color: red;
    font-weight: bold;
    background-color: transparent;
    border: none;
    font-size: 1.5rem;
    margin-bottom: 1rem;
  }

  .modal-body input,
  .modal-body textarea {
    width: 100%;
    padding: 0.5rem;
    border-radius: 0.3rem;
    border: 1px solid #444;
    background: #333;
    color: white;
    margin-bottom: 0.5rem;
  }

  .image-preview {
    display: flex;
    overflow-x: auto;
    gap: 0.5rem;
  }

  .image-preview img {
    width: 100px;
    height: 100px;
    object-fit: cover;
  }

  .modal-footer {
    position: sticky;
    bottom: 0;
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    padding-top: 0.5rem;
    background-color: #212529;
  }

  .btn-primary, .btn-secondary, .btn-success {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
  }

  .btn-primary { background-color: #007bff; color: white; }
  .btn-secondary { background-color: #6c757d; color: white; }
  .btn-success { background-color: #28a745; color: white; }

  .container {
    padding: 1rem;
  }

  .top-menu {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .btn-add-product {
    font-size: 1.5rem;
    border: none;
    background-color: #5142fc;
    color: white;
    width: 3rem;
    height: 3rem;
    border-radius: 7px;
  }

  .products-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }

  .pagination {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .pagination button {
    padding: 0.5rem 1rem;
    border-radius: 0.3rem;
    border: none;
    background-color: #343a40;
    color: white;
    cursor: pointer;
  }

  .pagination .active {
    background-color: #007bff;
  }

  .form-select {
    width: 13rem;
    height: 3rem; 
  }
</style>
