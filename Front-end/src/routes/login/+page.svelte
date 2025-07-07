<script>
	let username = "";
	let password = "";
	let errorMessage_username = '';
	let errorMessage_password = '';
	let showPassword = false;

	function sanitizeInput(input) {
		return input.replace(/[<>"'`;]/g, "");
	}

	function togglePassword() {
		showPassword = !showPassword;
	}

	async function entrar() {
		errorMessage_username = '';
		errorMessage_password = '';

		const cleanUsername = sanitizeInput(username);
		const cleanPassword = sanitizeInput(password);

		if (!cleanUsername) {
			errorMessage_username = 'Digite seu usuário';
		}
		if (!cleanPassword) {
			errorMessage_password = 'Digite sua senha';
		}
		if (errorMessage_username || errorMessage_password) {
			return;
		}

		try {
			const response = await fetch("http://localhost:3000/login", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					username: cleanUsername,
					password: cleanPassword,
				}),
			});

			const data = await response.json();

			if (response.ok) {
                localStorage.setItem("token", data.token);
                localStorage.setItem("username", data.username);
                localStorage.setItem("user_id", data.id);
                localStorage.setItem("role", data.role);
                window.location.href = "/";
            }
            else {
				errorMessage_password = 'Senha invalida';
			}
		} catch (err) {
			console.error("Erro na requisição:", err);
		}
	}
</script>

<main>
    <div class="container-login">
        <h1>Faça login com</h1>

        <div class="login-options">

            <div class="img-login">
                <img src="/google-icon.svg" alt="google-icon">
            </div>

            <div class="img-login">
                <img src="/facebook-icon.svg" alt="facebook-icon">
            </div>

            <div class="img-login">
                <img src="/twitter-icon.svg" alt="twitter-icon">
            </div>

        </div>

        <hr>

        <p style="margin-top: 1rem;">ou</p>

        <div class="user-login">
            <span>Seu usuário:</span>
            <input bind:value={username} class="user-input" type="text" placeholder="usuário" maxlength="20">
            {#if errorMessage_username}
                <p class="error-msg">{errorMessage_username}</p>
            {/if}
        </div>

        <div class="password-login">
            <span>Sua senha:</span>
            <div class="input">
            <input bind:value={password} class="password-input" type={showPassword ? 'text' : 'password'} placeholder="senha" maxlength="100">
            <button type="button" class="toggle-password" on:click={togglePassword}>
                {#if showPassword}
                    <img src="/see-password.png" alt="see-password" />
                {:else}
                    <img src="/ocult-password.png" alt="ocult-password.png" />
                {/if}
            </button>
            </div>
            <a class="forgot-password" href="/forgot-password">Esqueceu sua senha?</a>
            {#if errorMessage_password}
                <p class="error-msg">{errorMessage_password}</p>
            {/if}
        </div>

        <button id="default-btn" class="login-btn" on:click={entrar}>Entrar</button>

    <hr>

    <div class="container-register">
        <p>Ainda nao possui uma conta?</p>
        <a class="register" href="/register">Cadastre-se</a>
    </div>
</div>
</main>

<style>
    @import url("../../lib/styles/all.css");

    main {
        background-color: var(--color-primary);
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .img-login img {
        width: 2rem;
        height: 2rem;
        filter: invert(1);    
    }
    .container-login {
        display: flex;
        flex-direction: column;
        align-items: center;
        border: var(--border-default);
        padding: 5rem;
        border-radius: 15px;
    }
    .login-options {
        display: flex;
        gap: 2rem;
        margin-top: 2rem;
    }
    .img-login {
        display: flex;
        width: 3.5rem;
        height: 3.5rem;
        border: var(--border-default);
        border-radius: 15px;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }
    .user-login, .password-login {
        display: flex;
        flex-direction: column;
        margin-top: 1rem;
    }
    .login-btn {
        width: 10rem;
        height: 2.5rem;
        margin-top: 2rem;
        background-color: black;
        color: white;
    }
    .password-input, .user-input {
        width: 15rem;
        height: 2.5rem;
        padding-left: 0.5rem;
        margin-top: 0.5rem;
    }
    .container-register {
        margin-top: 1rem;
    }
    .register {
        color: rgb(85, 127, 255);
    }
    .error-msg {
        color: red;
    }
    .forgot-password {
        width: 11rem;
        height: auto;
        color: rgba(0, 0, 0, 0.250);
    }
    .input {
        display: flex;
        position: relative;
        margin-top: 0.5rem;
    }
    .input button {
        width: 1rem;
        height: 2.5rem;
        position: absolute;
        right: 0.5rem;
        cursor: pointer;
        background-color: transparent;
        border: none;
    }
    .input button img {
        width: 1rem;
        height: 1rem;
        margin-top: 1.2rem;
    }
</style>