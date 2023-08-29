import {BookMetadata} from "./models.ts";
import {copyFile, createDir, exists, readBinaryFile, readTextFile, writeTextFile} from "@tauri-apps/api/fs";
import {appDataDir} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/tauri";

export class LibraryData {
   
    public static INSTANCE: LibraryData = new LibraryData();

    private pathToUUID: any;
    private uuidToBook: any;

    private constructor() {
        this.pathToUUID = null;
        this.uuidToBook = null;
    }

    private async init() {
        let dir = await appDataDir();
        let hasData = await exists(dir + "/library.json");

        if (hasData) {
            let content = await readTextFile(dir + "/library.json");
            this.uuidToBook = JSON.parse(content);

            for (let dat of Object.values(this.uuidToBook)) {
                // @ts-ignore
                dat.lastRead = new Date(dat.lastRead);
            }
        } else {
            this.uuidToBook = {};
        }
        
        hasData = await exists(dir + "/uuids.json");
        if (hasData) {
            let content = await readTextFile(dir + "/uuids.json");
            this.pathToUUID = JSON.parse(content);
        } else {
            this.pathToUUID = {};
        }
    }

    public async getEpubBookData(path: string): Promise<BookMetadata> {
        if (this.uuidToBook === null) {
            await this.init();
        }

        if (!(path in this.pathToUUID)) {
            let uuid: string = await invoke("generate_uuid");
            let dataDir = await appDataDir();
            let booksDirExist = await exists(`${dataDir}books`);
            
            if (!booksDirExist) {
                createDir(`${dataDir}books`);
            }
            
            let newPath = `${dataDir}books/${uuid}.epub`;
            await copyFile(path, newPath);
            await invoke("unzip", {uuid: uuid});
            
            let mdata = await BookMetadata.FromEPub(newPath, uuid);
            this.pathToUUID[path] = uuid;
            this.uuidToBook[uuid] = mdata;

            writeTextFile(dataDir + "/uuids.json", JSON.stringify(this.pathToUUID));
            writeTextFile(dataDir + "/library.json", JSON.stringify(this.uuidToBook));
            
            return mdata;
        }

        return this.uuidToBook[this.pathToUUID[path]]! as BookMetadata;
    }
    
    public async getBookData(uuid: string): Promise<BookMetadata> {
        if (this.uuidToBook === null) {
            await this.init();
        }
        
        return this.uuidToBook[uuid]! as BookMetadata;
    }
    
    public async getBookSourcePath(uuid: string): Promise<string> {
        let dataDir = await appDataDir();
        return convertFileSrc(`${dataDir}books/${uuid}.epub`);
    }
    
    public async getEpubPkgOPF(uuid: string): Promise<ArrayBuffer> {
        let dataDir = await appDataDir();
        let path = `${dataDir}books/${uuid}/EPUB/package.opf`
        return readBinaryFile(path);
    }
}