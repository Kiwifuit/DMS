pub mod docker;
pub mod shell;

pub use docker::build_docker_image;
pub use shell::ServerSoftware;

use std::process::Command;

const DOCKERFILE: &str = include_str!("../res/Dockerfile");

pub fn test_docker() -> bool {
    Command::new("docker").spawn().is_ok()
}

fn generate_dockerfile<V: ToString>(java_version: V) -> String {
    DOCKERFILE.replace(
        char::REPLACEMENT_CHARACTER,
        java_version.to_string().as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dockerfile() {
        assert_eq!(
            DOCKERFILE,
            r#"FROM eclipse-temurin:�-jre-alpine

WORKDIR /srv/minecraft
COPY dockerfs .

EXPOSE 25565 25575
ENTRYPOINT [ "./run.sh" ]"#
        )
    }

    #[test]
    fn test_dockergen() {
        assert_eq!(
            generate_dockerfile(17),
            r#"FROM eclipse-temurin:17-jre-alpine

WORKDIR /srv/minecraft
COPY dockerfs .

EXPOSE 25565 25575
ENTRYPOINT [ "./run.sh" ]"#
        );
    }
}
