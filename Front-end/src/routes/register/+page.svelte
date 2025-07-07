<script>
    let username = "";
    let email = "";
    let password = "";
    let passwordConfirm = "";
    let erroUsernameMSG = "";
    let erroEmailMSG = "";
    let erroPasswordMSG = "";

    function sanitizeInput(input) {
        return input.replace(/[<>"'`;]/g, "");
    }

    function registrar() {
        erroUsernameMSG = "";
        erroEmailMSG = "";
        erroPasswordMSG = "";

        // Validação simples
        if (!username) {
            erroUsernameMSG = 'Digite seu usuário';
        } else if (username.length > 20) {
            erroUsernameMSG = 'O usuário deve ter no máximo 20 caracteres';
        } else if (username.length < 3) {
            erroUsernameMSG = 'O usuário deve ter no mínimo 3 caracteres';
        } else if (username.includes(' ')) {
            erroUsernameMSG = 'O usuário não pode conter espaços';
        } else if (/[<>"'`]/.test(username)) {
            erroUsernameMSG = 'O usuário não pode conter caracteres especiais';
        }

        // Email
        if (!email) {
            erroEmailMSG = 'Digite seu email';
        } else if (!email.includes('@') || !email.includes('.')) {
            erroEmailMSG = 'Isso não é um email';
        } else if (email.length > 50) {
            erroEmailMSG = 'O email deve ter no máximo 50 caracteres';
        } else if (email.includes(' ')) {
            erroEmailMSG = 'O email não pode conter espaços';
        } else if (/[<>"'`]/.test(email)) {
            erroEmailMSG = 'O email não pode conter caracteres especiais';
        }

        // Password
        if (!password) {
            erroPasswordMSG = 'Digite sua senha';
        } else if (password !== passwordConfirm) {
            erroPasswordMSG = 'As senhas devem ser iguais';
        } else if (password.length < 8) {
            erroPasswordMSG = 'A senha deve ter no mínimo 8 caracteres';
        } else if (password.length > 20) {
            erroPasswordMSG = 'A senha deve ter no máximo 20 caracteres';
        }

        // Se houver erros, para aqui
        if (erroUsernameMSG || erroEmailMSG || erroPasswordMSG) {
            return;
        }

        // Se tudo estiver certo
        sessionStorage.setItem("username", username);
        sessionStorage.setItem("password", password);
        window.location.href = "/login";
    }
</script>

<main>
    <div class="container-register">
        <h1>Registre-se com</h1>

        <div class="register-options">

            <div class="img-register">
                <img src="/google-icon.svg" alt="google-icon">
            </div>

            <div class="img-register">
                <img src="/facebook-icon.svg" alt="facebook-icon">
            </div>

            <div class="img-register">
                <img src="/twitter-icon.svg" alt="twitter-icon">
            </div>

        </div>

        <p style="margin-top: 1rem;">ou</p>

        <div class="user-register">
            <div class="username-register">
                <span>Escolha seu username:</span>
                <input class="user-input" bind:value={username} type="text" placeholder="exemplo_123" maxlength="20">
                {#if erroUsernameMSG}
                    <p class="error-msg">{erroUsernameMSG}</p>
                {/if}
            </div>

            <div class="email-register">
                <span>Seu email:</span>
                <input class="email-input" bind:value={email} type="email" placeholder="exemplo@exemplo.com" maxlength="50">
                {#if erroEmailMSG}
                    <p class="error-msg">{erroEmailMSG}</p>
                {/if}
            </div>
        </div>

        <div class="password-container">
            <div class="password-register">
                <span>Sua melhor senha:</span>
                <input class="password-input" bind:value={password} type="password" placeholder="*******" maxlength="20">
                {#if erroPasswordMSG}
                    <p class="error-msg">{erroPasswordMSG}</p>
                {/if}
            </div>

            <div class="password-register">
                <span>Confirme sua senha:</span>
                <input class="password-input" bind:value={passwordConfirm} type="password" placeholder="*******" maxlength="20">
                {#if erroPasswordMSG}
                    <p class="error-msg">{erroPasswordMSG}</p>
                {/if}
            </div>
        </div>

        <button id="default-btn" class="register-btn" on:click={registrar}>Registrar</button>

        <hr>

        <div class="login-container">
            <p>Ja possui uma conta? <a class="login" href="/login">Login</a></p>
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
    img {
        width: 2rem;
        height: 2rem;
        filter: invert(1);    
    }
    .container-register {
        display: flex;
        flex-direction: column;
        align-items: center;
        border: var(--border-default);
        padding: 5rem;
        border-radius: 15px;
    }
    .register-options {
        display: flex;
        gap: 2rem;
        margin-top: 2rem;
    }
    .img-register {
        display: flex;
        width: 3.5rem;
        height: 3.5rem;
        border: var(--border-default);
        border-radius: 15px;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }
    .username-register, .password-register, .email-register {
        display: flex;
        flex-direction: column;
        margin-top: 1rem;
    }
    .register-btn {
        width: 10rem;
        height: 2.5rem;
        margin-top: 2rem;
        background-color: black;
        color: white;
    }
    .user-input, .password-input, .email-input {
        width: 15rem;
        height: 2.5rem;
        padding-left: 0.5rem;
        margin-top: 0.5rem;
    }
    .login-container {
        margin-top: 2rem;
    }
    .password-container {
        display: flex;
        gap: 1rem
    }
    .login {
        color: rgb(85, 127, 255);
    }
    .user-register {
        display: flex;
        margin-top: 1rem;
        border-top: var(--border-default);
        gap: 1rem;
    }
    .error-msg {
        color: red;
    }
</style>