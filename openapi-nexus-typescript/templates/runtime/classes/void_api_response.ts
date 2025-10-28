export class VoidApiResponse {
    raw: Response;

    constructor(raw: Response) {
        this.raw = raw;
    }

    async value(): Promise<void> {
        return undefined;
    }
}
