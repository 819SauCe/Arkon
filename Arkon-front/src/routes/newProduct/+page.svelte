<script>
  let step = 1;
  let name = '', desc = '', short_desc = '';
  let price = null, old_price = null;
  let sku = '', supplier = '', stock = null;
  let width = null, height = null, weight = null;
  let category = '', tags = [];
  let active = true, free_shipping = true;
  let images = [];
  let atual = 0;
  let direcao = 1;
  let mainImageIndex = 0;
  let showForm = false;

  let products_list = [
    { img: "no-product.jpeg", name: "camisa nike", price: 19999, old_price: 23999, category: "camisa", stock: 234, href: "/teste" },
    { img: "no-product.jpeg", name: "tenis nike", price: 129999, old_price: 203999, category: "tenis", stock: 2, href: "/teste" },
    { img: "no-product.jpeg", name: "tenis adidas", price: 15999, old_price: 33899, category: "tenis", stock: 3, href: "/teste" },
    { img: "no-product.jpeg", name: "calça adidas", price: 10000, old_price: 20099, category: "calca", stock: 700, href: "/teste" },
    { img: "no-product.jpeg", name: "camisa abidas", price: 5999, old_price: 20099, category: "camisa", stock: 4, href: "/teste" },
    { img: "no-product.jpeg", name: "camisa flamengo", price: 5999, old_price: 20099, category: "camisa", stock: 234, href: "/teste" },
    { img: "no-product.jpeg", name: "Funko pop", price: 19999, old_price: 23999, category: "funko", stock: 234, href: "/teste" },
    { img: "no-product.jpeg", name: "Funko pop", price: 12999, old_price: 23999, category: "funko", stock: 123, href: "/teste" }
  ];

  function handleFileChange(event) {
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

  function changeImage(direction) {
    atual = (atual + direction + images.length) % images.length;
  }

  function setMainImage(index) {
    mainImageIndex = index;
    atual = index;
  }

  function navigate(stepValue) {
    step = stepValue;
  }

  function toggleForm() {
    showForm = !showForm;
  }

  function closeForm() {
    showForm = false;
  }
</script>

<main>
  <div class="top-menu">
    <h1>Novo produto</h1>
    <button class="btn-add-product" on:click={toggleForm}>+</button>
  </div>

  {#if showForm}
    {#if step === 1}
      <div class="form">
        <div style="display: flex;">
          <h2>Informações Básicas</h2>
          <a on:click={closeForm} style="margin-left: auto; cursor: pointer; color: red; font-weight: bold; text-decoration: none;">X</a>
        </div>
        <input placeholder="Nome" bind:value={name} />
        <textarea placeholder="Descrição Curta" bind:value={short_desc}></textarea>
        <input placeholder="Preço Atual" type="number" bind:value={price} />
        <input placeholder="Preço Antigo" type="number" bind:value={old_price} />
        <button on:click={() => navigate(2)}>Próximo</button>
      </div>
    {/if}

    {#if step === 2}
      <div class="form">
        <div style="display: flex;">
        <h2>Detalhes Adicionais</h2>
          <a on:click={closeForm} style="margin-left: auto; cursor: pointer; color: red; font-weight: bold; text-decoration: none;">X</a>
        </div>
        <input placeholder="SKU" bind:value={sku} />
        <input placeholder="Fornecedor" bind:value={supplier} />
        <input placeholder="Estoque" type="number" bind:value={stock} />
        <input placeholder="Largura" type="number" bind:value={width} />
        <input placeholder="Altura" type="number" bind:value={height} />
        <input placeholder="Peso" type="number" bind:value={weight} />
        <input placeholder="Categoria" bind:value={category} />
        <button on:click={() => navigate(1)}>Voltar</button>
        <button on:click={() => navigate(3)}>Próximo</button>
      </div>
    {/if}

    {#if step === 3}
      <div class="form">
        <div style="display: flex;">
          <h2>Imagens</h2>
          <a on:click={closeForm} style="margin-left: auto; cursor: pointer; color: red; font-weight: bold; text-decoration: none;">X</a>
        </div>
        <div class="carrossel-wrapper">
          <div>
            <!-- Exibição da imagem principal -->
            <div class="mb-4 d-flex justify-content-center">
              <img id="selectedImage" src={images[mainImageIndex] || "https://mdbootstrap.com/img/Photos/Others/placeholder.jpg"}
                alt="Imagem selecionada" style="width: 25rem; height: 25rem; margin-top: -1rem;" />
            </div>
            <!-- Botões de navegação do carrossel -->
            <div class="carrossel-controles" style="display: flex; justify-content: space-between;">
              <button style="" on:click={() => changeImage(-1)}>◀</button>
              <button style="" on:click={() => changeImage(1)}>▶</button>
            </div>
          </div>

          {#if images.length > 0}
            <!-- Exibição do carrossel de imagens -->
            <div class="carrossel-imagens" style="display: flex; ">
              {#each images as image, index}
                <div class="carrossel-item" on:click={() => setMainImage(index)}>
                  <img src={image} alt="Imagem do Carrossel" style="width: 1rem; height: 1rem; cursor: pointer;" />
                </div>
              {/each}
            </div>
            <div style="text-align: center; margin-top: 0.5rem;">
              <span style="color: #aaa;">Clique na imagem para definir como principal</span><br>
              <span style="color: #fff;">Imagem principal: {mainImageIndex + 1} / {images.length}</span>
            </div>
          {/if}

          <!-- Input de upload de imagem -->
          <div class="d-flex justify-content-center">
            <div data-mdb-ripple-init class="btn btn-primary btn-rounded">
              <label class="form-label text-white m-1" for="customFile1">Escolher Arquivo</label>
              <input type="file" class="form-control d-none" id="customFile1" multiple accept="image/*" on:change={handleFileChange} />
            </div>
          </div>
        </div>

        <button on:click={() => navigate(2)}>Voltar</button>
        <button on:click={() => alert("Produto salvo (simulação).")}>Salvar</button>
      </div>
    {/if}
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
    padding: 1rem;
  }

  .top-menu {
    display: flex;
    justify-content: space-between;
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

  .form {
    position: fixed;
    margin-left: 50%;
    transform: translate(-50%, -20%);
    background-color: #222;
    padding: 20px;
    border-radius: 10px;
    width: 50%;
    height: 40rem;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    z-index: 3;
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
    width: 49%;
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

  .produtos {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 2rem;
    z-index: 1;
  }

  .carrossel-wrapper {
    position: relative;
    margin-top: 1rem;
  }

  .carrossel-img {
    width: 100%;
    height: 400px;
  }

  .carrossel-controles button {
    background: transparent;
    border: none;
    color: #fff;
    font-size: 2rem;
  }
</style>
