import {invoke} from "@tauri-apps/api";

export class BookMetadata {
    public name: string;
    public thumbnail: string;
    public lastRead: Date;
    public author: string;
    public totalPages: number;
    public readProgress: number;
    public uuid: string;

    private constructor(name: string, thumbnail: string, lastRead: Date, author: string, totalPages: number, readProgress: number, uuid: string) {
        this.name = name;
        this.thumbnail = thumbnail;
        this.lastRead = lastRead;
        this.author = author;
        this.totalPages = totalPages;
        this.readProgress = readProgress;
        this.uuid = uuid;
    }
    
    public static async FromEPub(path: string, uuid: string): Promise<BookMetadata> {
        try {
            let data: any = await invoke("get_epub_data", {epub: path});
            return new BookMetadata(data["title"], data["cover_path"], new Date(Date.now()), data["author"], data["number_of_pages"], 0, uuid);
        } catch (err) {
            console.error(err);
        }
        
        return new BookMetadata("", "", new Date(), "", 0, 0, "");
    }
}