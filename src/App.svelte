<script lang="ts">
    import Router from "svelte-spa-router";
    import Library from "./lib/v2/Library.svelte";
    import Reader from "./lib/v2/Reader.svelte";
    import Uploader from "./lib/Uploader.svelte";
    import {Toaster} from "svelte-french-toast";
    
    const routes = {
        "/": Uploader,
        "/v2": Library,
        "/v2/read/:book": Reader,
    }
</script>

<main class="App">
    <Toaster/>
    <Router {routes}/>
</main>