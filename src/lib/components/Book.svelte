<script lang="ts">
    import type {BookMetadata} from "../../ts/models.ts";
    import {convertFileSrc} from "@tauri-apps/api/tauri";

    export let data: BookMetadata;
</script>

<a class="Book bg-primary-hover-token p-8 rounded-container-token cursor-pointer" href="#/read/{data.uuid}">
    <header>
        <img src={convertFileSrc(data.thumbnail)} alt="book-thumbnail">
    </header>

    <div class="mt-2">
        <h3 class="h3">{data.name}</h3>
    </div>

    <footer>
        <span class="h6 opacity-80">Written By {data.author}</span>
        <span class="h6 opacity-80">Last Read on {data.lastRead.toLocaleDateString()}</span>
        <span id="progress" class="h6 opacity-80 mt-1">{data.readProgress}/{data.totalPages}</span>
    </footer>
</a>

<style lang="scss">
  .Book {
    max-width: 15vw;
    min-width: 200px;
    transition: background-color 200ms ease-out;
  }
  
  footer {
    display: flex;
    flex-direction: column;
    
    #progress {
      align-self: end;
    }
  }
</style>