<script>
  import { onMount, afterUpdate } from 'svelte';

  let store = '',
  sku = '',
  active = false,
  name = '',
  desc = '',
  short_desc = '';
  let price = null,
  old_price = null,
  discount = null,
  free_shipping = false;
  let tags = [],
  category = [],
  stock = null,
  unit = '',
  width = null,
  height = null;
  let weight = null,
  supplier_id = '',
  supplier = '',
  created_at = new Date().toISOString();
  let imgFiles = [], img = [];
  let currentForm = null;

  function previewImages() {
    img = [];
    for (let i = 0; i < imgFiles.length; i++) {
      const reader = new FileReader();
      reader.onload = e => {
        img = [...img, e.target.result];
      };
      reader.readAsDataURL(imgFiles[i]);
    }
  }

  $: imgFiles, previewImages();

  afterUpdate(() => {
    if (img.length > 0) {
      const el = document.getElementById('carouselImages');
      if (el) {
        bootstrap.Carousel.getOrCreateInstance(el);
      }
    }
  });

  function handleSave() {
    const productData = {
      store,
      sku,
      active,
      name,
      desc,
      short_desc,
      price,
      old_price,
      discount,
      free_shipping,
      tags,
      category,
      stock,
      unit,
      width,
      height,
      weight,
      supplier_id,
      supplier,
      created_at,
      img
    };

    const token = localStorage.getItem('token');
    if (!token) {
      alert("Por favor, faça login antes de salvar o produto.");
      return;
    }

    fetch('http://localhost:3000/product', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: JSON.stringify(productData),
    })
            .then(response => {
              if (!response.ok) {
                throw new Error('Erro na resposta do servidor');
              }
              return response.json();
            })
            .then(data => {
              console.log('Resposta do servidor:', data);
            })
            .catch(error => {
              console.error('Erro ao salvar produto:', error);
            });
  }

  let products_list = [
        {img: "no-product.jpeg", name: "camisa nike", price: 19999, old_price: 23999, category: "camisa", stock: 234, href: "/teste"},
        {img: "no-product.jpeg", name: "tenis nike", price: 129999, old_price: 203999, category: "tenis", stock: 2, href: "/teste"},
        {img: "no-product.jpeg", name: "tenis adidas", price: 15999, old_price: 33899, category: "tenis", stock: 3, href: "/teste"},
        {img: "no-product.jpeg", name: "calça adidas", price: 10000, old_price: 20099, category: "calca", stock: 700, href: "/teste"},
        {img: "no-product.jpeg", name: "camisa abidas", price: 5999, old_price: 20099, category: "camisa", stock: 4, href: "/teste"},
        {img: "no-product.jpeg", name: "camisa flamengo", price: 5999, old_price: 20099, category: "camisa", stock: 234, href: "/teste"},
        {img: "no-product.jpeg", name: "Funko pop", price: 19999, old_price: 23999, category: "funko", stock: 234, href: "/teste"},
        {img: "no-product.jpeg", name: "Funko pop", price: 12999, old_price: 23999, category: "funko", stock: 123, href: "/teste"}
    ]

  function toggleForm() {
    currentForm = currentForm === null ? 1 : currentForm === 1 ? 2 : null;
  }

  function closeForm() {
    currentForm = null;
  }

  function goBack() {
    if (currentForm === 2) {
      currentForm = 1;
    } else {
      currentForm = null;
    }
  }
</script>

<main>
  <div class="top-menu">
    <h1>Novo produto</h1>
    <button class="btn-add-product" on:click={toggleForm}>+</button>
  </div>

  {#if currentForm === 1}
    <div class="form-container">
      <div class="form">
        <div style="display: flex;">
          <h2>Registrar Novo Produto</h2>
          <a on:click={closeForm} style="margin-left: auto; cursor: pointer; color: red; font-weight: bold; text-decoration: none;">X</a>
        </div>
        <div>
          <label for="name">Nome</label>
          <input id="name" type="text" bind:value={name}>
        </div>
        <div>
          <label for="desc">Descrição</label>
          <textarea id="desc" bind:value={desc}></textarea>
        </div>
        <div style="display: flex; gap: 10px">
          <div class="col-md-6">
            <label for="oldPrice" class="form-label">Preço antigo</label>
            <input id="oldPrice" type="number" class="form-control" bind:value={old_price} required>
          </div>
          <div class="col-md-6">
            <label for="newPrice" class="form-label">Preço Atual</label>
            <input id="newPrice" type="number" class="form-control" bind:value={price} required>
          </div>
        </div>
        <button on:click={closeForm}>Cancelar</button>
        <button on:click={toggleForm}>Avançar</button>
      </div>
    </div>

  {:else if currentForm === 2}
    <div class="form-container">
      <div class="form">
        <div style="display: flex;">
          <h2>Registrar {name}</h2>
          <a on:click={closeForm} style="margin-left: auto; cursor: pointer; color: red; font-weight: bold; text-decoration: none;">X</a>
        </div>
        <div>
          <label for="name">Código do produto</label>
          <input id="sku" type="text" bind:value={sku}>
        </div>
        <div>
          <label for="name">Fornecedor</label>
          <input id="supplier" type="text" bind:value={supplier}>
        </div>
        <button on:click={goBack}>Voltar</button>
        <button on:click={handleSave}>Salvar</button>
      </div>
    </div>
  {/if}

  <div class="produtos">
    {#each products_list as product}
      <div class="card" style="width: 18rem;">
        <img src={product.img} class="card-img-top" alt="...">
        <div class="card-body">
          <h5 class="card-title">{product.name}</h5>
          <p class="card-text">R${product.price}</p>
        </div>
      </div>
    {/each}
  </div>
</main>

<style>
  main {
    background-color: #141420;
    color: #fff;
    width: 100%;
    height: auto;
    padding: 1rem 0 1rem 0;
  }
  .produtos {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 2rem;
    z-index: 1;
  }
  .top-menu {
    display: flex;
    flex-direction: row;
    align-items: center;
  }
  .btn-add-product {
    border: none;
    border-radius: 7px;
    background-color: #5142fc;
    color: white;
    width: 3rem;
    height: 3rem;
  }
  .form-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 2;
  }
  .form {
    background-color: #222;
    padding: 20px;
    border-radius: 10px;
    margin-top: 5rem;
    width: 50rem;
    height: 35rem;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
  .form input, .form textarea {
    width: 100%;
    margin-bottom: 10px;
    padding: 10px;
    background-color: #333;
    border: none;
    border-radius: 5px;
    color: white;
  }
  .form button {
    width: 48%;
    padding: 10px;
    margin-top: 10px;
    border: none;
    border-radius: 5px;
    background-color: #5142fc;
    color: white;
  }
  .form button:hover {
    background-color: #4038c3;
  }
</style>




<!--
<main class="p-5 container-fluid bg-dark text-light">
  <h1 class="mb-4 text-success">
    {#if store && name}{name} — {store}
    {:else if store}Novo Produto — {store}
    {:else if name}{name} — Loja
    {:else}Novo Produto — Loja{/if}
  </h1>

  <div class="row g-4">
    <div class="col-md-5">
      <div class="card bg-secondary text-light shadow-lg rounded-4">
        {#if img.length > 0}
          <div id="carouselImages" class="carousel slide" data-bs-ride="carousel">
            <div class="carousel-inner rounded-top-4">
              {#each img as image, i}
                <div class="carousel-item {i === 0 ? 'active' : ''}">
                  <img src={image} class="d-block w-100 p-3" alt="Imagem" />
                </div>
              {/each}
            </div>
            <button class="carousel-control-prev" type="button" data-bs-target="#carouselImages" data-bs-slide="prev">
              <span class="carousel-control-prev-icon"></span>
            </button>
            <button class="carousel-control-next" type="button" data-bs-target="#carouselImages" data-bs-slide="next">
              <span class="carousel-control-next-icon"></span>
            </button>
          </div>
        {:else}
          <img src="/no-product.jpeg" class="card-img-top p-3 rounded-top-4" alt="Sem imagem" />
        {/if}
        <div class="card-body">
          <input type="file" class="form-control mb-3" bind:files={imgFiles} multiple accept="image/*" />
          <div class="form-switch">
            <input class="form-check-input" type="checkbox" id="activeSwitch" bind:checked={active}>
            <label class="form-check-label" for="activeSwitch">Ativo</label>
          </div>
          <div class="form-check mt-2">
            <input class="form-check-input" type="checkbox" id="freeShip" bind:checked={free_shipping}>
            <label class="form-check-label" for="freeShip">Frete Grátis</label>
          </div>
        </div>
      </div>
    </div>

    <div class="col-md-7">
      <div class="bg-secondary p-4 rounded-4 shadow-lg">
        <div class="mb-3">
          <input class="form-control form-control-lg text-light bg-dark border-success" type="text"
                 placeholder="Nome do Produto" bind:value={name}>
        </div>
        <div class="mb-3">
          <textarea class="form-control text-light bg-dark border-success" rows="2"
                    placeholder="Descrição curta" bind:value={short_desc}></textarea>
        </div>

        <div class="row g-3 mb-3">
          <div class="col">
            <label class="form-label">Preço antigo</label>
            <input class="form-control bg-dark text-light border-success" type="number" bind:value={old_price}>
          </div>
          <div class="col">
            <label class="form-label">Preço atual</label>
            <input class="form-control bg-dark text-light border-success" type="number" bind:value={price}>
          </div>
          <div class="col">
            <label class="form-label">Estoque</label>
            <input class="form-control bg-dark text-light border-success" type="number" bind:value={stock}>
          </div>
        </div>

        <div class="row g-3 mb-3">
          <div class="col">
            <label class="form-label">Largura (mm)</label>
            <input class="form-control bg-dark text-light border-success" type="number" bind:value={width}>
          </div>
          <div class="col">
            <label class="form-label">Altura (mm)</label>
            <input class="form-control bg-dark text-light border-success" type="number" bind:value={height}>
          </div>
          <div class="col">
            <label class="form-label">Peso (g)</label>
            <input class="form-control bg-dark text-light border-success" type="number" bind:value={weight}>
          </div>
        </div>

        <div class="mb-3">
          <textarea class="form-control bg-dark text-light border-success" rows="5"
                    placeholder="Descrição longa" bind:value={desc}></textarea>
        </div>

        <div class="d-flex gap-2">
          <button class="btn btn-success px-4" on:click={handleSave}>Salvar</button>
          <button class="btn btn-outline-light px-4">Cancelar</button>
        </div>
      </div>
    </div>
  </div>
</main>
-->