<script>
  import { onMount, afterUpdate } from 'svelte';

  let store = '', sku = '', active = false, name = '', desc = '', short_desc = '';
  let price = null, old_price = null, discount = null, free_shipping = false;
  let tags = [], category = [], stock = null, unit = '', width = null, height = null;
  let weight = null, supplier_id = '', supplier = '', created_at = new Date().toISOString();
  let imgFiles = [], img = [];

  // Função para pré-visualizar as imagens
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

  // Função chamada ao clicar no botão Salvar
  function handleSave() {
    // Criar um objeto com os dados para enviar ao backend
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

    // Obter o token JWT (deve ter sido armazenado no login)
    const token = localStorage.getItem('token');  // Obtém o token do localStorage

    if (!token) {
      alert("Por favor, faça login antes de salvar o produto.");
      return;
    }

    // Exibir os dados no console para verificar
    console.log('Dados do produto:', productData);

    fetch('http://localhost:3000/product', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,  // Envia o token JWT no cabeçalho Authorization
      },
      body: JSON.stringify(productData),  // Dados do produto a serem salvos
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
</script>

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
