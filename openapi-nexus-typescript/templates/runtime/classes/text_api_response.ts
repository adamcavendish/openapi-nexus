export class TextApiResponse {
    raw: Response;

    constructor(raw: Response) {
        this.raw = raw;
    }

    async value(): Promise<string> {
        return await this.raw.text();
    }
}
