<script lang="ts">
    import type {AutocompleteOption, PopupSettings} from "@skeletonlabs/skeleton";
    import {Autocomplete, FileDropzone, popup, storePopup} from "@skeletonlabs/skeleton";
    import {arrow, autoUpdate, computePosition, flip, offset, shift} from '@floating-ui/dom';
    import {writeBinaryFile} from "@tauri-apps/api/fs";
    import {dialog} from "@tauri-apps/api";
    import toast from "svelte-french-toast";
    import {Body, fetch, ResponseType} from "@tauri-apps/api/http";
    
    const baseURL = "https://translation-api.w1nterish3re.repl.co";
    // const baseURL = "http://127.0.0.1:8888";

    let files: FileList;
    let langSelectedLabel: string = '';
    let langSelected: string = '';
    let popupSettings: PopupSettings = {
        event: 'focus-click',
        target: 'popupAutocomplete',
        placement: 'bottom',
    };

    storePopup.set({computePosition, autoUpdate, flip, shift, offset, arrow});

    async function getLanguages() {
        const response = await fetch(baseURL + "/langs");
        return response.data;
    }

    function buildACOptions(langs: any): AutocompleteOption[] {
        return Object.keys(langs).map((lang) => {
            return {
                label: langs[lang],
                value: lang
            }
        });
    }

    async function process() {
        if (!files || files.length <= 0) {
            toast.error("No file selected");
            return;
        }

        if (!langSelected) {
            toast.error("No language selected");
            return;
        }

        const saveToPath = await dialog.save({
            filters: [{
                name: "Book Document (.pdf, .epub)",
                extensions: ["pdf", "epub"]
            }],
            title: "Save to"
        });
        
        if (!saveToPath) {
            toast.error("No path selected");
            return;
        }

        toast.promise(translate(saveToPath), {
            loading: "Translating...",
            success: "Downloaded!",
            error: err => err.toString()
        });
    }
    
    async function translate(saveToPath: string) {
        const file: File = files.item(0)!;
        const data = await file.arrayBuffer();
        const ext = getFileExtension(file.name);
        const contentType = ext == "pdf" ? "application/pdf" : "application/epub+zip";

        const response = await fetch(baseURL + "/translate", {
            responseType: ResponseType.Binary,
            method: "POST",
            headers: {
                "Content-Type": contentType
            },
            query: {
                lang: langSelected,
                type: ext
            },
            body: Body.bytes(data)
        });

        if (!response.ok) {
            toast.error("Request failed");
            return;
        }

        await writeBinaryFile(saveToPath, response.data as ArrayBuffer, {});
    }

    function getFileExtension(filename: string): string {
        return filename.slice(((filename.lastIndexOf(".") - 1) >>> 0) + 2);
    }
</script>

<main class="Uploader flex justify-center items-center flex-col p-32">
    <h2 class="h2 m-8">Translate a file</h2>

    {#await getLanguages()}
        <span>Loading</span>
    {:then languages}
        <input
                class="input autocomplete w-2/5 pt-2 pb-2 pl-4 pr-4"
                type="search"
                name="autocomplete-search"
                bind:value={langSelectedLabel}
                placeholder="Select a language"
                use:popup={popupSettings}
        />
        <div data-popup="popupAutocomplete" class="card max-h-48 p-4 overflow-y-auto z-50 w-1/5" tabindex="-1">
            <Autocomplete bind:input={langSelectedLabel} options={buildACOptions(languages)} on:selection={(evt) => {
                langSelectedLabel = evt.detail.label;
                langSelected = evt.detail.value;
            }}/>
        </div>
    {/await}

    <FileDropzone name="upload" bind:files class="w-2/5 mt-4 mb-4 h-full">
        <svelte:fragment slot="meta">PDF or EPub allowed</svelte:fragment>
        <svelte:fragment slot="message">
            {#if files === undefined || files.length <= 0}
                <b>Upload a file</b> or drag and drop
            {:else}
                Upload <b>{files.item(0).name}</b>
            {/if}
        </svelte:fragment>
    </FileDropzone>
    <button class="btn variant-filled" on:click={process}>Process</button>
</main>

<style>
    .Uploader {
        height: 100vh;
    }
</style>