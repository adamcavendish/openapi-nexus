export class JSONApiResponse<T> {
    raw: Response;
    transformer: ResponseTransformer<T>;

    constructor(raw: Response, transformer: ResponseTransformer<T> = (jsonValue: any) => jsonValue) {
        this.raw = raw;
        this.transformer = transformer;
    }

    async value(): Promise<T> {
        return this.transformer(await this.raw.json());
    }
}
