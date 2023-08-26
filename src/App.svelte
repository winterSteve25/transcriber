<script lang="ts">
    import {AppBar, AppShell, FileDropzone, LightSwitch} from "@skeletonlabs/skeleton";
</script>

<main class="App">
    <AppShell>
        <svelte:fragment slot="header">
            <AppBar>
                <svelte:fragment slot="lead"><h2 class="h2">Transcriber</h2></svelte:fragment>
                <svelte:fragment slot="trail"><LightSwitch/></svelte:fragment>
            </AppBar>
        </svelte:fragment>
        
        <slot>
            
        </slot>
    </AppShell>
</main>

<style lang="scss">
    
</style>