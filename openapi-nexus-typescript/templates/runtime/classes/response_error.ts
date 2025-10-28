export class ResponseError extends Error {
    response: Response;

    constructor(response: Response, msg?: string) {
        super(msg);
        this.response = response;
    }
}
