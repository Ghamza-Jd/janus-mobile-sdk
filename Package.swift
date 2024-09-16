// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = true
let releaseTag = "0.1.0"
let releaseChecksum = "0cd41f6d76d9d6f933871b1328d1689dbfd51fabbcbbc4f1d76164094bef083c"

let binaryTarget: Target

if useLocalFramework {
    binaryTarget = .binaryTarget(
        name: "JanusGatewayFFI",
        path: "./target/ios/libjanus_gateway-rs.xcframework"
    )
} else {
    binaryTarget = .binaryTarget(
        name: "JanusGatewayFFI",
        url: "https://github.com/Proximie/jarust-ios-package/releases/download/\(releaseTag)/libjanus_gateway-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "JanusGateway",
    platforms: [.iOS(.v13)],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "JanusGateway",
            targets: ["JanusGateway"]
        ),
        .library(
            name: "JanusGatewayPlugins",
            targets: ["JanusGatewayPlugins"]
        ),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "JanusGateway",
            dependencies: [.target(name: "UniFFI")],
            path: "apple/Sources/JanusGateway"
        ),
        .target(
            name: "JanusGatewayPlugins",
            dependencies: [
                .target(name: "UniFFI"),
                .target(name: "JanusGateway")
            ],
            path: "apple/Sources/Plugins"
        ),
        .target(
            name: "UniFFI",
            dependencies: [.target(name: "JanusGatewayFFI")],
            path: "apple/Sources/UniFFI"
        ),
        .testTarget(
            name: "JanusGatewayTests",
            dependencies: ["JanusGateway"],
            path: "apple/Tests/JanusGatewayTests"
        ),
    ]
)
