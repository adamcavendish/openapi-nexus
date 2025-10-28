export interface ResponseTransformer<T> {
    (json: any): T;
}
