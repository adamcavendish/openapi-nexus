/// Error type for TypeScript generation
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum GeneratorError {
    #[snafu(display("Generator error: {}", message))]
    Generic { message: String },
}
