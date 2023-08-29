<script lang="ts">
    import {AppBar, AppShell, LightSwitch} from "@skeletonlabs/skeleton";
    import {LibraryData} from "../../ts/data.ts";
    import Epub from "epubjs";

    export let params: any = {};
    let pagesElement: HTMLDivElement;

    async function setup() {
        let uuid = await params["book"];
        let bookData = await LibraryData.INSTANCE.getBookData(uuid);
        let pkg = await LibraryData.INSTANCE.getEpubPkgOPF(uuid);

        const book = Epub(pkg, {});

        return bookData;
    }
</script>

<main class="Reader">
    {#await setup()}
        <p>Loading book</p>
    {:then bookData}
        <AppShell>
            <svelte:fragment slot="header">
                <AppBar>
                    <svelte:fragment slot="lead"><h2 class="h2">{bookData.name}</h2></svelte:fragment>
                    <svelte:fragment slot="trail">
                        <LightSwitch/>
                    </svelte:fragment>
                </AppBar>
            </svelte:fragment>

            <slot>
                <div id="book-content" class="flex justify-center items-center">
                    <button>Last</button>
                    <div bind:this={pagesElement} id="book-pages"/>
                    <button>Next</button>
                </div>
            </slot>
        </AppShell>
    {/await}
</main>