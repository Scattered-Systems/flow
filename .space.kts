job("(Flow) Docker: Build and publish") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    host("Build artifacts and a Docker image") {
        env["HUB_USER"] = Secrets("dockerhub_username")
        env["HUB_TOKEN"] = Secrets("dockerhub_token")

        shellScript {
            content = """
                docker login --username ${'$'}HUB_USER --password "${'$'}HUB_TOKEN"
            """
        }

        dockerBuildPush {
            context = "."
            file = "./bin/flow/Dockerfile"
            labels["vendor"] = "Scattered-Systems, LLC"
            tags {
                +"scsys/flow:latest"
                +"scsys/flow:v0.1.${"$"}JB_SPACE_EXECUTION_NUMBER"
            }
        }
    }
}

job("(Flow) Rust: Build and test the workspace") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        env["CARGO_REGISTRY_TOKEN"] = Secrets("cargo_registry_token")
        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all-features
            """
        }
    }
}

job("(Flow) Rust: Publish crates") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/tags/v*.*.*"
            }
        }
    }
    container(displayName = "Rust", image = "rust") {
        env["TOKEN"] = Secrets("cargo_registry_token")

        shellScript {
            interpreter = "/bin/bash"
            content = """
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p fluidity-core
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p fluidity-sdk
                cargo publish --all-features --color always --jobs 1 --token ${'$'}TOKEN --verbose -p fluidity
            """
        }
    }
}