<script lang="ts">
    let school = "";
    let username = "";
    let password = "";
</script>

<div class="login">
	<div class="form">
        <label for="school">School</label>
		<input id="school" type="text" bind:value={school}/>

        <label for="username">Username</label>
		<input id="username" type="text" bind:value={username}/>

        <label for="password">Password</label>
		<input id="password" type="password" bind:value={password}/>

        <button>Login</button>
	</div>
</div>

<style lang="scss">
    .login {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;

        .form {
            display: flex;
            flex-direction: column;
        }
    }
</style>
