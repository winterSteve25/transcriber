<script lang="ts">
    import Router from "svelte-spa-router";
    import Uploader from "./lib/Uploader.svelte";
    import {Toaster} from "svelte-french-toast";
    
    const routes = {
        "/": Uploader,
    }
</script>

<main class="App">
    <Toaster/>
    <Router {routes}/>
</main>