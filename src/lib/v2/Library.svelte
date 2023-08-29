<script>
    import {AppBar, AppShell, LightSwitch} from "@skeletonlabs/skeleton";
    import Book from "../components/Book.svelte";
    import {LibraryData} from "../../ts/data.ts";
</script>

<AppShell>
    <svelte:fragment slot="header">
        <AppBar>
            <svelte:fragment slot="lead"><h2 class="h2">Library</h2></svelte:fragment>
            <svelte:fragment slot="trail">
                <LightSwitch/>
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

    <slot>
        <section class="books grid grid-cols-3 md:grid-cols-6 gap-y-8 m-8">
            {#await LibraryData.INSTANCE.getEpubBookData("/home/cadenz/Downloads/accessible_epub_3.epub")}
                Loading the library..
            {:then data}
                <Book data={data}/>
            {/await}
        </section>
    </slot>
</AppShell>