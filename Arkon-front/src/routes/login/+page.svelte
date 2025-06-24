<script>
    let email = '';
    let password = '';

    async function login(e) {
        e.preventDefault();
        const res = await fetch('http://localhost:3000/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password })
        });

        if (res.ok) {
            const data = await res.json();
            localStorage.setItem('token', data.token);
            alert('Login bem-sucedido!');
        } else {
            alert('Email ou senha inválidos.');
        }
    }
</script>

<main style="background-color: #141420; color: white; width: 100%; height: 100vh; justify-content: center; display: flex;">
    <div class="login-container" style="display: flex; flex-direction: column; text-align: center; align-items: center; justify-content: center; background-color: #2c2c39; width: 30rem; height: 35rem; margin-top: 4rem; border:1px solid #4d4c5a; border-radius: 10px;">
        <h1 style="font-size: 1.7rem;">Faça Login Agora</h1>
        <h2 style="color: grey; font-size: 1.1rem;">Faça login com sua conta para um checkout mais rapido.</h2>

        <div style="display: flex; gap: 1rem;">
        <a href="/auth/google" style="display: inline-flex; align-items: center; gap: 10px; padding: 10px; border:1px solid #4d4c5a; color: #141420; text-decoration: none; border-radius: 5px; font-weight: bold;">
            <img src="https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/google.svg" alt="Google icon" width="20" height="20">
        </a>

        <a href="/auth/facebook" style="display: inline-flex; align-items: center; gap: 10px; padding: 10px;border:1px solid #4d4c5a; color: #141420; text-decoration: none; border-radius: 5px; font-weight: bold;">
            <img src="https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/facebook.svg" alt="Google icon" width="20" height="20">
        </a>

        <a href="/auth/twitter" style="display: inline-flex; align-items: center; gap: 10px; padding: 10px;border:1px solid #4d4c5a; color: #141420; text-decoration: none; border-radius: 5px; font-weight: bold;">
            <img src="https://www.gstatic.com/firebasejs/ui/2.0.0/images/auth/twitter.svg" alt="Google icon" width="20" height="20">
        </a>
        </div>

        <p style="color: grey; font-size: 1rem; margin-top: 1rem; width: 70%; border-bottom: 2px solid #4d4c5a;">Ou faça login com seu Usuario</p>

        <form on:submit={login} style="display: flex; flex-direction: column; gap: 0.5rem; color: #fff;">
            <div style="display: flex; flex-direction: column;">
                <label for="Usuario">Usuario</label>
                <input id="Usuario" name="Usuario" type="text" placeholder="Digite seu Usuario..."
                       bind:value={email}
                       style="padding: 0.75rem; width: 20rem; background-color: #45455c; border: none; border-radius: 6px; color: #fff;">
            </div>

            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <label for="password">Senha</label>
                <input id="password" name="password" type="password" placeholder="Digite sua senha..."
                       bind:value={password}
                       style="padding: 0.75rem; background-color: #45455c; border: none; border-radius: 6px; color: #fff;">
            </div>

            <div style="display: flex; flex-direction: column; align-items: center; font-size: 0.9rem; color: #aaa;">
                <label style="display: flex; align-items: center; gap: 0.4rem;">
                <input type="checkbox" style="accent-color: #5c5c7d; ">Mantenhame logado</label>
                <a href="#" style="color: #aaa; text-decoration: none;">Esqueceu a senha ?</a>
            </div>

            <button type="submit"
                style="padding: 0.8rem; background-color: #6c4eff; color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: bold;">
                Login
            </button>
            <a href="/register" style="color: #aaa; text-decoration: none;">Ainda nao possui uma conta ?</a>
            </form>

    </div>
</main>