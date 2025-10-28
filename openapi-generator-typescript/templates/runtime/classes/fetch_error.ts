export class FetchError extends Error {
    cause: Error;

    constructor(cause: Error, msg?: string) {
        super(msg);
        this.cause = cause;
    }
}
