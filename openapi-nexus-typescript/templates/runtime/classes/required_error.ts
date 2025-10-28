export class RequiredError extends Error {
    field: string;

    constructor(field: string) {
        super(`Field ${field} is required`);
        this.field = field;
    }
}
