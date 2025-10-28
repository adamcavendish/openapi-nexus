export class BlobApiResponse {
    raw: Response;

    constructor(raw: Response) {
        this.raw = raw;
    }

    async value(): Promise<Blob> {
        return await this.raw.blob();
    }
}
