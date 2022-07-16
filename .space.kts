job("Flow: Build and Push Docker") {
    startOn {
        gitPush { enabled = false }
    }
    docker {
        // get auth data from secrets and put it to env vars
        env["DOCKERHUB_USER"] = Secrets("dockerhub_user")
        env["DOCKERHUB_TOKEN"] = Secrets("dockerhub_token")

        // put auth data to Docker config
        beforeBuildScript {
            content = """
                B64_AUTH=${'$'}(echo -n ${'$'}DOCKERHUB_USER:${'$'}DOCKERHUB_TOKEN | base64 -w 0)
                echo "{\"auths\":{\"https://index.docker.io/v1/\":{\"auth\":\"${'$'}B64_AUTH\"}}}" > ${'$'}DOCKER_CONFIG/config.json
            """
        }
        build {
            context = "."
            customPlatform = "linux/arm"
            file = "Dockerfile"
            labels["vendor"] = "scattered-systems"
        }
        push("jo3mccain/flow") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}