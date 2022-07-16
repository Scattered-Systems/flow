job("Flow: Build and Push Docker") {
    docker {
        build {
            args["HTTP_PROXY"] = "http://0.0.0.0:8888"
            context = "."
            customPlatform = "linux/arm"
            file = "Dockerfile"
            labels["vendor"] = "scattered-systems"
        }
        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/flow") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER")
        }
    }
}